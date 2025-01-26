use std::sync::Arc;

use anyhow::Context;
use sqlx::MySqlPool;

use h24w14::{self as lib, traq::auth::TraqOauthClientConfig};

#[derive(Debug, Clone)]
struct State {
    pool: MySqlPool,
    task_manager: lib::task::TaskManager,
    world_size: lib::world::WorldSize,
    event_channels: lib::event::EventChannels,
    client: reqwest::Client,
    session_config: SessionConfig,
    explorer_store: lib::explore::ExplorerStore,
    services: Services,
    traq_oauth_client_config: TraqOauthClientConfig,
    traq_host: lib::traq::TraqHost,
    traq_bot_config: lib::traq::bot::TraqBotConfig,
    traq_bot_channels: lib::traq::bot::TraqBotChannels,
    traq_channels_cache: lib::traq::channel::TraqChannelsCache,
    frontend_dist_dir: lib::router::FrontendDistDir,
}

#[derive(Debug, Clone, Copy, Default)]
struct Services {
    world_service: lib::world::WorldServiceImpl,
    event_service: lib::event::EventServiceImpl,
    user_service: lib::user::UserServiceImpl,
    session_service: lib::session::SessionServiceImpl,
    message_service: lib::message::MessageServiceImpl,
    reaction_service: lib::reaction::ReactionServiceImpl,
    speaker_phone_service: lib::speaker_phone::SpeakerPhoneServiceImpl,
    explore_service: lib::explore::ExploreServiceImpl,
    explorer_service: lib::explore::ExplorerServiceImpl,
    traq_user_service: lib::traq::user::TraqUserServiceImpl,
    traq_auth_service: lib::traq::auth::TraqAuthServiceImpl,
    traq_bot_service: lib::traq::bot::TraqBotServiceImpl,
    traq_channel_service: lib::traq::channel::TraqChannelServiceImpl,
    traq_message_service: lib::traq::message::TraqMessageServiceImpl,
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
    let traq_oauth_client_config = load::traq_oauth_client_config()?;
    let traq_host = load::traq_host()?;
    let traq_bot_config = load::traq_bot_config()?;
    let frontend_dist_dir = load::frontend_dist_dir()?;
    let state = Arc::new(State {
        pool,
        task_manager,
        world_size,
        event_channels,
        client,
        session_config,
        explorer_store: lib::explore::ExplorerStore::new(),
        services: Services::default(),
        traq_oauth_client_config,
        traq_host,
        traq_bot_config,
        traq_bot_channels: lib::traq::bot::TraqBotChannels::default(),
        traq_channels_cache: Default::default(),
        frontend_dist_dir,
    });
    state.migrate().await?;
    Arc::clone(&state).load().await?;

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

    pub fn traq_oauth_client_config() -> anyhow::Result<lib::traq::auth::TraqOauthClientConfig> {
        let client_id = env_var!("TRAQ_OAUTH_CLIENT_ID")?;
        let client_secret = env_var!("TRAQ_OAUTH_CLIENT_SECRET")?;
        Ok(lib::traq::auth::TraqOauthClientConfig {
            client_id,
            client_secret,
        })
    }

    pub fn traq_host() -> anyhow::Result<lib::traq::TraqHost> {
        let traq_host = env_var!("TRAQ_HOST")?;
        Ok(lib::traq::TraqHost(traq_host))
    }

    pub fn traq_bot_config() -> anyhow::Result<lib::traq::bot::TraqBotConfig> {
        let config = lib::traq::bot::TraqBotConfig::builder()
            .bot_id(env_var!("TRAQ_BOT_ID")?)
            .bot_user_id(env_var!("TRAQ_BOT_USER_ID")?)
            .access_token(env_var!("TRAQ_BOT_ACCESS_TOKEN")?)
            .verification_token(env_var!("TRAQ_BOT_VERIFICATION_TOKEN")?)
            .build();
        Ok(config)
    }

