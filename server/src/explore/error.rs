
// todo: 仮置き
/// infallible
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("exploration field stream closed")]
    ExplorationFieldStreamClosed,
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        // TODO: match書く
        tonic::Status::internal(value.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
