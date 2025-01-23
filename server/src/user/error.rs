/// infallible
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("not found")]
    NotFound,
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("not found"),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
