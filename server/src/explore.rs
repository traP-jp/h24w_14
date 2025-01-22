//! `explore.proto`

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExplorerId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Explorer {
    pub id: ExplorerId,
    pub inner: crate::user::User,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ExplorationField {
    pub position: crate::world::Coordinate,
    pub size: crate::world::Size,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExplorerAction {
    Arrive(Explorer),
    Move(Explorer),
    Leave(Explorer),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ExplorationFieldEvents {
    pub messages: Vec<crate::message::Message>,
    pub speaker_phones: Vec<crate::speaker_phone::SpeakerPhone>,
    pub reactions: Vec<crate::reaction::Reaction>,
    pub explorer_actions: Vec<ExplorerAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetExplorer {
    pub id: ExplorerId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateExplorer {
    pub inner: crate::user::User,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GetExplorersInArea {
    Rect {
        center: crate::world::Coordinate,
        size: crate::world::Size,
    },
    // これ必要かわからん
    // Circle {
    //     center: crate::world::Coordinate,
    //     radius: u32,
    // },
}

pub trait ExplorerService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetExplorer,
    ) -> BoxFuture<'a, Result<Explorer, Self::Error>>;
    fn create_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        req: CreateExplorer,
    ) -> BoxFuture<'a, Result<Explorer, Self::Error>>;
    fn get_explorers_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetExplorersInArea,
    ) -> BoxFuture<'a, Result<Vec<Explorer>, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideExplorerService: Send + Sync + 'static {
    type Context;
    type ExplorerService: ExplorerService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn explorer_service(&self) -> &Self::ExplorerService;

    fn get_explorer(
        &self,
        req: GetExplorer,
    ) -> BoxFuture<
        '_,
        Result<Explorer, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().get_explorer(ctx, req)
    }
    fn create_explorer(
        &self,
        req: CreateExplorer,
    ) -> BoxFuture<
        '_,
        Result<Explorer, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().create_explorer(ctx, req)
    }
    fn get_explorers_in_area(
        &self,
        req: GetExplorersInArea,
    ) -> BoxFuture<
        '_,
        Result<Vec<Explorer>, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().get_explorers_in_area(ctx, req)
    }
}
