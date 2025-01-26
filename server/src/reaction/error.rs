#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Reaction not in world")]
    ReactionNotInWorld,
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::ReactionNotInWorld => tonic::Status::not_found("Reaction not in world"),
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
