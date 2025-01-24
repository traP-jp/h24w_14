use futures::{future::BoxFuture, stream::BoxStream};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub mod config;

#[derive(Debug, Clone)]
pub struct BuildRequestAsBotParams<'a> {
    pub method: http::Method,
    pub uri: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SubscribeChannelParams {
    pub id: super::channel::TraqChannelId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LeaveChannelParams {
    pub id: super::channel::TraqChannelId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OnLeftChannelParams {
    pub channel: super::channel::TraqChannel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OnMessageCreatedParams {
    pub message: super::message::TraqMessage,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraqBotConfig {
    bot_id: String,
    bot_user_id: String,
    verification_token: String,
    access_token: String,
}

pub trait TraqBotService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn build_request_as_bot<'a>(
        &'a self,
        ctx: &'a Context,
        params: BuildRequestAsBotParams<'a>,
    ) -> BoxFuture<'a, Result<RequestBuilder, Self::Error>>;
    fn subscribe_channel<'a>(
        &'a self,
        ctx: &'a Context,
        params: SubscribeChannelParams,
    ) -> BoxFuture<'a, Result<BoxStream<'static, super::message::TraqMessage>, Self::Error>>;
    fn leave_channel<'a>(
        &'a self,
        ctx: &'a Context,
        params: LeaveChannelParams,
    ) -> BoxFuture<'a, Result<(), Self::Error>>;
    fn on_left_channel<'a>(
        &'a self,
        ctx: &'a Context,
        params: OnLeftChannelParams,
    ) -> BoxFuture<'a, Result<(), Self::Error>>;
    fn on_message_created<'a>(
        &'a self,
        ctx: &'a Context,
        params: OnMessageCreatedParams,
    ) -> BoxFuture<'a, Result<(), Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideTraqBotService: Send + Sync + 'static {
    type Context;
    type TraqBotService: TraqBotService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn traq_bot_service(&self) -> &Self::TraqBotService;

    fn build_request_as_bot<'a>(
        &'a self,
        params: BuildRequestAsBotParams<'a>,
    ) -> BoxFuture<
        'a,
        Result<RequestBuilder, <Self::TraqBotService as TraqBotService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_bot_service().build_request_as_bot(ctx, params)
    }
    fn subscribe_channel(
        &self,
        params: SubscribeChannelParams,
    ) -> BoxFuture<
        '_,
        Result<
            BoxStream<'static, super::message::TraqMessage>,
            <Self::TraqBotService as TraqBotService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_bot_service().subscribe_channel(ctx, params)
    }
    fn leave_channel(
        &self,
        params: LeaveChannelParams,
    ) -> BoxFuture<'_, Result<(), <Self::TraqBotService as TraqBotService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.traq_bot_service().leave_channel(ctx, params)
    }
    fn on_message_created(
        &self,
        params: OnMessageCreatedParams,
    ) -> BoxFuture<'_, Result<(), <Self::TraqBotService as TraqBotService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.traq_bot_service().on_message_created(ctx, params)
    }
}
