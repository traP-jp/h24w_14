use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State,
    },
    response::{IntoResponse, Response},
    Router,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use http::{header, HeaderMap, HeaderName, Method};
use tokio::sync::Notify;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::{
    explore::{ExplorationField, ExplorationFieldEvents},
    prelude::IntoStatus as _,
    session::ExtractParams,
};

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
                axum::extract::ws::Message::Ping(_) | axum::extract::ws::Message::Pong(_) => {
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
            () = notify_close2.notified() => {
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
