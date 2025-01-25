use futures::{future::BoxFuture, stream::BoxStream};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::{message::Message, prelude::IntoStatus, speaker_phone::SpeakerPhone};

pub mod error;
mod r#impl;

pub use error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    Explorer(crate::explore::ExplorerAction),
    SpeakerPhone(SpeakerPhone),
    Message(Message),
    Reaction(crate::reaction::Reaction),
}

#[derive(Debug, Clone)]
pub struct EventChannels {
    message_tx: broadcast::Sender<Message>,
    speaker_phone_tx: broadcast::Sender<SpeakerPhone>,
    event_tx: broadcast::Sender<Event>,
}

pub trait EventService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn subscribe_messages<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'static, Result<Message, Self::Error>>;
    fn subscribe_speaker_phones<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'static, Result<SpeakerPhone, Self::Error>>;
    fn subscribe_events<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'static, Result<Event, Self::Error>>;
    // 以下はおそらく不要なので書かない
    //     subscribe_explorers
    //     subscribe_reactions

    fn publish_event<'a>(
        &'a self,
        ctx: &'a Context,
        event: Event,
    ) -> BoxFuture<'a, Result<(), Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideEventService: Send + Sync + 'static {
    type Context;
    type EventService: EventService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn event_service(&self) -> &Self::EventService;

    fn subscribe_messages(
        &self,
    ) -> BoxStream<
        'static,
        Result<Message, <Self::EventService as EventService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.event_service().subscribe_messages(ctx)
    }
    fn subscribe_speaker_phones(
        &self,
    ) -> BoxStream<
        'static,
        Result<SpeakerPhone, <Self::EventService as EventService<Self::Context>>::Error>,
    > {
        let ctx = self.context();
        self.event_service().subscribe_speaker_phones(ctx)
    }
    fn subscribe_events(
        &self,
    ) -> BoxStream<'static, Result<Event, <Self::EventService as EventService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.event_service().subscribe_events(ctx)
    }

    fn publish_event(
        &self,
        event: Event,
    ) -> BoxFuture<'_, Result<(), <Self::EventService as EventService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.event_service().publish_event(ctx, event)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EventServiceImpl;
