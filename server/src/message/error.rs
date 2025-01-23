#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    NotFound,
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not found"),
            Error::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => tonic::Status::not_found("Not found"),
            Error::Unknown => tonic::Status::unknown("Unknown"),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
