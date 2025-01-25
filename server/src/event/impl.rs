use futures::stream::{BoxStream, StreamExt, TryStreamExt};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

impl super::EventChannels {
    pub fn new(capacity: usize) -> Self {
        let (message_tx, _) = broadcast::channel(capacity);
        let (speaker_phone_tx, _) = broadcast::channel(capacity);
        let (event_tx, _) = broadcast::channel(capacity);
        Self {
            message_tx,
            speaker_phone_tx,
            event_tx,
        }
    }
}

impl<Context> super::EventService<Context> for super::EventServiceImpl
where
    Context: AsRef<super::EventChannels>,
{
    type Error = super::Error;

    fn subscribe_messages<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'static, Result<crate::message::Message, Self::Error>> {
        let channels = ctx.as_ref();
        let rx = channels.message_tx.subscribe();
        BroadcastStream::new(rx).map_err(super::Error::from).boxed()
    }

    fn subscribe_speaker_phones<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'static, Result<crate::speaker_phone::SpeakerPhone, Self::Error>> {
        let channels = ctx.as_ref();
        let rx = channels.speaker_phone_tx.subscribe();
        BroadcastStream::new(rx).map_err(super::Error::from).boxed()
    }

    fn subscribe_events<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'static, Result<super::Event, Self::Error>> {
        let channels = ctx.as_ref();
        let rx = channels.event_tx.subscribe();
        BroadcastStream::new(rx).map_err(super::Error::from).boxed()
    }

    fn publish_event<'a>(
        &'a self,
        ctx: &'a Context,
        event: super::Event,
    ) -> futures::future::BoxFuture<'a, Result<(), Self::Error>> {
        use futures::FutureExt;

        let channels = ctx.as_ref();
        publish_event(channels, event).boxed()
    }
}

#[tracing::instrument(skip_all)]
async fn publish_event(
    channels: &super::EventChannels,
    event: super::Event,
) -> super::error::Result<()> {
    match &event {
        super::Event::Message(message) => {
            let subscribers = match channels.message_tx.send(message.clone()) {
                Ok(subscribers) => subscribers,
                Err(broadcast::error::SendError(_)) => {
                    tracing::warn!("Failed to publish message: there may be no subscribers");
                    0
                }
            };
            tracing::trace!(subscribers, "Published message");
        }
        super::Event::SpeakerPhone(speaker_phone) => {
            let subscribers = match channels.speaker_phone_tx.send(speaker_phone.clone()) {
                Ok(subscribers) => subscribers,
                Err(broadcast::error::SendError(_)) => {
                    tracing::warn!("Failed to publish speaker phone: there may be no subscribers");
                    0
                }
            };
            tracing::trace!(subscribers, "Published speaker phone");
        }
        super::Event::Explorer(_) | super::Event::Reaction(_) => {}
    }

    let subscribers = match channels.event_tx.send(event) {
        Ok(subscribers) => subscribers,
        Err(broadcast::error::SendError(_)) => {
            tracing::warn!("Failed to publish event: there may be no subscribers");
            0
        }
    };
    tracing::trace!(subscribers, "Published event");
    Ok(())
}
