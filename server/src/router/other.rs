pub trait Requirements:
    crate::traq::auth::ProvideTraqAuthService
    + crate::explore::ProvideExploreService
    + crate::session::ProvideSessionService
    + crate::traq::user::ProvideTraqUserService
    + crate::traq::bot::ProvideTraqBotService
    + AsRef<crate::traq::bot::TraqBotConfig>
    + AsRef<super::FrontendDistDir>
{
}

impl<T> Requirements for T where
    T: crate::traq::auth::ProvideTraqAuthService
        + crate::explore::ProvideExploreService
        + crate::session::ProvideSessionService
        + crate::traq::user::ProvideTraqUserService
        + crate::traq::bot::ProvideTraqBotService
        + AsRef<crate::traq::bot::TraqBotConfig>
        + AsRef<super::FrontendDistDir>
{
}
