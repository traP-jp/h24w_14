use std::sync::Arc;

use anyhow::Context;
use futures::stream::BoxStream;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub(super) async fn init<Context>(
    ctx: Arc<Context>,
    speaker_phones: impl IntoIterator<Item = super::SpeakerPhone>,
) -> Result<(), super::Error>
where
    Context: AsRef<crate::task::TaskManager>
        + crate::event::ProvideEventService
        + crate::traq::bot::ProvideTraqBotService
        + crate::traq::user::ProvideTraqUserService
        + crate::traq::message::ProvideTraqMessageService
        + crate::traq::channel::ProvideTraqChannelService,
{
    use futures::stream::{pending, StreamExt};

    let task_manager: &crate::task::TaskManager = (*ctx).as_ref();
    let speaker_phone_stream = ctx
        .subscribe_speaker_phones()
        .map(|r| r.context("Failed to receive new speaker phone"));
    let message_stream = ctx
        .subscribe_messages()
        .map(|r| r.context("Failed to receive new message"));
    let traq_message_stream = futures::stream::select_all([pending().boxed()]);
    let mut state = State {
        ctx: Arc::clone(&ctx),
        speaker_phones: Default::default(),
        speaker_phone_stream,
        message_stream,
        traq_message_stream,
    };
    state.load_speaker_phones(speaker_phones).await?;
    task_manager.spawn(move |c| state.spawn(c)).await;
    Ok(())
}

struct State<Context, SP, M> {
    ctx: Arc<Context>,
    speaker_phones: Arc<RwLock<Vec<super::SpeakerPhone>>>,
    speaker_phone_stream: SP,
    message_stream: M,
    traq_message_stream: futures::stream::SelectAll<
        BoxStream<
            'static,
            anyhow::Result<(crate::traq::message::TraqMessage, super::SpeakerPhone)>,
        >,
    >,
}

