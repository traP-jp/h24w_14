//! `message.proto`

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::{IntoStatus, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MessageId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Message {
    pub id: MessageId,
    pub user_id: crate::user::UserId,
    pub position: crate::world::Coordinate,
    pub content: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub expires_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetMessageParams {
    pub id: MessageId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetMessagesInAreaParams {
    pub center: crate::world::Coordinate,
    pub size: crate::world::Size,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateMessageParams {
    pub user_id: crate::user::UserId,
    pub position: crate::world::Coordinate,
    pub content: String,
}

pub trait MessageService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetMessageParams,
    ) -> BoxFuture<'a, Result<Message, Self::Error>>;
    fn get_messages_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetMessagesInAreaParams,
    ) -> BoxFuture<'a, Result<Vec<Message>, Self::Error>>;
    fn create_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: CreateMessageParams,
    ) -> BoxFuture<'a, Result<Message, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideMessageService: Send + Sync + 'static {
    type Context;
    type MessageService: MessageService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn message_service(&self) -> &Self::MessageService;

    fn get_message(
        &self,
        params: GetMessageParams,
    ) -> BoxFuture<
        '_,
        Result<Message, <Self::MessageService as MessageService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.message_service().get_message(ctx, params)
    }
    fn create_message(
        &self,
        params: CreateMessageParams,
    ) -> BoxFuture<
        '_,
        Result<Message, <Self::MessageService as MessageService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.message_service().create_message(ctx, params)
    }

    // TODO: build_server(this: Arc<Self>) -> MessageServiceServer<...>
}
