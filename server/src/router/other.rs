pub trait Requirements: crate::traq::auth::ProvideTraqAuthService {}

impl<T> Requirements for T where T: crate::traq::auth::ProvideTraqAuthService {}
