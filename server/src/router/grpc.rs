pub trait Requirements: crate::world::ProvideWorldService {}

impl<T> Requirements for T where T: crate::world::ProvideWorldService {}
