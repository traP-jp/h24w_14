#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Received unexpected response from traQ")]
    UnexpectedResponseFromTraq,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Unauthorized => tonic::Status::unauthenticated("Unauthorized"),
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::UnexpectedResponseFromTraq => {
                tracing::warn!("Received unexpected response from traQ");
                tonic::Status::unknown("Received unexpected response from traQ")
            }
            Error::Sqlx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Database error");
                tonic::Status::internal("database error")
            }
            Error::Reqwest(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "HTTP request error");
                tonic::Status::unknown("HTTP request error")
            }
            Error::Status(s) => {
                tracing::error!(error = &s as &dyn std::error::Error, "Unexpected");
                s
            }
        }
    }
}
