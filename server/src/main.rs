use std::sync::Arc;

use anyhow::Context;
use sqlx::MySqlPool;

use h24w14 as lib;

#[derive(Debug, Clone)]
struct State {
    pool: MySqlPool,
    task_manager: lib::task::TaskManager,
    world_size: lib::world::WorldSize,
    services: Services,
}

#[derive(Debug, Clone, Copy, Default)]
struct Services {
    world_service: lib::world::WorldServiceImpl,
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
    let world_size = load::world_size()?;
    let state = Arc::new(State {
        pool,
        task_manager,
        world_size,
        services: Services::default(),
    });
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

    macro_rules! env_var {
        ($name:expr) => {
            std::env::var($name).with_context(|| format!("Failed to read {}", $name))
        };
    }

    #[tracing::instrument]
    pub async fn mysql(env_prefix: &str) -> anyhow::Result<MySqlPool> {
        macro_rules! var {
            ($n:ident) => {{
                let var_name = format!(concat!("{}", stringify!($n)), env_prefix);
                env_var!(&var_name)
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

    pub fn world_size() -> anyhow::Result<lib::world::WorldSize> {
        let width = env_var!("WORLD_WIDTH")?
            .parse()
            .context("Failed to parse WORLD_WIDTH as u32")?;
        let height = env_var!("WORLD_HEIGHT")?
            .parse()
            .context("Failed to parse WORLD_HEIGHT as u32")?;
        let size = lib::world::Size { width, height };
        Ok(lib::world::WorldSize(size))
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

impl AsRef<lib::world::WorldSize> for State {
    fn as_ref(&self) -> &lib::world::WorldSize {
        &self.world_size
    }
}

impl lib::world::ProvideWorldService for State {
    type Context = Self;
    type WorldService = lib::world::WorldServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn world_service(&self) -> &Self::WorldService {
        &self.services.world_service
    }
}
