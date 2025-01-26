use std::{collections::HashMap, sync::Arc};

use futures::{FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

use crate::prelude::IntoStatus;

const RECEIVE_RANGE: u32 = 100;

impl<Context> super::SpeakerPhoneService<Context> for super::SpeakerPhoneServiceImpl
where
    Context: AsRef<MySqlPool>
        + AsRef<crate::task::TaskManager>
        + crate::event::ProvideEventService
        + crate::traq::channel::ProvideTraqChannelService
        + crate::traq::message::ProvideTraqMessageService
        + crate::traq::user::ProvideTraqUserService,
{
    type Error = super::Error;

    fn get_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetSpeakerPhoneParams,
    ) -> futures::future::BoxFuture<'a, Result<super::SpeakerPhone, Self::Error>> {
        get_speaker_phone(ctx.as_ref(), params).boxed()
    }

    fn get_speaker_phones_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetSpeakerPhonesInAreaParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::SpeakerPhone>, Self::Error>> {
        get_speaker_phones_in_area(ctx.as_ref(), params).boxed()
    }

    fn create_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CreateSpeakerPhoneParams,
    ) -> futures::future::BoxFuture<'a, Result<super::SpeakerPhone, Self::Error>> {
        create_speaker_phone(ctx, ctx, ctx.as_ref(), params).boxed()
    }

    fn load_all_speaker_phones(
        &self,
        ctx: Arc<Context>,
        params: super::LoadAllSpeakerPhonesParams,
    ) -> futures::future::BoxFuture<'_, Result<(), Self::Error>> {
        load_all_speaker_phones(ctx, params).boxed()
    }

    fn get_available_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetAvailableChannelsParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Channel>, Self::Error>> {
        get_available_channels(ctx, params).boxed()
    }

    fn search_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SearchChannelsParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Channel>, Self::Error>> {
        search_channels(ctx, params).boxed()
    }
}

// MARK: DB operations

