use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Other(#[from] tonic::Status),
    #[error("Unknown error")]
    Unknown,
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidRequest(msg) => tonic::Status::invalid_argument(msg),
            Error::Sqlx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Sqlx error");
                tonic::Status::internal(e.to_string())
            }
            Error::Reqwest(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Reqwest error");
                tonic::Status::unknown(e.to_string())
            }
            Error::Other(s) => {
                tracing::error!(error = &s as &dyn std::error::Error, "Unexpected");
                s
            }
            Error::Unknown => {
                tracing::error!("Unknown error");
                tonic::Status::unknown("Unknown error")
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            Error::Sqlx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Sqlx error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Error::Reqwest(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Reqwest error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Error::Other(s) => {
                tracing::error!(error = &s as &dyn std::error::Error, "Unexpected");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Error::Unknown => {
                tracing::error!("Unknown error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
