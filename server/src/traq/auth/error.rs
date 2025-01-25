use axum::response::{IntoResponse, Response};

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
            Error::InvalidRequest(msg) => Response::builder()
                .status(400)
                .body(axum::body::Body::from(msg))
                .unwrap_or_else(|_| fallback_error()),
            Error::Sqlx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Sqlx error");
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::from(e.to_string()))
                    .unwrap_or_else(|_| fallback_error())
            }
            Error::Reqwest(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Reqwest error");
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::from(e.to_string()))
                    .unwrap_or_else(|_| fallback_error())
            }
            Error::Other(s) => {
                tracing::error!(error = &s as &dyn std::error::Error, "Unexpected");
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::from(s.message().to_string()))
                    .unwrap_or_else(|_| fallback_error())
            }
            Error::Unknown => {
                tracing::error!("Unknown error");
                Response::builder()
                    .status(500)
                    .body(axum::body::Body::empty())
                    .unwrap_or_else(|_| fallback_error())
            }
        }
    }
}

fn fallback_error() -> Response {
    Response::builder()
        .status(500)
        .body(axum::body::Body::empty())
        .unwrap()
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
