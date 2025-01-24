use std::sync::Arc;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

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
    let world_service = crate::world::ProvideWorldService::build_server(state);
    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_grpc());
    // TODO: tonic_web::enable
    Router::new().layer(layer).route_service(
        &format!("/{}/{{*rest}}", crate::world::SERVICE_NAME),
        world_service,
    )
}

fn other_routes<State: other::Requirements>(state: Arc<State>) -> Router<()> {
    use axum::routing;

    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    Router::new()
        .route("/ping", routing::get(|| async { "pong".to_string() }))
        .with_state(state)
        .layer(layer)
}
