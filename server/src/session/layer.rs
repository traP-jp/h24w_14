use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{body::Body, response::IntoResponse};
use axum_extra::extract::{cookie::Key, PrivateCookieJar};
use http::{Request, Response};
use tower::{Layer, Service};

use super::SessionName;

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
    State: AsRef<Key> + AsRef<SessionName>,
    Kind: Copy + ToResponse,
{
    type Response = Response<Body>;
    type Error = SessionError;
    type Future = SessionFuture<S::Future, Kind>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // NOTE: service が Infallible を返すので、ここでエラーは発生しない
        self.inner.poll_ready(ctx).map_err(|_| unreachable!())
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let key: &Key = self.state.as_ref().as_ref();
        let session_name: &SessionName = self.state.as_ref().as_ref();
        let jar = PrivateCookieJar::from_headers(req.headers(), key.clone());
        let user_id = jar
            .get(&session_name.0)
            .and_then(|cookie| cookie.value().parse().ok())
            .map(crate::user::UserId);

        let fut = self.inner.call(req);
        SessionFuture {
            fut,
            authorized: user_id.is_some(),
            _kind: self._kind,
        }
    }
}

pin_project_lite::pin_project! {
    pub struct SessionFuture<Fut, Kind> {
        #[pin]
        fut: Fut,
        #[pin]
        authorized: bool,
        #[pin]
        _kind: Kind,
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

impl<Fut, ResBody, ServiceError, Kind> Future for SessionFuture<Fut, Kind>
where
    Fut: Future<Output = Result<Response<ResBody>, ServiceError>>,
    ServiceError: Into<Infallible> + 'static,
    ResBody: Into<Body> + 'static,
    Kind: ToResponse,
{
    type Output = Result<Response<Body>, SessionError>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.authorized {
            return Poll::Ready(Ok(Kind::unauthorized()));
        }

        let this = self.project();
        this.fut
            .poll(ctx)
            .map_ok(|r| r.map(Into::into))
            .map_err(|_| unreachable!())
    }
}

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
pub fn build_http_layer<State>(state: Arc<State>) -> SessionLayer<State, HTTP> {
    SessionLayer { state, _kind: HTTP }
}

pub fn build_grpc_layer<State>(state: Arc<State>) -> SessionLayer<State, Grpc> {
    SessionLayer { state, _kind: Grpc }
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::HTTP {}
    impl Sealed for super::Grpc {}
}
