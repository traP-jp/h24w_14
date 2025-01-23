use futures::{future::BoxFuture, stream::BoxStream};
use serde::{Deserialize, Serialize};

use crate::{message::Message, prelude::IntoStatus};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    Explorer(crate::explore::ExplorerAction),
    SpkeakerPhone(crate::speaker_phone::SpeakerPhone),
    Message(Message),
    Reaction(crate::reaction::Reaction),
}

pub trait EventService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn subscribe_messages<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'a, Result<Message, Self::Error>>;
    fn subscribe_events<'a>(
        &'a self,
        ctx: &'a Context,
    ) -> BoxStream<'a, Result<Event, Self::Error>>;
    // 以下はおそらく不要なので書かない
    //     subscribe_explorers
    //     subscribe_speaker_phones
    //     subscribe_reactions

    fn publish_event<'a>(
        &'a self,
        ctx: &'a Context,
        req: Event,
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
    ) -> BoxStream<'_, Result<Message, <Self::EventService as EventService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.event_service().subscribe_messages(ctx)
    }
    fn subscribe_events(
        &self,
    ) -> BoxStream<'_, Result<Event, <Self::EventService as EventService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.event_service().subscribe_events(ctx)
    }

    fn publish_event(
        &self,
        req: Event,
    ) -> BoxFuture<'_, Result<(), <Self::EventService as EventService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.event_service().publish_event(ctx, req)
    }
}
