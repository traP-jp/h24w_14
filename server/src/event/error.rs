use tokio::sync::broadcast::error::{RecvError, SendError};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SendMessage(#[from] SendError<super::Message>),
    #[error(transparent)]
    SendSpeakerPhone(#[from] SendError<super::SpeakerPhone>),
    #[error(transparent)]
    SendEvent(#[from] SendError<super::Event>),
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    RecvStream(#[from] BroadcastStreamRecvError),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        tracing::error!(error = &value as &dyn std::error::Error, "channel error");
        tonic::Status::internal("")
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
