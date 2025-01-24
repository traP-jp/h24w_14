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

pub struct SessionLayer<State> {
    state: Arc<State>,
}

impl<S, State> Layer<S> for SessionLayer<State> {
    type Service = SessionService<S, State>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionService {
            inner,
            state: Arc::clone(&self.state),
        }
    }
}

pub struct SessionService<Service, State> {
    inner: Service,
    state: Arc<State>,
}

impl<S, State> SessionService<S, State>
where
    S: Service<Request<Body>> + Clone + Send + Sync + 'static,
    <S as Service<Request<Body>>>::Response: IntoResponse + 'static,
    <S as Service<Request<Body>>>::Error: Into<Infallible> + 'static,
    <S as Service<Request<Body>>>::Future: Send + 'static,
{
    pub fn new(inner: S, state: Arc<State>) -> Self {
        Self { inner, state }
    }
}
impl<S, State, ResBody> Service<Request<Body>> for SessionService<S, State>
where
    S: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + Sync + 'static,
    ResBody: Into<Body> + 'static,
    Response<ResBody>: IntoResponse + 'static,
    <S as Service<Request<Body>>>::Error: Into<Infallible> + 'static,
    <S as Service<Request<Body>>>::Future: Send + 'static,
    State: AsRef<Key> + AsRef<SessionName>,
{
    type Response = Response<Body>;
    type Error = SessionError;
    type Future = SessionFuture<S::Future>;

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
        }
    }
}

pin_project_lite::pin_project! {
    pub struct SessionFuture<Fut> {
        #[pin]
        fut: Fut,
        #[pin]
        authorized: bool,
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

impl<Fut, ResBody, ServiceError> Future for SessionFuture<Fut>
where
    Fut: Future<Output = Result<Response<ResBody>, ServiceError>>,
    ServiceError: Into<Infallible> + 'static,
    ResBody: Into<Body> + 'static,
{
    type Output = Result<Response<Body>, SessionError>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.authorized {
            let resp = Response::builder()
                .status(http::StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap();
            return Poll::Ready(Ok(resp));
        }

        let this = self.project();
        this.fut
            .poll(ctx)
            .map_ok(|r| r.map(Into::into))
            .map_err(|_| unreachable!())
    }
}

// extract できなかったら Unauthorized を返すレイヤー
pub fn build_layer<State>(state: Arc<State>) -> SessionLayer<State> {
    SessionLayer { state }
}
