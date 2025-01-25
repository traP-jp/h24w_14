pub trait Requirements:
    crate::traq::auth::ProvideTraqAuthService
    + crate::explore::ProvideExploreService
    + crate::session::ProvideSessionService
    + crate::traq::user::ProvideTraqUserService
{
}

impl<T> Requirements for T where
    T: crate::traq::auth::ProvideTraqAuthService
        + crate::explore::ProvideExploreService
        + crate::session::ProvideSessionService
        + crate::traq::user::ProvideTraqUserService
{
}
