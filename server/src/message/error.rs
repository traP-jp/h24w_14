#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {}
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
