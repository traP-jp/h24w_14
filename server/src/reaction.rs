//! `reaction.proto`

pub mod error;
pub mod grpc;
mod r#impl;

use std::sync::Arc;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::{IntoStatus, Timestamp};

pub use error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ReactionId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Reaction {
    pub id: ReactionId,
    pub user_id: crate::user::UserId,
    pub position: crate::world::Coordinate,
    pub kind: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub expires_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetReactionParams {
    pub id: ReactionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateReactionParams {
    pub user_id: crate::user::UserId,
    pub position: crate::world::Coordinate,
    pub kind: String,
}

pub trait ReactionService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_reaction<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetReactionParams,
    ) -> BoxFuture<'a, Result<Reaction, Self::Error>>;
    fn create_reaction<'a>(
        &'a self,
        ctx: &'a Context,
        params: CreateReactionParams,
    ) -> BoxFuture<'a, Result<Reaction, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideReactionService: Send + Sync + 'static {
    type Context;
    type ReactionService: ReactionService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn reaction_service(&self) -> &Self::ReactionService;

    fn get_reaction(
        &self,
        params: GetReactionParams,
    ) -> BoxFuture<
        '_,
        Result<Reaction, <Self::ReactionService as ReactionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.reaction_service().get_reaction(ctx, params)
    }
    fn create_reaction(
        &self,
        params: CreateReactionParams,
    ) -> BoxFuture<
        '_,
        Result<Reaction, <Self::ReactionService as ReactionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.reaction_service().create_reaction(ctx, params)
    }
}

pub fn build_server<State>(this: Arc<State>) -> ReactionServiceServer<State>
where
    State: ProvideReactionService + crate::session::ProvideSessionService,
{
    let service = grpc::ServiceImpl::new(this);
    ReactionServiceServer::new(service)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReactionServiceImpl;

pub type ReactionServiceServer<State> =
    schema::reaction::reaction_service_server::ReactionServiceServer<grpc::ServiceImpl<State>>;

pub use schema::reaction::reaction_service_server::SERVICE_NAME;
