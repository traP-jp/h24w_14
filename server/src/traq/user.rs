//! traQのユーザーと連携

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::{IntoStatus, Timestamp};

pub mod error;
mod r#impl;

pub use error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TraqUserId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TraqUser {
    pub id: TraqUserId,
    pub inner: crate::user::User,
    pub bot: bool,
    pub bio: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetTraqUserParams {
    pub id: TraqUserId,
}

pub type FindTraqUserParams = GetTraqUserParams;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RegisterTraqUserParams {
    pub id: TraqUserId,
}

pub trait TraqUserService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetTraqUserParams,
    ) -> BoxFuture<'a, Result<TraqUser, Self::Error>>;
    fn find_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: FindTraqUserParams,
    ) -> BoxFuture<'a, Result<Option<TraqUser>, Self::Error>>;
    fn register_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: RegisterTraqUserParams,
    ) -> BoxFuture<'a, Result<TraqUser, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideTraqUserService: Send + Sync + 'static {
    type Context;
    type TraqUserService: TraqUserService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn traq_user_service(&self) -> &Self::TraqUserService;

    fn get_traq_user(
        &self,
        params: GetTraqUserParams,
    ) -> BoxFuture<
        '_,
        Result<TraqUser, <Self::TraqUserService as TraqUserService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_user_service().get_traq_user(ctx, params)
    }
    fn find_traq_user(
        &self,
        params: FindTraqUserParams,
    ) -> BoxFuture<
        '_,
        Result<Option<TraqUser>, <Self::TraqUserService as TraqUserService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_user_service().find_traq_user(ctx, params)
    }
    fn register_traq_user(
        &self,
        params: RegisterTraqUserParams,
    ) -> BoxFuture<
        '_,
        Result<TraqUser, <Self::TraqUserService as TraqUserService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_user_service().register_traq_user(ctx, params)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TraqUserServiceImpl;
