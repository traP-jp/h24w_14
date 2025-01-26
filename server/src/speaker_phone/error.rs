#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Bad position")]
    BadPositionProvided,
    #[error("Bad channel name")]
    BadChannelProvided,
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::BadPositionProvided => tonic::Status::not_found("Bad position"),
            Error::BadChannelProvided => tonic::Status::invalid_argument("Bad channel name"),
            Error::Sqlx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error);
                tonic::Status::internal("Database error")
            }
            Error::Status(e) => {
                tracing::error!(error = &e as &dyn std::error::Error);
                tonic::Status::internal("Status error")
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
