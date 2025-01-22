//! `reaction.proto`

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::{IntoStatus, Timestamp};

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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetReaction {
    pub id: ReactionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateReaction {
    pub position: crate::world::Coordinate,
    pub kind: String,
}

pub trait ReactionService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_reaction<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetReaction,
    ) -> BoxFuture<'a, Result<Reaction, Self::Error>>;
    fn create_reaction<'a>(
        &'a self,
        ctx: &'a Context,
        req: CreateReaction,
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
        req: GetReaction,
    ) -> BoxFuture<
        '_,
        Result<Reaction, <Self::ReactionService as ReactionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.reaction_service().get_reaction(ctx, req)
    }
    fn create_reaction(
        &self,
        req: CreateReaction,
    ) -> BoxFuture<
        '_,
        Result<Reaction, <Self::ReactionService as ReactionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.reaction_service().create_reaction(ctx, req)
    }

    // TODO: build_server(this: Arc<Self>) -> ReactionServiceServer<...>
}
