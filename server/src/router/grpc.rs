pub trait Requirements:
    crate::world::ProvideWorldService
    + crate::traq::auth::ProvideTraqAuthService
    + crate::user::ProvideUserService
    + crate::session::ProvideSessionService
    + crate::reaction::ProvideReactionService
{
}

impl<T> Requirements for T where
    T: crate::world::ProvideWorldService
        + crate::traq::auth::ProvideTraqAuthService
        + crate::user::ProvideUserService
        + crate::session::ProvideSessionService
        + crate::reaction::ProvideReactionService
{
}
