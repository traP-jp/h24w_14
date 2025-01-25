//! `explore.proto`

use std::collections::HashMap;
use std::sync::Arc;

use futures::{future::BoxFuture, stream::BoxStream};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::prelude::IntoStatus;

pub mod error;
mod r#impl;

pub use error::Error;

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

impl ExplorerAction {
    pub fn explorer(&self) -> &Explorer {
        match self {
            Self::Arrive(explorer) => explorer,
            Self::Move(explorer) => explorer,
            Self::Leave(explorer) => explorer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ExplorationFieldEvents {
    pub messages: Vec<crate::message::Message>,
    pub speaker_phones: Vec<crate::speaker_phone::SpeakerPhone>,
    pub reactions: Vec<crate::reaction::Reaction>,
    pub explorer_actions: Vec<ExplorerAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetExplorerParams {
    pub id: ExplorerId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateExplorerParams {
    pub inner: crate::user::User,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdateExplorerParams {
    pub id: ExplorerId,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DeleteExplorerParams {
    pub id: ExplorerId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GetExplorersInAreaParams {
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

pub struct ExploreParams<'a> {
    pub id: crate::user::UserId,
    pub stream: BoxStream<'a, ExplorationField>,
}

#[derive(Clone)]
pub struct ExplorerStore {
    explorers: Arc<RwLock<HashMap<ExplorerId, Explorer>>>,
}

pub trait ExploreService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn explore<'a>(
        &'a self,
        ctx: &'a Context,
        params: ExploreParams<'a>,
    ) -> BoxStream<'a, Result<ExplorationFieldEvents, Self::Error>>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExploreServiceImpl;

#[allow(clippy::type_complexity)]
pub trait ProvideExploreService: Send + Sync + 'static {
    type Context;
    type ExploreService: ExploreService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn explore_service(&self) -> &Self::ExploreService;

    fn explore<'a>(
        &'a self,
        req: ExploreParams<'a>,
    ) -> BoxStream<
        'a,
        Result<
            ExplorationFieldEvents,
            <Self::ExploreService as ExploreService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.explore_service().explore(ctx, req)
    }
}

pub trait ExplorerService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetExplorerParams,
    ) -> BoxFuture<'a, Result<Explorer, Self::Error>>;
    fn create_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: CreateExplorerParams,
    ) -> BoxFuture<'a, Result<Explorer, Self::Error>>;
    fn get_explorers_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetExplorersInAreaParams,
    ) -> BoxFuture<'a, Result<Vec<Explorer>, Self::Error>>;
    fn update_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: UpdateExplorerParams,
    ) -> BoxFuture<'a, Result<Explorer, Self::Error>>;
    fn delete_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: DeleteExplorerParams,
    ) -> BoxFuture<'a, Result<Explorer, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideExplorerService: Send + Sync + 'static {
    type Context;
    type ExplorerService: ExplorerService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn explorer_service(&self) -> &Self::ExplorerService;

    fn get_explorer(
        &self,
        params: GetExplorerParams,
    ) -> BoxFuture<
        '_,
        Result<Explorer, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().get_explorer(ctx, params)
    }
    /// Explorerを作成して, `ExplorerAction::Arrive`イベントを`EventService`に発行する
    fn create_explorer(
        &self,
        params: CreateExplorerParams,
    ) -> BoxFuture<
        '_,
        Result<Explorer, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().create_explorer(ctx, params)
    }
    fn get_explorers_in_area(
        &self,
        params: GetExplorersInAreaParams,
    ) -> BoxFuture<
        '_,
        Result<Vec<Explorer>, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().get_explorers_in_area(ctx, params)
    }
    /// Explorerを作成して, `ExplorerAction::Move`イベントを`EventService`に発行する
    fn update_explorer(
        &self,
        params: UpdateExplorerParams,
    ) -> BoxFuture<
        '_,
        Result<Explorer, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().update_explorer(ctx, params)
    }
    /// Explorerを作成して, `ExplorerAction::Leave`イベントを`EventService`に発行する
    fn delete_explorer(
        &self,
        params: DeleteExplorerParams,
    ) -> BoxFuture<
        '_,
        Result<Explorer, <Self::ExplorerService as ExplorerService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.explorer_service().delete_explorer(ctx, params)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExplorerServiceImpl;
