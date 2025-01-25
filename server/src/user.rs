//! user.proto

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
pub struct UserId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub display_name: String,
    pub created_at: Timestamp,
    // 予約
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetUserParams {
    pub id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateUserParams {
    pub name: String,
    pub display_name: String,
}

pub trait UserService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetUserParams,
    ) -> BoxFuture<'a, Result<User, Self::Error>>;
    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: CreateUserParams,
    ) -> BoxFuture<'a, Result<User, Self::Error>>;

    // NOTE: `update_user`と`delete_user`は今の所実装しない
}

#[allow(clippy::type_complexity)]
pub trait ProvideUserService: Send + Sync + 'static {
    type Context;
    type UserService: UserService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn user_service(&self) -> &Self::UserService;

    fn get_user(
        &self,
        params: GetUserParams,
    ) -> BoxFuture<'_, Result<User, <Self::UserService as UserService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.user_service().get_user(ctx, params)
    }
    fn create_user(
        &self,
        params: CreateUserParams,
    ) -> BoxFuture<'_, Result<User, <Self::UserService as UserService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.user_service().create_user(ctx, params)
    }
}

pub fn build_server<State>(state: Arc<State>) -> UserServiceServer<State>
where
    State: ProvideUserService + crate::session::ProvideSessionService,
{
    let service = grpc::ServiceImpl::new(state);
    UserServiceServer::new(service)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UserServiceImpl;

pub type UserServiceServer<State> =
    schema::user::user_service_server::UserServiceServer<grpc::ServiceImpl<State>>;

pub use schema::user::user_service_server::SERVICE_NAME;
