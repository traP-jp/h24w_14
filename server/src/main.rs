use anyhow::Context;
use tokio::net::TcpListener;

use h24w14 as lib;

struct State;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let state = std::sync::Arc::new(State);

    let router = lib::router::make(state);
    let tcp_listener = load_tcp_listener().await?;
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown())
        .await?;

    Ok(())
}

#[tracing::instrument]
async fn load_tcp_listener() -> anyhow::Result<TcpListener> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| 8000.to_string())
        .parse()
        .context("Failed to parse PORT value")?;
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind {addr}"))?;
    tracing::info!(%addr, "Listening");
    Ok(listener)
}

#[tracing::instrument]
async fn shutdown() {
    let ctrl_c = tokio::signal::ctrl_c();
    match ctrl_c.await {
        Ok(()) => tracing::info!("Received ctrl-c"),
        Err(e) => tracing::error!(
            error = &e as &dyn std::error::Error,
            "Failed to watch ctrl-c"
        ),
    }
}
