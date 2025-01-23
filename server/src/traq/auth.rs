use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OAuth2EntrypointUri {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AuthorizedUser {
    pub user: super::user::TraqUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "answer", rename_all = "snake_case")]
pub enum CheckAuthorizedAnswer {
    Authorized(AuthorizedUser),
    Unauthorized,
}

pub trait TraqAuthService<Context>: Send + Sync + 'static {
    type Error: axum::response::IntoResponse + IntoStatus;

    fn oauth2_entrypoint_uri<'a>(
        &'a self,
        ctx: &'a Context,
        req: OAuth2EntrypointUri,
    ) -> BoxFuture<'a, Result<String, Self::Error>>;
    fn oauth2_handle_redirect<'a>(
        &'a self,
        ctx: &'a Context,
        req: http::Request<()>,
    ) -> BoxFuture<'a, Result<AuthorizedUser, Self::Error>>;
    fn check_authorized<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::user::TraqUser,
    ) -> BoxFuture<'a, Result<CheckAuthorizedAnswer, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideTraqAuthService: Send + Sync + 'static {
    type Context;
    type TraqAuthService: TraqAuthService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn traq_auth_service(&self) -> &Self::TraqAuthService;

    fn oauth2_entrypoint_uri(
        &self,
        req: OAuth2EntrypointUri,
    ) -> BoxFuture<
        '_,
        Result<String, <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_auth_service().oauth2_entrypoint_uri(ctx, req)
    }
    fn oauth2_handle_redirect(
        &self,
        req: http::Request<()>,
    ) -> BoxFuture<
        '_,
        Result<AuthorizedUser, <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_auth_service().oauth2_handle_redirect(ctx, req)
    }
    fn check_authorized(
        &self,
        req: super::user::TraqUser,
    ) -> BoxFuture<
        '_,
        Result<
            CheckAuthorizedAnswer,
            <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_auth_service().check_authorized(ctx, req)
    }
}
