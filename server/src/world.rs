//! `world.proto`

pub mod error;
pub mod grpc;
mod r#impl;

use std::sync::Arc;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub use error::Error;
pub use schema::world::world_service_server::SERVICE_NAME;

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

impl Coordinate {
    pub fn is_inside_circle(self, center: Coordinate, radius: u32) -> bool {
        let dx = self.x.abs_diff(center.x) as u64;
        let dy = self.y.abs_diff(center.y) as u64;
        let distance = dx * dx + dy * dy;
        let radius = radius as u64;
        distance <= radius * radius
    }
}

#[test]
fn test_coordinate_is_inside_circle() {
    let center = Coordinate { x: 0, y: 0 };
    let radius = 5;
    assert!(Coordinate { x: 0, y: 0 }.is_inside_circle(center, radius));
    assert!(Coordinate { x: 5, y: 0 }.is_inside_circle(center, radius));
    assert!(Coordinate { x: 0, y: 5 }.is_inside_circle(center, radius));
    assert!(Coordinate { x: 3, y: 4 }.is_inside_circle(center, radius));
    assert!(!Coordinate { x: 1, y: 5 }.is_inside_circle(center, radius));
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct WorldSize(pub Size);

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
}

pub fn build_server<State>(state: Arc<State>) -> WorldServiceServer<State>
where
    State: ProvideWorldService + Sized,
{
    let service = grpc::ServiceImpl::new(state);
    WorldServiceServer::new(service)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WorldServiceImpl;

pub type WorldServiceServer<State> =
    schema::world::world_service_server::WorldServiceServer<grpc::ServiceImpl<State>>;