#[derive(Debug, Clone, Hash, Deserialize, Serialize, FromRow)]
struct SpeakerPhoneRow {
    pub id: uuid::Uuid,
    pub position_x: u32,
    pub position_y: u32,
    pub receive_range: u32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<SpeakerPhoneRow> for super::SpeakerPhone {
    fn from(value: SpeakerPhoneRow) -> Self {
        Self {
            id: super::SpeakerPhoneId(value.id),
            position: crate::world::Coordinate {
                x: value.position_x,
                y: value.position_y,
            },
            receive_range: value.receive_range,
            name: super::Channel(value.name),
            created_at: super::Timestamp(value.created_at),
            updated_at: super::Timestamp(value.updated_at),
        }
    }
}

async fn get_speaker_phone(
    pool: &MySqlPool,
    params: super::GetSpeakerPhoneParams,
) -> Result<super::SpeakerPhone, super::Error> {
    let super::GetSpeakerPhoneParams {
        id: super::SpeakerPhoneId(id),
    } = params;
    let speaker_phone: Option<SpeakerPhoneRow> =
        sqlx::query_as(r#"SELECT * FROM `speaker_phones` WHERE `id` = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
    speaker_phone
        .map(|row| row.into())
        .ok_or(super::Error::NotFound)
}

async fn get_speaker_phones_in_area(
    pool: &MySqlPool,
    params: super::GetSpeakerPhonesInAreaParams,
) -> Result<Vec<super::SpeakerPhone>, super::Error> {
    let super::GetSpeakerPhonesInAreaParams { center, size } = params;
    // TODO: SpeakerPhoneの中央はAreaにないが範囲が被っているやつも含めたいね
    let speaker_phones: Vec<SpeakerPhoneRow> = sqlx::query_as(
        r#"
            SELECT * FROM `speaker_phones`
            WHERE
                `position_x` BETWEEN ? AND ?
                AND `position_y` BETWEEN ? AND ?
        "#,
    )
    .bind(center.x.saturating_sub(size.width / 2))
    .bind(center.x.saturating_add(size.width / 2))
    .bind(center.y.saturating_sub(size.height / 2))
    .bind(center.y.saturating_add(size.height / 2))
    .fetch_all(pool)
    .await?;
    Ok(speaker_phones.into_iter().map(Into::into).collect())
}

async fn create_speaker_phone(
    event_service: &impl crate::event::ProvideEventService,
    traq_channel_service: &impl crate::traq::channel::ProvideTraqChannelService,
    pool: &MySqlPool,
    params: super::CreateSpeakerPhoneParams,
) -> Result<super::SpeakerPhone, super::Error> {
    let super::CreateSpeakerPhoneParams { position, name } = params;
    let all_channels = traq_channel_service
        .get_all_channels(crate::traq::channel::GetAllChannelsParams {})
        .await
        .map_err(IntoStatus::into_status)?;
    let Some(_traq_channel) = all_channels.iter().find(|ch| ch.path == name) else {
        return Err(super::Error::BadChannelProvided);
    };
    let speaker_phone = SpeakerPhoneRow {
        id: uuid::Uuid::now_v7(),
        position_x: position.x,
        position_y: position.y,
        receive_range: RECEIVE_RANGE,
        name,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    sqlx::query(
        r#"
            INSERT INTO `speaker_phones` (`id`, `position_x`, `position_y`, `receive_range`, `name`, `created_at`, `updated_at`)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(speaker_phone.id)
    .bind(speaker_phone.position_x)
    .bind(speaker_phone.position_y)
    .bind(speaker_phone.receive_range)
    .bind(speaker_phone.name)
    .bind(speaker_phone.created_at)
    .bind(speaker_phone.updated_at)
    .execute(pool)
    .await?;
    tracing::info!(id = %speaker_phone.id, "Created a speaker phone");
    let speaker_phone = get_speaker_phone(
        pool,
        super::GetSpeakerPhoneParams {
            id: super::SpeakerPhoneId(speaker_phone.id),
        },
    )
    .await?;

    event_service
        .publish_event(crate::event::Event::SpeakerPhone(speaker_phone.clone()))
        .await
        .map_err(crate::prelude::IntoStatus::into_status)?;

    Ok(speaker_phone)
}

async fn load_all_speaker_phones<Context>(
    ctx: Arc<Context>,
    _params: super::LoadAllSpeakerPhonesParams,
) -> Result<(), super::Error>
where
    Context: AsRef<MySqlPool>
        + AsRef<crate::task::TaskManager>
        + crate::traq::channel::ProvideTraqChannelService
        + crate::traq::message::ProvideTraqMessageService
        + crate::traq::user::ProvideTraqUserService
        + crate::event::ProvideEventService,
{
    let pool: &MySqlPool = (*ctx).as_ref();

    let speaker_phones: Vec<super::SpeakerPhone> =
        sqlx::query_as::<_, SpeakerPhoneRow>(r#"SELECT * FROM `speaker_phones`"#)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<super::SpeakerPhone>>();

    let channels = ctx
        .get_all_channels(crate::traq::channel::GetAllChannelsParams {})
        .await
        .map_err(IntoStatus::into_status)?;

    let channel_map: std::collections::HashMap<
        super::SpeakerPhoneId,
        crate::traq::channel::TraqChannel,
    > = speaker_phones
        .iter()
        .filter_map(|speaker_phone| {
            channels
                .iter()
                .find(|channel| channel.path == speaker_phone.name.0)
                .map(|channel| (speaker_phone.id, channel.clone()))
        })
        .collect();

    let ctx_clone = ctx.clone();
    let task_manager: &crate::task::TaskManager = (*ctx_clone).as_ref();
    task_manager
        .spawn(|_cancellation_token| async move {
            let traq_user_service = &*ctx;
            let traq_message_service = &*ctx;
            let event_service = &*ctx;
            run_subscription_loop(
                traq_user_service,
                traq_message_service,
                event_service,
                speaker_phones,
                channel_map,
            )
            .await;
        })
        .await;

    Ok(())
}

async fn run_subscription_loop(
    traq_user_service: &impl crate::traq::user::ProvideTraqUserService,
    traq_message_service: &impl crate::traq::message::ProvideTraqMessageService,
    event_service: &impl crate::event::ProvideEventService,
    mut speaker_phones: Vec<super::SpeakerPhone>,
    channel_map: HashMap<super::SpeakerPhoneId, crate::traq::channel::TraqChannel>,
) {
    let mut speaker_phone_rx = event_service
        .subscribe_speaker_phones()
        .map_err(|e| super::Error::from(e.into_status()));
    let mut message_rx = event_service
        .subscribe_messages()
        .map_err(|e| super::Error::from(e.into_status()));

    loop {
        let channel_map = channel_map.clone();
        tokio::select! {
            speaker_phone = speaker_phone_rx.try_next() => {
                let speaker_phone = match speaker_phone {
                    Ok(Some(speaker_phone)) => speaker_phone,
                    Ok(None) => break,
                    Err(err) => {
                        tracing::error!(error = %err, "Failed to receive a speaker phone");
                        continue;
                    }
                };

                speaker_phones.push(speaker_phone);
            }

            message = message_rx.try_next() => {
                let message = match message {
                    Ok(Some(msg)) => msg,
                    Ok(None) => break,
                    Err(err) => {
                        tracing::error!(error = %err, "Failed to receive a message");
                        continue;
                    }
                };

                for speaker_phone in &speaker_phones {
                    post_message_to_traq(
                        traq_user_service,
                        traq_message_service,
                        speaker_phone,
                        &channel_map,
                        &message,
                    ).await;
                }
            }
        }
    }
}

async fn post_message_to_traq(
    traq_user_service: &impl crate::traq::user::ProvideTraqUserService,
    traq_message_service: &impl crate::traq::message::ProvideTraqMessageService,
    speaker_phone: &super::SpeakerPhone,
    channel_map: &HashMap<super::SpeakerPhoneId, crate::traq::channel::TraqChannel>,
    message: &crate::message::Message,
) {
    if !message
        .position
        .is_inside_circle(speaker_phone.position, speaker_phone.receive_range)
    {
        return;
    }

    let channel = channel_map
        .get(&speaker_phone.id)
        .expect("SpeakerPhoneのチャンネルが存在する");

    let traq_user = traq_user_service
        .find_traq_user_by_app_user_id(crate::traq::user::FindTraqUserByAppUserIdParams {
            id: message.user_id,
        })
        .await
        .map_err(IntoStatus::into_status);
    let traq_user = match traq_user {
        Ok(Some(u)) => u,
        Ok(None) => {
            tracing::error!("User not found");
            return;
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to find a user");
            return;
        }
    };

    let res = traq_message_service
        .send_message(crate::traq::message::SendMessageParams {
            inner: message.clone(),
            channel_id: channel.id,
            user_id: traq_user.id,
        })
        .await
        .map_err(IntoStatus::into_status);
    match res {
        Ok(_) => tracing::info!("Sent a message"),
        Err(err) => tracing::error!(error = %err, "Failed to send a message"),
    }
}

async fn get_available_channels(
    traq_channel_service: &impl crate::traq::channel::ProvideTraqChannelService,
    _params: super::GetAvailableChannelsParams,
) -> Result<Vec<super::Channel>, super::Error> {
    let params = crate::traq::channel::GetAllChannelsParams {};
    let channels = traq_channel_service
        .get_all_channels(params)
        .await
        .map_err(IntoStatus::into_status)?
        .into_iter()
        .map(|channel| super::Channel(channel.path))
        .collect();
    Ok(channels)
}

async fn search_channels(
    traq_channel_service: &impl crate::traq::channel::ProvideTraqChannelService,
    params: super::SearchChannelsParams,
) -> Result<Vec<super::Channel>, super::Error> {
    let super::SearchChannelsParams { name } = params;
    let params = crate::traq::channel::GetAllChannelsParams {};
    let channels = traq_channel_service
        .get_all_channels(params)
        .await
        .map_err(IntoStatus::into_status)?
        .into_iter()
        .map(|channel| super::Channel(channel.path));
    let hits = channels
        .into_iter()
        .filter(|channel| channel.0.contains(&name))
        .collect();
    Ok(hits)
}
