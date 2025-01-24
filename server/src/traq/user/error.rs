#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::Sqlx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Database error");
                tonic::Status::internal("database error")
            }
            Error::Status(s) => {
                tracing::error!(error = &s as &dyn std::error::Error, "Unexpected");
                s
            }
        }
    }
}
