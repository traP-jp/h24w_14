pub trait Requirements: Send + Sync + 'static {}

impl<T> Requirements for T where T: Send + Sync + 'static {}