    pub fn frontend_dist_dir() -> anyhow::Result<lib::router::FrontendDistDir> {
        let v = env_var!("FRONTEND_DIST_DIR")?;
        Ok(lib::router::FrontendDistDir(v))
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

    #[tracing::instrument(skip_all)]
    async fn load(self: Arc<Self>) -> anyhow::Result<()> {
        use lib::speaker_phone::{LoadAllSpeakerPhonesParams, SpeakerPhoneService};

        self.services
            .speaker_phone_service
            .load_all_speaker_phones(Arc::clone(&self), LoadAllSpeakerPhonesParams {})
            .await?;
        Ok(())
    }
}

impl AsRef<lib::task::TaskManager> for State {
    fn as_ref(&self) -> &lib::task::TaskManager {
        &self.task_manager
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

impl AsRef<lib::explore::ExplorerStore> for State {
    fn as_ref(&self) -> &lib::explore::ExplorerStore {
        &self.explorer_store
    }
}

impl AsRef<TraqOauthClientConfig> for State {
    fn as_ref(&self) -> &TraqOauthClientConfig {
        &self.traq_oauth_client_config
    }
}

impl AsRef<lib::traq::TraqHost> for State {
    fn as_ref(&self) -> &lib::traq::TraqHost {
        &self.traq_host
    }
}

impl AsRef<lib::traq::bot::TraqBotConfig> for State {
    fn as_ref(&self) -> &lib::traq::bot::TraqBotConfig {
        &self.traq_bot_config
    }
}

impl AsRef<lib::traq::bot::TraqBotChannels> for State {
    fn as_ref(&self) -> &lib::traq::bot::TraqBotChannels {
        &self.traq_bot_channels
    }
}

impl AsRef<lib::router::FrontendDistDir> for State {
    fn as_ref(&self) -> &lib::router::FrontendDistDir {
        &self.frontend_dist_dir
    }
}

impl AsRef<lib::traq::channel::TraqChannelsCache> for State {
    fn as_ref(&self) -> &lib::traq::channel::TraqChannelsCache {
        &self.traq_channels_cache
    }
}

// MARK: impl ProvideHogeService

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

impl lib::message::ProvideMessageService for State {
    type Context = Self;
    type MessageService = lib::message::MessageServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn message_service(&self) -> &Self::MessageService {
        &self.services.message_service
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

impl lib::speaker_phone::ProvideSpeakerPhoneService for State {
    type Context = Self;
    type SpeakerPhoneService = lib::speaker_phone::SpeakerPhoneServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn speaker_phone_service(&self) -> &Self::SpeakerPhoneService {
        &self.services.speaker_phone_service
    }
}

impl lib::explore::ProvideExploreService for State {
    type Context = Self;
    type ExploreService = lib::explore::ExploreServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn explore_service(&self) -> &Self::ExploreService {
        &self.services.explore_service
    }
}

impl lib::explore::ProvideExplorerService for State {
    type Context = Self;
    type ExplorerService = lib::explore::ExplorerServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn explorer_service(&self) -> &Self::ExplorerService {
        &self.services.explorer_service
    }
}

impl lib::traq::user::ProvideTraqUserService for State {
    type Context = Self;
    type TraqUserService = lib::traq::user::TraqUserServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }

    fn traq_user_service(&self) -> &Self::TraqUserService {
        &self.services.traq_user_service
    }
}

impl lib::traq::auth::ProvideTraqAuthService for State {
    type Context = Self;
    type TraqAuthService = lib::traq::auth::TraqAuthServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn traq_auth_service(&self) -> &Self::TraqAuthService {
        &self.services.traq_auth_service
    }
}

// channel message

impl lib::traq::bot::ProvideTraqBotService for State {
    type Context = Self;
    type TraqBotService = lib::traq::bot::TraqBotServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn traq_bot_service(&self) -> &Self::TraqBotService {
        &self.services.traq_bot_service
    }
}

impl lib::traq::channel::ProvideTraqChannelService for State {
    type Context = Self;
    type TraqChannelService = lib::traq::channel::TraqChannelServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn traq_channel_service(&self) -> &Self::TraqChannelService {
        &self.services.traq_channel_service
    }
}

impl lib::traq::message::ProvideTraqMessageService for State {
    type Context = Self;
    type TraqMessageService = lib::traq::message::TraqMessageServiceImpl;

    fn context(&self) -> &Self::Context {
        self
    }
    fn traq_message_service(&self) -> &Self::TraqMessageService {
        &self.services.traq_message_service
    }
}
