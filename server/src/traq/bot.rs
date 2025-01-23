use futures::{future::BoxFuture, stream::BoxStream};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

#[derive(Debug, Clone)]
pub struct BuildRequestAsBot<'a> {
    pub method: http::Method,
    pub uri: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SubscribeChannel {
    pub id: super::channel::TraqChannelId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LeaveChannel {
    pub id: super::channel::TraqChannelId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OnLeftChannel {
    pub channel: super::channel::TraqChannel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OnMessageCreated {
    pub message: super::message::TraqMessage,
}

pub trait TraqBotService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn build_request_as_bot<'a>(
        &'a self,
        ctx: &'a Context,
        req: BuildRequestAsBot<'a>,
    ) -> BoxFuture<'a, Result<RequestBuilder, Self::Error>>;
    fn subscribe_channel<'a>(
        &'a self,
        ctx: &'a Context,
        req: SubscribeChannel,
    ) -> BoxFuture<'a, Result<BoxStream<'static, super::message::TraqMessage>, Self::Error>>;
    fn leave_channel<'a>(
        &'a self,
        ctx: &'a Context,
        req: LeaveChannel,
    ) -> BoxFuture<'a, Result<(), Self::Error>>;
    fn on_left_channel<'a>(
        &'a self,
        ctx: &'a Context,
        req: OnLeftChannel,
    ) -> BoxFuture<'a, Result<(), Self::Error>>;
    fn on_message_created<'a>(
        &'a self,
        ctx: &'a Context,
        req: OnMessageCreated,
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
        req: BuildRequestAsBot<'a>,
    ) -> BoxFuture<
        'a,
        Result<RequestBuilder, <Self::TraqBotService as TraqBotService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_bot_service().build_request_as_bot(ctx, req)
    }
    fn subscribe_channel(
        &self,
        req: SubscribeChannel,
    ) -> BoxFuture<
        '_,
        Result<
            BoxStream<'static, super::message::TraqMessage>,
            <Self::TraqBotService as TraqBotService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_bot_service().subscribe_channel(ctx, req)
    }
    fn leave_channel(
        &self,
        req: LeaveChannel,
    ) -> BoxFuture<'_, Result<(), <Self::TraqBotService as TraqBotService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.traq_bot_service().leave_channel(ctx, req)
    }
    fn on_message_created(
        &self,
        req: OnMessageCreated,
    ) -> BoxFuture<'_, Result<(), <Self::TraqBotService as TraqBotService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.traq_bot_service().on_message_created(ctx, req)
    }
}
