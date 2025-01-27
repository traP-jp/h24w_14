use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State,
    },
    response::{IntoResponse, Response},
    Router,
};
use http::{header, HeaderMap, HeaderName, Method};
use tokio::sync::Notify;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::session::ExtractParams;

pub mod grpc;
pub mod other;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrontendDistDir(pub String);

pub fn make<State>(state: Arc<State>) -> Router<()>
where
    State: grpc::Requirements + other::Requirements,
{
    let grpcs = grpc_routes(state.clone());
    let others = other_routes(state.clone());
    Router::merge(grpcs, others).layer(
        CorsLayer::new()
            .allow_origin(AllowOrigin::mirror_request()) // FIXME
            .allow_methods(AllowMethods::list(vec![
                Method::POST,
                Method::OPTIONS,
                Method::GET,
            ]))
            .allow_headers(AllowHeaders::list(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::COOKIE,
                // header::X_GRPC_WEB,
                HeaderName::from_static("x-grpc-web"),
            ]))
            .allow_credentials(true),
    )
}

fn grpc_routes<State: grpc::Requirements>(state: Arc<State>) -> Router<()> {
    use axum::body::Body as AxumBody;
    use http_body_util::BodyExt;

    macro_rules! services {
        { $( $package:ident ; )+ } => {
            $(
                let $package = tonic_web::enable($crate::$package::build_server(Arc::clone(&state)))
                    .map_request(|r: http::Request<AxumBody>| {
                        r.map(|b| {
                            b.map_err(|e| tonic::Status::from_error(e.into_inner()))
                                .boxed_unsync()
                        })
                    });
            )+
        };
    }

    macro_rules! route_services {
        ($router:expr; [ $( $package:ident ),+ ]) => {
            $router $(
                .route_service(
                    &format!("/{}/{{*rest}}", $crate::$package::SERVICE_NAME),
                    $package,
                )
            )+
        };
    }

    services! { world; user; reaction; message; speaker_phone; }
    let traq_auth = tonic_web::enable(crate::traq::auth::build_server(Arc::clone(&state)))
        .map_request(|r: http::Request<AxumBody>| {
            r.map(|b| {
                b.map_err(|e| tonic::Status::from_error(e.into_inner()))
                    .boxed_unsync()
            })
        });
    let trace_layer = TraceLayer::new_for_grpc();
    let session_layer = crate::session::build_grpc_layer(state);
    route_services!(Router::new(); [ world, user, reaction, message, speaker_phone ])
        .layer(session_layer)
        .route_service(
            &format!("/{}/{{*res}}", crate::traq::auth::SERVICE_NAME),
            traq_auth,
        )
        .layer(trace_layer)
}

fn other_routes<State: other::Requirements>(state: Arc<State>) -> Router<()> {
    use axum::{routing, ServiceExt};

    let bot = crate::traq::bot::tower::build_server::<_, axum::body::Body>(Arc::clone(&state))
        .handle_error::<_, ()>(|e: traq_bot_http::Error| async move {
            tracing::error!(error = &e as &dyn std::error::Error);
            http::StatusCode::INTERNAL_SERVER_ERROR
        });
    let serve_dir: &FrontendDistDir = (*state).as_ref();
    let serve_dir = tower_http::services::ServeDir::new(&serve_dir.0);
    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    Router::new()
        .route("/ping", routing::get(|| async { "pong".to_string() }))
        .route("/oauth2/redirect", routing::get(handle_redirect))
        .route("/ws", routing::get(handle_ws))
        .route_service("/bot", bot)
        .with_state(state)
        .fallback_service(serve_dir)
        .layer(layer)
}

