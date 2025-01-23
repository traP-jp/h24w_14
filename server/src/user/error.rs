/// infallible
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::Sqlx(_) => tonic::Status::internal("Database error"),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
