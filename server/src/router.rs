use std::sync::Arc;

use axum::Router;
use tower::{ServiceBuilder, ServiceExt};
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

    services! { world; }
    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_grpc());
    // TODO: tonic_web::enable
    route_services!(Router::new().layer(layer); [ world ])
}

fn other_routes<State: other::Requirements>(state: Arc<State>) -> Router<()> {
    use axum::routing;

    let layer = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    Router::new()
        .route("/ping", routing::get(|| async { "pong".to_string() }))
        .with_state(state)
        .layer(layer)
}
