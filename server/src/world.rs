//! `world.proto`

pub mod error;
mod r#impl;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub use error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetWorldSize {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CheckCoordinate {
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "check", rename_all = "snake_case")]
pub enum CheckCoordinateAnswer {
    Valid(Coordinate),
    Invalid,
}

pub trait WorldSizeStore: Send + Sync + 'static {
    fn world_size(&self) -> Size;
}

pub trait WorldService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_world_size<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetWorldSize,
    ) -> BoxFuture<'a, Result<Size, Self::Error>>;
    fn check_coordinate<'a>(
        &'a self,
        ctx: &'a Context,
        req: CheckCoordinate,
    ) -> BoxFuture<'a, Result<CheckCoordinateAnswer, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideWorldService: Send + Sync + 'static {
    type Context;
    type WorldService: WorldService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn world_service(&self) -> &Self::WorldService;

    fn get_world_size(
        &self,
        req: GetWorldSize,
    ) -> BoxFuture<'_, Result<Size, <Self::WorldService as WorldService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.world_service().get_world_size(ctx, req)
    }
    fn check_coordinate(
        &self,
        req: CheckCoordinate,
    ) -> BoxFuture<
        '_,
        Result<CheckCoordinateAnswer, <Self::WorldService as WorldService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.world_service().check_coordinate(ctx, req)
    }

    // TODO: build_server(this: Arc<Self>) -> WorldServiceServer<...>
}
