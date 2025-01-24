#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unauthorized => tonic::Status::unauthenticated("Unauthorized"),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
