//! HTTP セッション管理

pub mod error;
pub mod r#impl;
pub mod layer;

use std::sync::Arc;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub use axum_extra::extract::cookie::PrivateCookieJar;
pub use error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SessionName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CookieDomain(pub String);

pub struct ExtractParams<'a>(pub &'a http::HeaderMap);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Session {
    pub user_id: crate::user::UserId,
}

#[derive(Debug, Clone)]
pub struct SaveParams<'a> {
    pub user_id: crate::user::UserId,
    pub header_map: &'a http::HeaderMap,
}

pub trait SessionService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn extract<'a>(
        &'a self,
        ctx: &'a Context,
        params: ExtractParams<'a>,
    ) -> BoxFuture<'a, Result<Session, Self::Error>>;
    fn save<'a>(
        &'a self,
        ctx: &'a Context,
        params: SaveParams,
    ) -> BoxFuture<'a, Result<PrivateCookieJar, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideSessionService: Send + Sync + 'static {
    type Context;
    type SessionService: SessionService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn session_service(&self) -> &Self::SessionService;

    fn extract<'a>(
        &'a self,
        params: ExtractParams<'a>,
    ) -> BoxFuture<
        'a,
        Result<Session, <Self::SessionService as SessionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.session_service().extract(ctx, params)
    }

    fn save(
        &self,
        params: SaveParams,
    ) -> BoxFuture<
        '_,
        Result<PrivateCookieJar, <Self::SessionService as SessionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.session_service().save(ctx, params)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SessionServiceImpl;

// extract できなかったら Unauthorized を返すレイヤー
pub fn build_http_layer<State>(state: Arc<State>) -> layer::SessionLayer<State, layer::HTTP>
where
    State: ProvideSessionService,
{
    layer::SessionLayer {
        state,
        _kind: layer::HTTP,
    }
}

pub fn build_grpc_layer<State>(state: Arc<State>) -> layer::SessionLayer<State, layer::Grpc>
where
    State: ProvideSessionService,
{
    layer::SessionLayer {
        state,
        _kind: layer::Grpc,
    }
}
