#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No explorer found")]
    NotFound,
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::Status(s) => {
                tracing::error!(error = &s as &dyn std::error::Error);
                s
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
