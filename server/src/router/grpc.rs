pub trait Requirements:
    crate::world::ProvideWorldService
    + crate::user::ProvideUserService
    + crate::session::ProvideSessionService
    + crate::reaction::ProvideReactionService
{
}

impl<T> Requirements for T where
    T: crate::world::ProvideWorldService
        + crate::user::ProvideUserService
        + crate::session::ProvideSessionService
        + crate::reaction::ProvideReactionService
{
}
