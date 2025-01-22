use std::sync::Arc;

use schema::world as schema;

use crate::prelude::IntoStatus;

// MARK: type conversions

impl From<super::Coordinate> for schema::Coordinate {
    fn from(value: super::Coordinate) -> Self {
        let super::Coordinate { x, y } = value;
        Self { x, y }
    }
}

impl From<schema::Coordinate> for super::Coordinate {
    fn from(value: schema::Coordinate) -> Self {
        let schema::Coordinate { x, y } = value;
        Self { x, y }
    }
}

impl From<super::Size> for schema::Size {
    fn from(value: super::Size) -> Self {
        let super::Size { width, height } = value;
        Self { width, height }
    }
}

impl From<schema::Size> for super::Size {
    fn from(value: schema::Size) -> Self {
        let schema::Size { width, height } = value;
        Self { width, height }
    }
}

// MARK: ServiceImpl

pub struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State>
where
    State: super::ProvideWorldService,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<State> ServiceImpl<State>
where
    State: super::ProvideWorldService,
{
    pub(super) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl<State> schema::world_service_server::WorldService for ServiceImpl<State>
where
    State: super::ProvideWorldService,
{
    async fn get_world(
        &self,
        request: tonic::Request<schema::GetWorldRequest>,
    ) -> tonic::Result<tonic::Response<schema::GetWorldResponse>> {
        let (_, _, schema::GetWorldRequest {}) = request.into_parts();
        let req = super::GetWorldSize {};
        let size = self
            .state
            .get_world_size(req)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let world = schema::World { size: Some(size) };
        let res = schema::GetWorldResponse { world: Some(world) };
        Ok(tonic::Response::new(res))
    }
}
