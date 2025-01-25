use std::{convert::Infallible, sync::Arc};

use axum::{body::Body, response::IntoResponse};
use futures::future::BoxFuture;
use http::{Request, Response};
use tower::{Layer, Service};

pub struct SessionLayer<State, Kind> {
    state: Arc<State>,
    _kind: Kind,
}

impl<State, Kind> Clone for SessionLayer<State, Kind>
where
    Kind: Copy,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            _kind: self._kind,
        }
    }
}

impl<S, State, Kind> Layer<S> for SessionLayer<State, Kind>
where
    Kind: Copy,
{
    type Service = SessionService<S, State, Kind>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionService {
            inner,
            state: Arc::clone(&self.state),
            _kind: self._kind,
        }
    }
}

pub struct SessionService<Service, State, Kind> {
    inner: Service,
    state: Arc<State>,
    _kind: Kind,
}

impl<S, State, ResBody, Kind> SessionService<S, State, Kind>
where
    S: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + Sync + 'static,
    ResBody: Into<Body> + 'static,
    Response<ResBody>: IntoResponse + 'static,
    <S as Service<Request<Body>>>::Error: Into<Infallible> + 'static,
    <S as Service<Request<Body>>>::Future: Send + 'static,
{
    pub fn new(inner: S, state: Arc<State>, kind: Kind) -> Self {
        Self {
            inner,
            state,
            _kind: kind,
        }
    }
}
impl<S, State, ResBody, Kind> Service<Request<Body>> for SessionService<S, State, Kind>
where
    S: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + Sync + 'static,
    ResBody: Into<Body> + 'static,
    Response<ResBody>: IntoResponse + 'static,
    <S as Service<Request<Body>>>::Error: Into<Infallible> + 'static,
    <S as Service<Request<Body>>>::Future: Send + 'static,
    State: super::ProvideSessionService,
    Kind: Copy + ToResponse,
{
    type Response = Response<Body>;
    type Error = SessionError;
    type Future = BoxFuture<'static, Result<Response<Body>, SessionError>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // NOTE: service が Infallible を返すので、ここでエラーは発生しない
        self.inner.poll_ready(ctx).map_err(|_| unreachable!())
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let ctx = self.state.clone();
        let mut srv = self.inner.clone();
        std::mem::swap(&mut srv, &mut self.inner);
        Box::pin(async move {
            let extract_params = super::ExtractParams(req.headers());
            match ctx.extract(extract_params).await {
                Ok(super::Session { user_id }) => {
                    tracing::trace!(user_id = %user_id.0, "pass session");
                }
                Err(_) => return Ok(Kind::unauthorized()),
            };
            srv.call(req)
                .await
                .map(|r| r.map(Into::into))
                .map_err(|_| unreachable!())
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SessionError {}
impl std::fmt::Display for SessionError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for SessionError {}

pub trait ToResponse: private::Sealed {
    fn unauthorized() -> Response<Body>;
}
#[derive(Debug, Clone, Copy, Default)]
pub struct HTTP;
#[derive(Debug, Clone, Copy, Default)]
pub struct Grpc;

impl ToResponse for HTTP {
    fn unauthorized() -> Response<Body> {
        Response::builder()
            .status(http::StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap()
    }
}

impl ToResponse for Grpc {
    fn unauthorized() -> Response<Body> {
        tonic::Status::unauthenticated("Unauthorized")
            .into_http()
            .map(Body::new)
    }
}

// extract できなかったら Unauthorized を返すレイヤー
pub fn build_http_layer<State>(state: Arc<State>) -> SessionLayer<State, HTTP>
where
    State: super::ProvideSessionService,
{
    SessionLayer { state, _kind: HTTP }
}

pub fn build_grpc_layer<State>(state: Arc<State>) -> SessionLayer<State, Grpc>
where
    State: super::ProvideSessionService,
{
    SessionLayer { state, _kind: Grpc }
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::HTTP {}
    impl Sealed for super::Grpc {}
}
