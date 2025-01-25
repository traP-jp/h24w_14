pub mod error;
pub mod grpc;
pub mod r#impl;

use std::sync::Arc;

use futures::future::BoxFuture;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub use error::Error;
pub use schema::auth::auth_service_server::SERVICE_NAME;

#[derive(Debug, Clone)]
pub struct TraqOauthClientConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OAuth2EntrypointUriParams {}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AuthorizedUser {
    pub user_id: super::user::TraqUserId,
}

#[derive(Debug, Clone)]
pub struct BuildRequestAsAuthorizedUserParams<'a> {
    pub user: &'a AuthorizedUser,
    pub method: http::Method,
    pub uri: &'a str,
}

pub trait TraqAuthService<Context>: Send + Sync + 'static {
    type Error: axum::response::IntoResponse + IntoStatus;

    fn oauth2_entrypoint_uri<'a>(
        &'a self,
        ctx: &'a Context,
        params: OAuth2EntrypointUriParams,
    ) -> BoxFuture<'a, Result<String, Self::Error>>;
    fn oauth2_handle_redirect<'a>(
        &'a self,
        ctx: &'a Context,
        req: http::Request<()>,
    ) -> BoxFuture<'a, Result<AuthorizedUser, Self::Error>>;
    fn check_authorized<'a>(
        &'a self,
        ctx: &'a Context,
        user_id: super::user::TraqUserId,
    ) -> BoxFuture<'a, Result<Option<AuthorizedUser>, Self::Error>>;
    /// Bearer Token を設定した [`RequestBuilder`] を作る
    ///
    /// 設定する Bearer Token は OAuth2.0 Authorization Code Flow で取得したもの
    fn build_request_as_authorized_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: BuildRequestAsAuthorizedUserParams<'a>,
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
        params: OAuth2EntrypointUriParams,
    ) -> BoxFuture<
        '_,
        Result<String, <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_auth_service().oauth2_entrypoint_uri(ctx, params)
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
        user_id: super::user::TraqUserId,
    ) -> BoxFuture<
        '_,
        Result<
            Option<AuthorizedUser>,
            <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_auth_service().check_authorized(ctx, user_id)
    }
    fn build_request_as_authorized_user<'a>(
        &'a self,
        params: BuildRequestAsAuthorizedUserParams<'a>,
    ) -> BoxFuture<
        'a,
        Result<RequestBuilder, <Self::TraqAuthService as TraqAuthService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.traq_auth_service()
            .build_request_as_authorized_user(ctx, params)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TraqAuthServiceImpl;

pub fn build_server<State>(this: Arc<State>) -> AuthServiceServer<State>
where
    State: ProvideTraqAuthService + crate::session::ProvideSessionService + Sized,
{
    let service = grpc::ServiceImpl::new(this);
    AuthServiceServer::new(service)
}

pub type AuthServiceServer<State> =
    schema::auth::auth_service_server::AuthServiceServer<grpc::ServiceImpl<State>>;
