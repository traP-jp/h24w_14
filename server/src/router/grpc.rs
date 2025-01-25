pub trait Requirements:
    crate::world::ProvideWorldService
    + crate::traq::auth::ProvideTraqAuthService
    + crate::user::ProvideUserService
    + crate::session::ProvideSessionService
    + crate::reaction::ProvideReactionService
    + crate::message::ProvideMessageService
    + crate::speaker_phone::ProvideSpeakerPhoneService
{
}

impl<T> Requirements for T where
    T: crate::world::ProvideWorldService
        + crate::traq::auth::ProvideTraqAuthService
        + crate::user::ProvideUserService
        + crate::session::ProvideSessionService
        + crate::reaction::ProvideReactionService
        + crate::message::ProvideMessageService
        + crate::speaker_phone::ProvideSpeakerPhoneService
{
}