impl<Context, SP, M> State<Context, SP, M>
where
    Context: crate::traq::bot::ProvideTraqBotService
        + crate::traq::user::ProvideTraqUserService
        + crate::traq::message::ProvideTraqMessageService
        + crate::traq::channel::ProvideTraqChannelService,
    SP: futures::Stream<Item = anyhow::Result<super::SpeakerPhone>> + Send + Unpin + 'static,
    M: futures::Stream<Item = anyhow::Result<crate::message::Message>> + Send + Unpin + 'static,
{
    async fn load_speaker_phones(
        &mut self,
        speaker_phones: impl IntoIterator<Item = super::SpeakerPhone>,
    ) -> Result<(), super::Error> {
        for speaker_phone in speaker_phones {
            self.handle_speaker_phone(speaker_phone)
                .await
                .inspect_err(|e| tracing::error!("{e:?}"))
                .map_err(|e| tonic::Status::from_error(e.into()))?;
        }
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn spawn(mut self, cancel: CancellationToken) {
        enum Next {
            SpeakerPhone(super::SpeakerPhone),
            Message(crate::message::Message),
            TraqMessage((crate::traq::message::TraqMessage, super::SpeakerPhone)),
        }

        use futures::StreamExt;

        loop {
            let next = tokio::select! {
                _ = cancel.cancelled() => {
                    tracing::debug!("Received cancel order");
                    break;
                },
                Some(speaker_phone) = self.speaker_phone_stream.next() => {
                    speaker_phone
                        .map(Next::SpeakerPhone)
                        .context("Failed to receive new speaker phone")
                },
                Some(message) = self.message_stream.next() => {
                    message
                        .map(Next::Message)
                        .context("Failed to receive new message")
                },
                Some(traq_message) = self.traq_message_stream.next() => {
                    traq_message
                        .map(Next::TraqMessage)
                        .context("Failed to receive new traq message")
                },
                else => {
                    tracing::error!("Never ending stream exhausted");
                    continue;
                },
            };
            let res = match next {
                Ok(Next::SpeakerPhone(speaker_phone)) => {
                    tracing::debug!("Received new speaker phone");
                    self.handle_speaker_phone(speaker_phone).await
                }
                Ok(Next::Message(message)) => {
                    tracing::debug!("Received new message");
                    self.handle_message(message).await
                }
                Ok(Next::TraqMessage((traq_message, speaker_phone))) => {
                    tracing::debug!("Received new traQ message");
                    self.handle_traq_message(traq_message, speaker_phone).await
                }
                Err(e) => Err(e),
            };
            match res {
                Ok(()) => tracing::debug!("Handled successfully"),
                Err(e) => tracing::error!("Handle error: {e:?}"),
            };
        }
    }

    #[tracing::instrument(skip_all, fields(id = ?speaker_phone.id))]
    async fn handle_speaker_phone(
        &mut self,
        speaker_phone: super::SpeakerPhone,
    ) -> anyhow::Result<()> {
        use futures::{StreamExt, TryStreamExt};

        let traq_channel = find_traq_channel(&*self.ctx, &speaker_phone.name.0).await?;
        let new_traq_message_stream = self
            .ctx
            .subscribe_channel(crate::traq::bot::SubscribeChannelParams {
                id: traq_channel.id,
            })
            .await
            .context("Failed to subscribe traQ channel")?;
        let new_traq_message_stream = {
            let speaker_phone = speaker_phone.clone();
            new_traq_message_stream
                .map_ok(move |m| (m, speaker_phone.clone()))
                .map_err(anyhow::Error::new)
                .boxed()
        };
        self.traq_message_stream.push(new_traq_message_stream);

        self.speaker_phones.write().await.push(speaker_phone);
        tracing::info!("Done");
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(id = ?message.id))]
    async fn handle_message(&mut self, message: crate::message::Message) -> anyhow::Result<()> {
        let synced_message = self
            .ctx
            .check_message_sent(crate::traq::message::CheckMessageSentParams {
                message: message.clone(),
            })
            .await
            .context("Failed to check traQ message")?;
        if let Some(synced_message) = synced_message {
            // FIXME: 1メッセージと複数のspeaker phoneが対応してしまうとバグる
            tracing::info!(traq_message_id = ?synced_message.id, "Message already synced");
            return Ok(());
        }

        let traq_user = self
            .ctx
            .find_traq_user_by_app_user_id(crate::traq::user::FindTraqUserByAppUserIdParams {
                id: message.user_id,
            })
            .await
            .context("Failed to find traQ user counterpart")?
            .context("No traQ user counterpart found")?;

        let speaker_phones: Vec<_> = {
            // Read lockを早めに解放するためネスト
            let speaker_phones = self.speaker_phones.read().await;
            let speaker_phones = speaker_phones.iter().filter_map(|sp| {
                let distance_x = u32::abs_diff(sp.position.x, message.position.x).pow(2);
                let distance_y = u32::abs_diff(sp.position.y, message.position.y).pow(2);
                let distance = u32::saturating_add(distance_x, distance_y);
                if distance < sp.receive_range.pow(2) {
                    Some(sp.clone())
                } else {
                    None
                }
            });
            speaker_phones.collect()
        };
        let channel_relation = speaker_phones.into_iter().map(|speaker_phone| async {
            let traq_channel = find_traq_channel(&*self.ctx, &speaker_phone.name.0).await?;
            Ok::<_, anyhow::Error>((speaker_phone, traq_channel))
        });
        let channel_relation = futures::future::try_join_all(channel_relation).await?;
        // 複数回traQに送信しないための `.first`
        let Some((speaker_phone, traq_channel)) = channel_relation.first() else {
            tracing::info!("Found no matching speaker phone");
            return Ok(());
        };

        let synced_message = self
            .ctx
            .send_message(crate::traq::message::SendMessageParams {
                inner: message,
                channel_id: traq_channel.id,
                user_id: traq_user.id,
            })
            .await
            .context("Failed to send message to traQ via speaker phone")?;
        tracing::info!(
            speaker_phone_id = ?speaker_phone.id,
            synced_message_id = ?synced_message.id,
            "Sent message to traQ via speaker phone"
        );
        Ok(())
    }

    #[tracing::instrument(
        skip_all,
        fields(traq_message_id = ?traq_message.id, speaker_phone_id = ?speaker_phone.id),
    )]
    async fn handle_traq_message(
        &mut self,
        traq_message: crate::traq::message::TraqMessage,
        speaker_phone: super::SpeakerPhone,
    ) -> anyhow::Result<()> {
        let synced_message = self
            .ctx
            .check_message_received(crate::traq::message::CheckMessageReceivedParams {
                traq_message: traq_message.clone(),
            })
            .await
            .context("Failed to check message synced")?;
        if let Some(synced_message) = synced_message {
            tracing::info!(message_id = ?synced_message.inner.id, "Message already synced");
            return Ok(());
        }

        let user = self
            .ctx
            .find_traq_user(crate::traq::user::FindTraqUserParams {
                id: traq_message.user_id,
            })
            .await
            .context("Failed to find service user by traQ user ID")?;
        let user = match user {
            Some(u) => u,
            None => self
                .ctx
                .register_traq_user(crate::traq::user::RegisterTraqUserParams {
                    id: traq_message.user_id,
                })
                .await
                .context("Failed to register traQ user to service")
                .inspect(|u| {
                    tracing::info!(
                        user_id = ?u.inner.id,
                        traq_user_id = ?u.id,
                        "Registered traQ user to service"
                    )
                })?,
        };
        let user = user.inner;

        let receive_params = crate::traq::message::RecvMessageParams {
            traq_message,
            user_id: user.id,
            position: speaker_phone.position,
        };
        let message = self
            .ctx
            .recv_message(receive_params)
            .await
            .context("Failed to receive traQ message")?;

        tracing::info!(
            message_id = ?message.id,
            user_id = ?user.id,
            "Synced traQ message to service"
        );
        Ok(())
    }
}

// FIXME: これTraqChannelServiceに欲しい
async fn find_traq_channel<Context>(
    ctx: &Context,
    path: &str,
) -> anyhow::Result<crate::traq::channel::TraqChannel>
where
    Context: crate::traq::channel::ProvideTraqChannelService,
{
    let all_channels = ctx
        .get_all_channels(crate::traq::channel::GetAllChannelsParams {})
        .await
        .context("Failed to get traQ channels")?;
    let channel = all_channels
        .into_iter()
        .find(|c| c.path == path)
        .with_context(|| format!("No matching traQ channel found path = {path}"))?;
    Ok(channel)
}
