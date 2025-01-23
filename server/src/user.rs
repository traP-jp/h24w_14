//! user.proto

pub mod error;
pub mod grpc;
mod r#impl;

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

pub trait UserStore: Send + Sync + 'static {
    fn find(&self, id: UserId) -> Option<User>;
    fn create(&self, name: String, display_name: String) -> User;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetUser {
    pub id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub display_name: String,
}

pub trait UserService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetUser,
    ) -> BoxFuture<'a, Result<User, Self::Error>>;
    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: CreateUser,
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
        req: GetUser,
    ) -> BoxFuture<'_, Result<User, <Self::UserService as UserService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.user_service().get_user(ctx, req)
    }
    fn create_user(
        &self,
        req: CreateUser,
    ) -> BoxFuture<'_, Result<User, <Self::UserService as UserService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.user_service().create_user(ctx, req)
    }
    // TODO: build_server(this: Arc<Self>) -> UserServiceServer<...>
    //     get_userをgRPCのUserServiceで公開する
}

#[derive(Debug, Clone, Copy)]
pub struct UserServiceImpl;

pub type UserServiceServer<State> =
    schema::user::user_service_server::UserServiceServer<grpc::ServiceImpl<State>>;
