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
        match value {
            Error::RequestSendError => tonic::Status::internal("request sending error"),
            Error::ParseError => tonic::Status::internal("parsing error"),
            Error::Status(status) => status,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