#[tracing::instrument(skip_all)]
async fn handle_redirect<AppState>(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
) -> Response
where
    AppState: other::Requirements,
{
    let req = req.map(|_| ());
    let res = match state.oauth2_handle_redirect(&req).await {
        Ok(user) => user,
        Err(e) => return e.into_response(),
    };

    let user = state
        .find_traq_user(crate::traq::user::FindTraqUserParams { id: res.user_id })
        .await;
    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => {
            tracing::error!(id = %res.user_id.0, "No use found");
            return http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        Err(e) => {
            tracing::error!(error = &e as &dyn std::error::Error);
            return http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let save_params = crate::session::SaveParams {
        header_map: req.headers(),
        user_id: user.inner.id,
    };
    let jar = state.save(save_params).await;
    let jar = match jar {
        Ok(j) => j,
        Err(e) => {
            tracing::error!(error = &e as &dyn std::error::Error);
            return http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let headers: http::HeaderMap = [(http::header::LOCATION, "/".parse().unwrap())]
        .into_iter()
        .collect();
    (http::StatusCode::FOUND, headers, jar).into_response()
}

async fn handle_ws<AppState>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ws: axum::extract::WebSocketUpgrade,
) -> Response
where
    AppState: other::Requirements,
{
    let user_id = state.extract(ExtractParams(&headers)).await;
    let user_id = match user_id {
        Ok(session) => session.user_id,
        Err(e) => {
            tracing::warn!(
                error = &e as &dyn std::error::Error,
                "failed to extract session"
            );
            return http::StatusCode::UNAUTHORIZED.into_response();
        }
    };
    ws.on_upgrade(move |socket| handle_websocket(socket, user_id, state))
}

#[tracing::instrument(skip_all)]
async fn handle_websocket<AppState: other::Requirements>(
    ws: WebSocket,
    user_id: crate::user::UserId,
    state: Arc<AppState>,
) {
    use futures::StreamExt;

    let (ws_tx, ws_rx) = ws.split();
    let (field_tx, field_rx) = tokio::sync::mpsc::channel(2);
    let field_sink = tokio_util::sync::PollSender::new(field_tx);
    let field_stream = tokio_stream::wrappers::ReceiverStream::new(field_rx);
    let events_stream = (*state).explore(crate::explore::ExploreParams {
        id: user_id,
        stream: field_stream.boxed(),
    });
    let close = Arc::new(Notify::new());
    let send = ws_message_to_field(ws_rx, field_sink, Arc::clone(&close));
    let recv = events_to_ws_message(events_stream, ws_tx, close);
    match tokio::join!(send, recv) {
        (Ok(()), Ok(())) => tracing::info!("Finish websocket session cleanly"),
        (Err(e), Ok(())) => tracing::error!(error = ?e, "Sending error"),
        (Ok(()), Err(e)) => tracing::error!(error = ?e, "Receiving error"),
        (Err(se), Err(re)) => {
            tracing::error!(error = ?se, "Sending error");
            tracing::error!(error = ?re, "Receiving error");
        }
    };
}

#[tracing::instrument(skip_all)]
async fn ws_message_to_field<M, F>(
    mut message: M,
    mut field: F,
    close: Arc<Notify>,
) -> anyhow::Result<()>
where
    M: futures::TryStream<Ok = Message> + Send + Unpin,
    M::Error: std::error::Error + Send + Sync + 'static,
    F: futures::Sink<crate::explore::ExplorationField> + Send + Unpin,
    F::Error: std::error::Error + Send + Sync + 'static,
{
    use anyhow::Context;
    use futures::{SinkExt, TryStreamExt};

    while let Some(msg) = message
        .try_next()
        .await
        .context("Failed to receive WebSocket message")?
    {
        match msg {
            Message::Text(text) => {
                let text = text.as_str();
                let f: crate::explore::ExplorationField =
                    serde_json::from_str(text).context("Failed to parse message")?;
                field.send(f).await.context("Failed to send message")?;
            }
            Message::Binary(_) => anyhow::bail!("Received unexpected binary message"),
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Close(_) => break,
        }
    }
    close.notify_one();
    tracing::debug!("Finish");
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn events_to_ws_message<E, M>(
    mut events: E,
    mut message: M,
    close: Arc<Notify>,
) -> anyhow::Result<()>
where
    E: futures::TryStream<Ok = crate::explore::ExplorationFieldEvents> + Send + Unpin,
    E::Error: std::error::Error + Send + Sync + 'static,
    M: futures::Sink<Message> + Send + Unpin,
    M::Error: std::error::Error + Send + Sync + 'static,
{
    use anyhow::Context;
    use futures::{SinkExt, TryStreamExt};

    loop {
        let events = tokio::select! {
            () = close.notified() => break,
            events = events.try_next() => events.context("Failed to receive event")?,
        };
        let Some(events) = events else {
            break;
        };
        let msg_text = serde_json::to_string(&events).context("Failed to serialize JSON")?;
        let msg = Message::text(msg_text);
        message
            .send(msg)
            .await
            .context("Failed to send text message")?;
    }
    message
        .flush()
        .await
        .context("Failed to flush message sink")?;
    message
        .close()
        .await
        .context("Failed to close message sink")?;
    tracing::debug!("Finish");
    Ok(())
}
