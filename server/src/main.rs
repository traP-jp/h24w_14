use std::sync::Arc;

use anyhow::Context;
use sqlx::MySqlPool;

use h24w14 as lib;

#[derive(Debug, Clone)]
struct State {
    pool: MySqlPool,
    task_manager: lib::task::TaskManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use futures::TryFutureExt;
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let pool = load::mysql("MYSQL_")
        .or_else(|_| load::mysql("MARIADB_"))
        .or_else(|_| load::mysql("NS_MARIADB_"))
        .await?;
    let task_manager = lib::task::TaskManager::new();
    let state = Arc::new(State { pool, task_manager });
    state.migrate().await?;

    let router = lib::router::make(Arc::clone(&state));
    let tcp_listener = load::tcp_listener().await?;
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(shutdown())
        .await?;
    state.graceful_shutdown().await?;

    Ok(())
}

// MARK: helper `fn`s

mod load {
    use tokio::net::TcpListener;

    use super::*;

    #[tracing::instrument]
    pub async fn mysql(env_prefix: &str) -> anyhow::Result<MySqlPool> {
        macro_rules! var {
            ($n:ident) => {{
                let var_name = format!(concat!("{}", stringify!($n)), env_prefix);
                std::env::var(&var_name).with_context(|| format!("Failed to read {var_name}"))
            }};
        }

        let hostname = var!(HOSTNAME)?;
        let user = var!(USER)?;
        let password = var!(PASSWORD)?;
        let port: u16 = var!(PORT)?.parse().context("Failed to read PORT value")?;
        let database = var!(DATABASE)?;
        let options = sqlx::mysql::MySqlConnectOptions::new()
            .host(&hostname)
            .username(&user)
            .password(&password)
            .port(port)
            .database(&database);
        sqlx::MySqlPool::connect_with(options)
            .await
            .inspect_err(|e| {
                tracing::error!(
                    error = e as &dyn std::error::Error,
                    "Failed to connect database"
                )
            })
            .context("Failed to connect to MySQL")
    }

    #[tracing::instrument]
    pub async fn tcp_listener() -> anyhow::Result<TcpListener> {
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

// MARK: State `impl`s

impl AsRef<MySqlPool> for State {
    fn as_ref(&self) -> &MySqlPool {
        &self.pool
    }
}

impl State {
    const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

    async fn migrate(&self) -> anyhow::Result<()> {
        Self::MIGRATOR
            .run(&self.pool)
            .await
            .context("Migration failed")?;
        Ok(())
    }

    async fn graceful_shutdown(&self) -> anyhow::Result<()> {
        let duration = std::time::Duration::from_secs(5);
        let fut = self.task_manager.graceful_shutdown();
        tokio::time::timeout(duration, fut).await??;
        Ok(())
    }
}
