pub trait Requirements:
    crate::traq::auth::ProvideTraqAuthService
    + crate::explore::ProvideExploreService
    + crate::session::ProvideSessionService
{
}

impl<T> Requirements for T where
    T: crate::traq::auth::ProvideTraqAuthService
        + crate::explore::ProvideExploreService
        + crate::session::ProvideSessionService
{
}
