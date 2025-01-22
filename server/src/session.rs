//! HTTP セッション管理

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub struct Extract<'a>(pub &'a http::HeaderMap);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Session {
    pub user_id: crate::user::UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Save {
    pub user_id: crate::user::UserId,
}

pub trait SessionService<Context>: Send + Sync + 'static {
    type Jar: axum::response::IntoResponseParts + 'static;
    type Error: IntoStatus;

    fn extract<'a>(
        &'a self,
        ctx: &'a Context,
        req: Extract<'a>,
    ) -> BoxFuture<'a, Result<Session, Self::Error>>;
    fn save<'a>(
        &'a self,
        ctx: &'a Context,
        req: Save,
    ) -> BoxFuture<'a, Result<Self::Jar, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideSessionService: Send + Sync + 'static {
    type Context;
    type SessionService: SessionService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn session_service(&self) -> &Self::SessionService;

    fn extract<'a>(
        &'a self,
        req: Extract<'a>,
    ) -> BoxFuture<
        'a,
        Result<Session, <Self::SessionService as SessionService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.session_service().extract(ctx, req)
    }
    fn save(
        &self,
        req: Save,
    ) -> BoxFuture<
        '_,
        Result<
            <Self::SessionService as SessionService<Self::Context>>::Jar,
            <Self::SessionService as SessionService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.session_service().save(ctx, req)
    }
}
