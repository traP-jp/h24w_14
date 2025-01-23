use futures::future::BoxFuture;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OAuth2EntrypointUri {}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AuthorizedUser {
    pub user: super::user::TraqUser,
}

#[derive(Debug, Clone)]
pub struct BuildRequestAsAuthorizedUser<'a> {
    pub user: &'a AuthorizedUser,
    pub method: http::Method,
    pub uri: &'a str,
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
    ) -> BoxFuture<'a, Result<Option<AuthorizedUser>, Self::Error>>;
    /// Bearer Token を設定した [`RequestBuilder`] を作る
    ///
    /// 設定する Bearer Token は OAuth2.0 Authorization Code Flow で取得したもの
    fn build_request_as_authorized_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: BuildRequestAsAuthorizedUser<'a>,
    ) -> BoxFuture<'a, Result<RequestBuilder, Self::Error>>;
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
            Option<AuthorizedUser>,
            <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_auth_service().check_authorized(ctx, req)
    }
    fn build_request_as_authorized_user<'a>(
        &'a self,
        req: BuildRequestAsAuthorizedUser<'a>,
    ) -> BoxFuture<
        'a,
        Result<RequestBuilder, <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_auth_service()
            .build_request_as_authorized_user(ctx, req)
    }
}
