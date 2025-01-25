use std::{collections::HashMap, sync::Arc};

use futures::future::{BoxFuture, FutureExt};
use futures::stream::{BoxStream, StreamExt, TryStreamExt};
use tokio::sync::{broadcast, RwLock};

use crate::traq::TraqHost;

impl Default for super::TraqBotChannels {
    fn default() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<Context> super::TraqBotService<Context> for super::TraqBotServiceImpl
where
    Context: AsRef<reqwest::Client>
        + AsRef<super::TraqBotChannels>
        + AsRef<TraqHost>
        + AsRef<super::TraqBotConfig>
        + Send
        + Sync
        + 'static,
{
    type Error = super::Error;

    fn build_request_as_bot<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::BuildRequestAsBotParams<'a>,
    ) -> BoxFuture<'a, Result<reqwest::RequestBuilder, Self::Error>> {
        build_request_as_bot(ctx.as_ref(), ctx.as_ref(), params)
            .map(Result::<_, super::Error>::Ok)
            .boxed()
    }
    fn subscribe_channel<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SubscribeChannelParams,
    ) -> BoxFuture<
        'a,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<crate::traq::message::TraqMessage, Self::Error>,
            >,
            Self::Error,
        >,
    > {
        subscribe_channel(ctx, params).boxed()
    }
    fn leave_channel<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::LeaveChannelParams,
    ) -> BoxFuture<'a, Result<(), Self::Error>> {
        leave_channel(ctx, params).boxed()
    }
    fn on_left_channel<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::OnLeftChannelParams,
    ) -> BoxFuture<'a, Result<(), Self::Error>> {
        on_left_channel(ctx, params).boxed()
    }
    fn on_message_created<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::OnMessageCreatedParams,
    ) -> BoxFuture<'a, Result<(), Self::Error>> {
        on_message_created(ctx, params).boxed()
    }
}

async fn build_request_as_bot(
    client: &reqwest::Client,
    bot_config: &super::TraqBotConfig,
    params: super::BuildRequestAsBotParams<'_>,
) -> reqwest::RequestBuilder {
    let super::BuildRequestAsBotParams { method, uri } = params;
    client
        .request(method, uri)
        .bearer_auth(&bot_config.access_token)
}

#[tracing::instrument(skip_all)]
async fn subscribe_channel<C>(
    ctx: &C,
    params: super::SubscribeChannelParams,
) -> Result<BoxStream<'static, Result<crate::traq::message::TraqMessage, super::Error>>, super::Error>
where
    C: AsRef<reqwest::Client>
        + AsRef<super::TraqBotChannels>
        + AsRef<TraqHost>
        + AsRef<super::TraqBotConfig>
        + Send
        + Sync
        + 'static,
{
    let super::SubscribeChannelParams { id: channel_id } = params;
    let traq_host: &TraqHost = ctx.as_ref();
    let bot_channels: &super::TraqBotChannels = ctx.as_ref();
    let bot_config: &super::TraqBotConfig = ctx.as_ref();
    let uri = format!(
        "https://{traq_host}/api/v3/bots/{id}/actions/join",
        id = &bot_config.bot_id
    );
    let req_params = super::BuildRequestAsBotParams {
        method: http::Method::POST,
        uri: &uri,
    };
    let _ = build_request_as_bot(ctx.as_ref(), ctx.as_ref(), req_params)
        .await
        .json(&serde_json::json!({
            "channelId": channel_id,
        }))
        .send()
        .await?
        .error_for_status()?;
    let mut channels = bot_channels.channels.write().await;
    let tx = channels
        .entry(channel_id)
        .or_insert_with(|| broadcast::Sender::new(10));
    let rx = tokio_stream::wrappers::BroadcastStream::new(tx.subscribe());
    let stream = rx.map_err(super::Error::BroadcastRx).boxed();
    Ok(stream)
}

#[tracing::instrument(skip_all)]
async fn leave_channel<C>(ctx: &C, params: super::LeaveChannelParams) -> Result<(), super::Error>
where
    C: AsRef<reqwest::Client>
        + AsRef<TraqHost>
        + AsRef<super::TraqBotConfig>
        + Send
        + Sync
        + 'static,
{
    let super::LeaveChannelParams { id: channel_id } = params;
    let traq_host: &TraqHost = ctx.as_ref();
    let bot_config: &super::TraqBotConfig = ctx.as_ref();
    let uri = format!(
        "https://{traq_host}/api/v3/bots/{id}/actions/leave",
        id = &bot_config.bot_id
    );
    let req_params = super::BuildRequestAsBotParams {
        method: http::Method::POST,
        uri: &uri,
    };
    let _ = build_request_as_bot(ctx.as_ref(), ctx.as_ref(), req_params)
        .await
        .json(&serde_json::json!({
            "channelId": channel_id,
        }))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

async fn on_left_channel<C>(ctx: &C, params: super::OnLeftChannelParams) -> Result<(), super::Error>
where
    C: AsRef<super::TraqBotChannels> + Send + Sync + 'static,
{
    let super::OnLeftChannelParams { channel } = params;
    let bot_channels: &super::TraqBotChannels = ctx.as_ref();
    let mut channels = bot_channels.channels.write().await;
    let _ = channels.remove(&channel.id);
    tracing::debug!(id = %channel.id.0, "Removed channel");
    Ok(())
}

async fn on_message_created<C>(
    ctx: &C,
    params: super::OnMessageCreatedParams,
) -> Result<(), super::Error>
where
    C: AsRef<reqwest::Client>
        + AsRef<super::TraqBotChannels>
        + AsRef<TraqHost>
        + AsRef<super::TraqBotConfig>
        + Send
        + Sync
        + 'static,
{
    let super::OnMessageCreatedParams { message } = params;
    let channels: &super::TraqBotChannels = ctx.as_ref();
    let channels = channels.channels.read().await;
    let Some(tx) = channels.get(&message.channel_id) else {
        return Ok(());
    };
    tx.send(message)?;
    Ok(())
}
