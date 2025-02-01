use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub mod error;
mod r#impl;

pub use error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TraqMessageId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TraqMessage {
    pub id: TraqMessageId,
    pub channel_id: super::channel::TraqChannelId,
    pub user_id: super::user::TraqUserId,
    pub content: String,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SyncedTraqMessage {
    pub id: TraqMessageId,
    pub channel_id: super::channel::TraqChannelId,
    pub user_id: super::user::TraqUserId,
    pub inner: crate::message::Message,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SendMessageParams {
    pub inner: crate::message::Message,
    pub channel_id: super::channel::TraqChannelId,
    pub user_id: super::user::TraqUserId,
    // ユーザーはtraQのユーザーと1対1で紐づいている前提
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RecvMessageParams {
    pub traq_message: TraqMessage,
    pub user_id: crate::user::UserId,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckMessageSyncedParams {
    ToTraq(crate::message::Message),
    FromTraq(TraqMessage),
}

pub trait TraqMessageService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    /// アプリに投稿されたメッセージをtraQに反映される
    fn send_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: SendMessageParams,
    ) -> BoxFuture<'a, Result<SyncedTraqMessage, Self::Error>>;
    /// traQのメッセージをアプリに反映させる
    fn recv_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: RecvMessageParams,
    ) -> BoxFuture<'a, Result<SyncedTraqMessage, Self::Error>>;
    fn check_message_synced<'a>(
        &'a self,
        ctx: &'a Context,
        params: CheckMessageSyncedParams,
    ) -> BoxFuture<'a, Result<Option<SyncedTraqMessage>, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideTraqMessageService: Send + Sync + 'static {
    type Context;
    type TraqMessageService: TraqMessageService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn traq_message_service(&self) -> &Self::TraqMessageService;

    fn send_message(
        &self,
        params: SendMessageParams,
    ) -> BoxFuture<
        '_,
        Result<
            SyncedTraqMessage,
            <Self::TraqMessageService as TraqMessageService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_message_service().send_message(ctx, params)
    }
    fn recv_message(
        &self,
        params: RecvMessageParams,
    ) -> BoxFuture<
        '_,
        Result<
            SyncedTraqMessage,
            <Self::TraqMessageService as TraqMessageService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_message_service().recv_message(ctx, params)
    }
    fn check_message_synced(
        &self,
        params: CheckMessageSyncedParams,
    ) -> BoxFuture<
        '_,
        Result<
            Option<SyncedTraqMessage>,
            <Self::TraqMessageService as TraqMessageService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_message_service()
            .check_message_synced(ctx, params)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TraqMessageServiceImpl;
