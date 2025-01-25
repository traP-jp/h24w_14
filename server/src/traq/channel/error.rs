#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("request send error")]
    RequestSendError,
    #[error("parse error")]
    ParseError,
    #[error("status")]
    Status(#[from] tonic::Status),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        tonic::Status::internal(value.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;