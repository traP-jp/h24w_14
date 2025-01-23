//! `world.proto`

pub mod error;
pub mod grpc;
mod r#impl;

use std::sync::Arc;

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
pub struct GetWorldSizeParams {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CheckCoordinateParams {
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
        params: GetWorldSizeParams,
    ) -> BoxFuture<'a, Result<Size, Self::Error>>;
    fn check_coordinate<'a>(
        &'a self,
        ctx: &'a Context,
        params: CheckCoordinateParams,
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
        params: GetWorldSizeParams,
    ) -> BoxFuture<'_, Result<Size, <Self::WorldService as WorldService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.world_service().get_world_size(ctx, params)
    }
    fn check_coordinate(
        &self,
        params: CheckCoordinateParams,
    ) -> BoxFuture<
        '_,
        Result<CheckCoordinateAnswer, <Self::WorldService as WorldService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.world_service().check_coordinate(ctx, params)
    }

    fn build_server(this: Arc<Self>) -> WorldServiceServer<Self>
    where
        Self: Sized,
    {
        let service = grpc::ServiceImpl::new(this);
        WorldServiceServer::new(service)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WorldServiceImpl;

pub type WorldServiceServer<State> =
    schema::world::world_service_server::WorldServiceServer<grpc::ServiceImpl<State>>;
