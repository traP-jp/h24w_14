use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {}
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {}
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
