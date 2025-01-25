use std::sync::Arc;

use tower::service_fn;
use tower::util::BoxCloneSyncService;
use traq_bot_http::payloads;

pub fn build_server<State, B>(
    state: Arc<State>,
) -> BoxCloneSyncService<http::Request<B>, http::Response<String>, traq_bot_http::Error>
where
    State: super::ProvideTraqBotService + AsRef<super::TraqBotConfig>,
    B: http_body::Body + Send + 'static,
    B::Data: Send + 'static,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    let config: &super::TraqBotConfig = (*state).as_ref();
    let parser = traq_bot_http::RequestParser::new(&config.verification_token);
    let handler = parser
        .into_handler()
        .on_left(service_fn(
            |(s, left): (Arc<State>, payloads::LeftPayload)| async move {
                let channel = crate::traq::channel::TraqChannel {
                    id: crate::traq::channel::TraqChannelId(left.channel.id),
                    path: left.channel.path,
                };
                (*s).on_left_channel(super::OnLeftChannelParams { channel })
                    .await
            },
        ))
        .on_message_created(service_fn(
            |(s, c): (Arc<State>, payloads::MessageCreatedPayload)| async move {
                let message = crate::traq::message::TraqMessage {
                    id: crate::traq::message::TraqMessageId(c.message.id),
                    channel_id: crate::traq::channel::TraqChannelId(c.message.channel_id),
                    user_id: crate::traq::user::TraqUserId(c.message.user.id),
                    content: c.message.plain_text,
                };
                (*s).on_message_created(super::OnMessageCreatedParams { message })
                    .await
            },
        ))
        .with_state(state);
    BoxCloneSyncService::new(handler)
}
