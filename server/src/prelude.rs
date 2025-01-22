use std::error::Error as StdError;

use serde::{Deserialize, Serialize};

// MARK: IntoStatus

/// Represents a error type which implements `Into<tonic::Status>`.
/// This trait is **NOT** [dyn-compatible].
///
/// Do not try to `impl` this trait manually;
/// types implementing [`Error`] and `Into<tonic::Status>` will implement this trait automatically.
///
/// ```
/// use std::fmt;
///
/// #[derive(Debug, Clone)]
/// struct NotFoundError {
///     pub id: String,
/// }
///
/// impl fmt::Display for NotFoundError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "no entry found with id {}", self.id)
///     }
/// }
///
/// impl std::error::Error for NotFoundError {}
///
/// impl From<NotFoundError> for tonic::Status {
///     fn from(value: NotFoundError) -> tonic::Status {
///         tonic::Status::not_found(value.to_string())
///     }
/// }
///
/// # fn main() {
/// # use h24w14::prelude::IntoStatus;
/// let error = NotFoundError { id: "some_id".to_string() };
/// let status = error.into_status();
/// assert_eq!(status.code(), tonic::Code::NotFound);
/// # }
/// ```
///
/// [dyn-compatible]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
/// [`Error`]: std::error::Error
pub trait IntoStatus: Into<tonic::Status> + StdError + Sized + Send + Sync + 'static {
    fn into_status(self) -> tonic::Status {
        self.into()
    }
}

impl<E> IntoStatus for E where E: Into<tonic::Status> + StdError + Sized + Send + Sync + 'static {}

// MARK: Timestamp

/// A bridge type between [`chrono::DateTime<chrono::Utc>`] and [`prost_types::Timestamp`].
///
/// ```
/// # use h24w14::prelude::Timestamp;
/// let timestamp: Timestamp = chrono::Utc::now().into();
/// let message: prost_types::Timestamp = timestamp.into();
/// # let _ = message;
/// ```
///
/// [`chrono::DateTime<chrono::Utc>`]: chrono::DateTime
/// [`prost_types::Timestamp`]: prost_types::Timestamp
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Timestamp(pub chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl From<prost_types::Timestamp> for Timestamp {
    fn from(value: prost_types::Timestamp) -> Self {
        let prost_types::Timestamp { seconds, nanos } = value;
        let nanos = u32::try_from(nanos).unwrap();
        chrono::DateTime::from_timestamp(seconds, nanos)
            .expect("Invalid timestamp")
            .into()
    }
}

impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl From<Timestamp> for prost_types::Timestamp {
    fn from(value: Timestamp) -> Self {
        let seconds = value.0.timestamp();
        let nanos = value.0.timestamp_subsec_nanos();
        let nanos = i32::try_from(nanos).unwrap();
        prost_types::Timestamp { seconds, nanos }
    }
}
