use std::sync::Arc;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn make<State: Send + Sync + 'static>(state: Arc<State>) -> Router<()> {
    let grpcs = grpc_routes(state.clone());
    let others = other_routes(state.clone());
    Router::merge(grpcs, others)
}

fn grpc_routes<State: Send + Sync + 'static>(_state: Arc<State>) -> Router<()> {
    // TODO
    Router::new()
}

fn other_routes<State: Send + Sync + 'static>(state: Arc<State>) -> Router<()> {
    use axum::routing;

    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    Router::new()
        .route("/ping", routing::get(|| async { "pong".to_string() }))
        .with_state(state)
        .layer(layer)
}
