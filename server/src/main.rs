use std::sync::Arc;

use anyhow::Context;
use sqlx::MySqlPool;

use h24w14 as lib;

#[derive(Debug, Clone)]
struct State {
    pool: MySqlPool,
    task_manager: lib::task::TaskManager,
    world_size: lib::world::WorldSize,
    event_channels: lib::event::EventChannels,
    client: reqwest::Client,
    session_config: SessionConfig,
    services: Services,
}

#[derive(Debug, Clone, Copy, Default)]
struct Services {
    world_service: lib::world::WorldServiceImpl,
    event_service: lib::event::EventServiceImpl,
    user_service: lib::user::UserServiceImpl,
    session_service: lib::session::SessionServiceImpl,
    reaction_service: lib::reaction::ReactionServiceImpl,
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
    let event_channels = load::event_channels()?;
    let client = reqwest::Client::new();
    let session_config = load::session_config()?;
    let state = Arc::new(State {
        pool,
        task_manager,
        world_size,
        event_channels,
        client,
        session_config,
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

// MARK: helper `fn`s, `struct`s

#[derive(Debug, Clone)]
struct SessionConfig {
    key: axum_extra::extract::cookie::Key,
    name: lib::session::SessionName,
    domain: lib::session::CookieDomain,
}

mod load {
    use anyhow::Ok;
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

    pub fn event_channels() -> anyhow::Result<lib::event::EventChannels> {
        let capacity = env_var!("EVENT_CHANNELS_CAPACITY")?
            .parse()
            .context("Failed to parse EVENT_CHANNELS_CAPACITY")?;
        Ok(lib::event::EventChannels::new(capacity))
    }

    #[tracing::instrument]
    pub fn session_config() -> anyhow::Result<SessionConfig> {
        use axum_extra::extract::cookie::Key as SessionKey;

        let key = env_var!("SESSION_KEY")
            .map(|k| {
                let k = hex::decode(&k).context("Failed to decode SESSION_KEY value as hex")?;
                SessionKey::try_from(&*k).context("Failed to load SESSION_KEY")
            })
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = e.as_ref() as &dyn std::error::Error,
                    "Generating session key"
                );
                SessionKey::try_generate().context("Could not generate session key")
            })?;
        let name = env_var!("SESSION_NAME")?;
        let domain = env_var!("COOKIE_ATTR_DOMAIN")?;
        Ok(SessionConfig {
            key,
            name: lib::session::SessionName(name),
            domain: lib::session::CookieDomain(domain),
        })
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
        tokio::time::timeout(duration, fut).await?;
        Ok(())
    }
}

impl AsRef<lib::world::WorldSize> for State {
    fn as_ref(&self) -> &lib::world::WorldSize {
        &self.world_size
    }
}

impl AsRef<lib::event::EventChannels> for State {
    fn as_ref(&self) -> &lib::event::EventChannels {
        &self.event_channels
    }
}

impl AsRef<reqwest::Client> for State {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
    }
}

impl AsRef<axum_extra::extract::cookie::Key> for State {
    fn as_ref(&self) -> &axum_extra::extract::cookie::Key {
        &self.session_config.key
    }
}

impl AsRef<lib::session::SessionName> for State {
    fn as_ref(&self) -> &lib::session::SessionName {
        &self.session_config.name
    }
}

impl AsRef<lib::session::CookieDomain> for State {
    fn as_ref(&self) -> &lib::session::CookieDomain {
        &self.session_config.domain
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

impl lib::event::ProvideEventService for State {
    type Context = Self;
    type EventService = lib::event::EventServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn event_service(&self) -> &Self::EventService {
        &self.services.event_service
    }
}

impl lib::user::ProvideUserService for State {
    type Context = Self;
    type UserService = lib::user::UserServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn user_service(&self) -> &Self::UserService {
        &self.services.user_service
    }
}

impl lib::session::ProvideSessionService for State {
    type Context = Self;
    type SessionService = lib::session::SessionServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn session_service(&self) -> &Self::SessionService {
        &self.services.session_service
    }
}

impl lib::reaction::ProvideReactionService for State {
    type Context = Self;
    type ReactionService = lib::reaction::ReactionServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn reaction_service(&self) -> &Self::ReactionService {
        &self.services.reaction_service
    }
}
