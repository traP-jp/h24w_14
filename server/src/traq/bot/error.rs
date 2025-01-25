use tokio::sync::broadcast;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Bot(#[from] traq_bot_http::Error),
    #[error("Send to failed")]
    BroadcastTx(#[from] broadcast::error::SendError<crate::traq::message::TraqMessage>),
    #[error("Receive from stream failed")]
    BroadcastRx(#[from] BroadcastStreamRecvError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl From<Error> for tonic::Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Bot(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "BOT error");
                tonic::Status::unknown(e.to_string())
            }
            Error::BroadcastTx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Channel error");
                tonic::Status::internal("Channel error")
            }
            Error::BroadcastRx(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Channel error");
                tonic::Status::internal("Channel error")
            }
            Error::Reqwest(e) => {
                tracing::error!(error = &e as &dyn std::error::Error, "Reqwest error");
                tonic::Status::unknown(e.to_string())
            }
        }
    }
}
