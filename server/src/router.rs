use std::sync::Arc;

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        State,
    },
    response::{IntoResponse, Response},
    Router,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use http::HeaderMap;
use tokio::sync::Notify;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::trace::TraceLayer;

use crate::{
    explore::{ExplorationField, ExplorationFieldEvents},
    prelude::IntoStatus as _,
    session::ExtractParams,
};

pub mod grpc;
pub mod other;

pub fn make<State>(state: Arc<State>) -> Router<()>
where
    State: grpc::Requirements + other::Requirements,
{
    let grpcs = grpc_routes(state.clone());
    let others = other_routes(state.clone());
    Router::merge(grpcs, others)
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

    services! { world; user; reaction; }
    let traq_auth = tonic_web::enable(crate::traq::auth::build_server(Arc::clone(&state)))
        .map_request(|r: http::Request<AxumBody>| {
            r.map(|b| {
                b.map_err(|e| tonic::Status::from_error(e.into_inner()))
                    .boxed_unsync()
            })
        });
    let layer = ServiceBuilder::new()
        .layer(TraceLayer::new_for_grpc())
        .layer(crate::session::build_grpc_layer(state));
    route_services!(Router::new(); [ world, user, reaction ])
        .layer(layer)
        .route_service(
            &format!("/{}/{{*res}}", crate::traq::auth::SERVICE_NAME),
            traq_auth,
        )
}

fn other_routes<State: other::Requirements>(state: Arc<State>) -> Router<()> {
    use axum::routing;

    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    Router::new()
        .route("/ping", routing::get(|| async { "pong".to_string() }))
        .route("/oauth2/redirect", routing::get(handle_redirect))
        .route("/ws", routing::get(handle_ws))
        .with_state(state)
        .layer(layer)
}

async fn handle_redirect<AppState>(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
) -> Response
where
    AppState: other::Requirements,
{
    let res = match state.oauth2_handle_redirect(req.map(|_| ())).await {
        Ok(user) => user,
        Err(e) => return e.into_response(),
    };
    let Ok(res) = serde_json::to_string(&res) else {
        return Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error".into())
            .unwrap();
    };

    Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::new(res))
        .unwrap()
}

async fn handle_ws<AppState>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ws: axum::extract::WebSocketUpgrade,
) -> Response
where
    AppState: other::Requirements,
{
    ws.on_upgrade(|socket| handle_socket(socket, headers, state))
}

async fn handle_socket<AppState>(mut ws: WebSocket, headers: HeaderMap, state: Arc<AppState>)
where
    AppState: other::Requirements,
{
    let user_id = state.extract(ExtractParams(&headers)).await;
    let user_id = match user_id {
        Ok(session) => session.user_id,
        Err(e) => {
            tracing::error!(
                error = &e as &dyn std::error::Error,
                "failed to extract session"
            );
            if let Err(err) = ws.close().await {
                tracing::error!(error = &err as &dyn std::error::Error, "failed to close ws");
            }
            return;
        }
    };

    let (mut ws_tx, mut ws_rx) = ws.split();
    let notify_close = Arc::new(Notify::new());
    let notify_close2 = Arc::clone(&notify_close);

    let ws_rx = Box::pin(async_stream::stream! {
        while let Some(msg) = ws_rx.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    tracing::error!(error = &e as &dyn std::error::Error, "failed to receive ws message");
                    return;
                }
            };

            match msg {
                axum::extract::ws::Message::Text(msg) => {
                    let msg = match serde_json::from_str::<ExplorationField>(&msg) {
                        Ok(msg) => msg,
                        Err(e) => {
                            tracing::error!(error = &e as &dyn std::error::Error, "failed to parse ws message");
                            notify_close.notify_one();
                            return;
                        }
                    };
                    yield msg;
                }
                axum::extract::ws::Message::Binary(msg) => {
                    tracing::error!("unexpected binary message {:?}", msg);
                    return;
                }
                axum::extract::ws::Message::Close(_) => {
                    notify_close.notify_one();
                    return;
                }
                _ => {
                    // NOTE: Ping/Pong は勝手に処理してくれる
                }
            }
        }
    });

    let mut event_rx = state.explore(crate::explore::ExploreParams {
        id: user_id,
        stream: ws_rx,
    });

    loop {
        tokio::select! {
            _ = notify_close2.notified() => {
                if let Err(e) = ws_tx.close().await {
                    tracing::error!(error = &e as &dyn std::error::Error, "failed to close ws");
                }
            }
            Some(event) = event_rx.next() => {
                handle_explore_event(event.map_err(
                    |e| e.into_status()
                ), &mut ws_tx, Arc::clone(&notify_close2)).await;
            }
            else => {
                notify_close2.notify_one();
                return;
            }
        }
    }
}

async fn handle_explore_event(
    event: Result<ExplorationFieldEvents, tonic::Status>,
    ws_tx: &mut SplitSink<WebSocket, Message>,
    notify_close2: Arc<Notify>,
) {
    let event = match event {
        Ok(event) => event,
        Err(e) => {
            tracing::error!(
                error = &e as &dyn std::error::Error,
                "failed to receive event"
            );
            notify_close2.notify_one();
            return;
        }
    };

    if let Err(e) = ws_tx
        .send(axum::extract::ws::Message::Text(
            serde_json::to_string(&event)
                .expect("failed to serialize event")
                .into(),
        ))
        .await
    {
        tracing::error!(
            error = &e as &dyn std::error::Error,
            "failed to send ws message"
        );
        notify_close2.notify_one();
    }
}
