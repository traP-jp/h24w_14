use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::ControlFlow;
use std::sync::Arc;

use futures::future::{BoxFuture, FutureExt};
use tokio::sync::RwLock;

use crate::event::{Event, ProvideEventService};
use crate::prelude::IntoStatus;

// MARK: ExplorerService

impl super::ExplorerStore {
    pub fn new() -> Self {
        Self {
            explorers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for super::ExplorerStore {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for super::ExplorerStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExplorerStore")
            .field("explorers", &"...")
            .finish()
    }
}

impl<Context> super::ExplorerService<Context> for super::ExplorerServiceImpl
where
    Context: ProvideEventService + AsRef<super::ExplorerStore>,
{
    type Error = super::Error;

    fn get_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetExplorerParams,
    ) -> BoxFuture<'a, Result<super::Explorer, Self::Error>> {
        get_explorer(ctx.as_ref(), params).boxed()
    }

    fn create_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CreateExplorerParams,
    ) -> BoxFuture<'a, Result<super::Explorer, Self::Error>> {
        create_explorer(ctx, ctx.as_ref(), params).boxed()
    }

    fn get_explorers_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetExplorersInAreaParams,
    ) -> BoxFuture<'a, Result<Vec<super::Explorer>, Self::Error>> {
        get_explorers_in_area(ctx.as_ref(), params).boxed()
    }

    fn update_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::UpdateExplorerParams,
    ) -> BoxFuture<'a, Result<super::Explorer, Self::Error>> {
        update_explorer(ctx, ctx.as_ref(), params).boxed()
    }

    fn delete_explorer<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::DeleteExplorerParams,
    ) -> BoxFuture<'a, Result<super::Explorer, Self::Error>> {
        delete_explorer(ctx, ctx.as_ref(), params).boxed()
    }
}

async fn get_explorer(
    store: &super::ExplorerStore,
    params: super::GetExplorerParams,
) -> Result<super::Explorer, super::Error> {
    let super::GetExplorerParams { id } = params;
    let explorers = store.explorers.read().await;
    explorers.get(&id).ok_or(super::Error::NotFound).cloned()
}

async fn create_explorer<E: ProvideEventService>(
    event_service: &E,
    store: &super::ExplorerStore,
    params: super::CreateExplorerParams,
) -> Result<super::Explorer, super::Error> {
    let id = super::ExplorerId(uuid::Uuid::now_v7());
    let super::CreateExplorerParams { inner, position } = params;
    let explorer = super::Explorer {
        id,
        inner,
        position,
    };
    store.explorers.write().await.insert(id, explorer.clone());
    let event = Event::Explorer(super::ExplorerAction::Arrive(explorer.clone()));
    event_service
        .publish_event(event)
        .await
        .map_err(IntoStatus::into_status)?;
    Ok(explorer)
}

// FIXME
async fn get_explorers_in_area(
    store: &super::ExplorerStore,
    params: super::GetExplorersInAreaParams,
) -> Result<Vec<super::Explorer>, super::Error> {
    let f = match params {
        super::GetExplorersInAreaParams::Rect { center, size } => {
            let x_min = center.x.saturating_sub(size.width >> 1);
            let x_max = center.x.saturating_add(size.width >> 1);
            let y_min = center.y.saturating_sub(size.height >> 1);
            let y_max = center.y.saturating_add(size.height >> 1);
            move |e: &super::Explorer| {
                if x_min < e.position.x
                    && e.position.x < x_max
                    && y_min < e.position.y
                    && e.position.y < y_max
                {
                    Some(e.clone())
                } else {
                    None
                }
            }
        }
    };
    let explorers = store.explorers.read().await;
    let res = explorers.iter().filter_map(|(_, e)| f(e)).collect();
    Ok(res)
}

async fn update_explorer<E: ProvideEventService>(
    event_service: &E,
    store: &super::ExplorerStore,
    params: super::UpdateExplorerParams,
) -> Result<super::Explorer, super::Error> {
    let super::UpdateExplorerParams { id, position } = params;
    let updated = {
        let mut explorers = store.explorers.write().await;
        let explorer = explorers.get_mut(&id).ok_or(super::Error::NotFound)?;
        explorer.position = position;
        explorer.clone()
    };
    let event = Event::Explorer(super::ExplorerAction::Move(updated.clone()));
    event_service
        .publish_event(event)
        .await
        .map_err(IntoStatus::into_status)?;
    Ok(updated)
}

async fn delete_explorer<E: ProvideEventService>(
    event_service: &E,
    store: &super::ExplorerStore,
    params: super::DeleteExplorerParams,
) -> Result<super::Explorer, super::Error> {
    let super::DeleteExplorerParams { id } = params;
    let deleted = {
        let mut explorers = store.explorers.write().await;
        explorers.remove(&id).ok_or(super::Error::NotFound)
    }?;
    let event = Event::Explorer(super::ExplorerAction::Leave(deleted.clone()));
    event_service
        .publish_event(event)
        .await
        .map_err(IntoStatus::into_status)?;
    Ok(deleted)
}

// MARK: ExploreService

impl<Context> super::ExploreService<Context> for super::ExploreServiceImpl
where
    Context: ProvideEventService
        + crate::world::ProvideWorldService
        + crate::user::ProvideUserService
        + crate::message::ProvideMessageService
        + crate::speaker_phone::ProvideSpeakerPhoneService
        + super::ProvideExplorerService
        + crate::reaction::ProvideReactionService,
{
    // type Error = super::error::Error;
    type Error = tonic::Status;

    fn explore<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::ExploreParams<'a>,
    ) -> futures::stream::BoxStream<'a, Result<super::ExplorationFieldEvents, Self::Error>> {
        use futures::StreamExt;

        explore(ctx, params).boxed()
    }
}

fn explore<'a, Context>(
    ctx: &'a Context,
    params: super::ExploreParams<'a>,
) -> impl futures::Stream<Item = Result<super::ExplorationFieldEvents, tonic::Status>>
       + Send
       + use<'a, Context>
where
    Context: ProvideEventService
        + crate::world::ProvideWorldService
        + crate::user::ProvideUserService
        + crate::message::ProvideMessageService
        + crate::speaker_phone::ProvideSpeakerPhoneService
        + super::ProvideExplorerService
        + crate::reaction::ProvideReactionService,
{
    use futures::TryStreamExt;

    async_stream::try_stream! {
        // arrive
        let arrived = explore_arrive(ctx, params).await.map_err(IntoStatus::into_status)?;
        let Some(arrived) = arrived else {
            return;
        };
        let Arrived {
            explorer,
            field_size,
            initial_events,
            stream: field_stream,
        } = arrived;
        yield initial_events.clone();

        let explorer_id = explorer.id;

        // updates
        let explore_state = ExploreState::new(ctx, explorer, field_size, initial_events, field_stream);
        let explore_stream = explore_state.into_stream();
        tokio::pin!(explore_stream);
        while let Some(events) = explore_stream.try_next().await? {
            yield events;
        }

        // leave
        ctx.delete_explorer(super::DeleteExplorerParams {
            id: explorer_id
        }).await.map_err(IntoStatus::into_status)?;
    }
}

// MARK: explore / arrive

struct Arrived<'a> {
    explorer: super::Explorer,
    field_size: crate::world::Size,
    initial_events: super::ExplorationFieldEvents,
    stream: futures::stream::BoxStream<'a, super::ExplorationField>,
}

#[tracing::instrument(skip_all)]
async fn explore_arrive<'a, Context>(
    ctx: &'a Context,
    params: super::ExploreParams<'a>,
) -> Result<Option<Arrived<'a>>, tonic::Status>
where
    Context: ProvideEventService
        + crate::world::ProvideWorldService
        + crate::user::ProvideUserService
        + crate::message::ProvideMessageService
        + crate::speaker_phone::ProvideSpeakerPhoneService
        + super::ProvideExplorerService
        + crate::reaction::ProvideReactionService,
{
    use futures::StreamExt;

    let super::ExploreParams { id, mut stream } = params;
    let Some(initial_field) = stream.next().await else {
        return Ok(None);
    };

    match ctx
        .check_coordinate(crate::world::CheckCoordinateParams {
            coordinate: initial_field.position,
        })
        .await
        .map_err(IntoStatus::into_status)?
    {
        crate::world::CheckCoordinateAnswer::Valid(_) => {}
        crate::world::CheckCoordinateAnswer::Invalid => {
            tracing::warn!(position = ?initial_field.position, "Received invalid position");
            return Ok(None);
        }
    }

    let user = ctx
        .get_user(crate::user::GetUserParams { id })
        .await
        .map_err(IntoStatus::into_status)?;
    let explorer = ctx
        .create_explorer(super::CreateExplorerParams {
            inner: user,
            position: initial_field.position,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    let other_explorers: Vec<_> = ctx
        .get_explorers_in_area(super::GetExplorersInAreaParams::Rect {
            center: initial_field.position,
            size: initial_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?
        .into_iter()
        .filter_map(|e| {
            if e.id != explorer.id {
                Some(super::ExplorerAction::Arrive(e))
            } else {
                None
            }
        })
        .collect();
    let messages = ctx
        .get_messages_in_area(crate::message::GetMessagesInAreaParams {
            center: initial_field.position,
            size: initial_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;
    let reactions = ctx
        .get_reactions_in_area(crate::reaction::GetReactionsInAreaParams {
            center: initial_field.position,
            size: initial_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;
    let speaker_phones = ctx
        .get_speaker_phones_in_area(crate::speaker_phone::GetSpeakerPhonesInAreaParams {
            center: initial_field.position,
            size: initial_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    let initial_events = super::ExplorationFieldEvents {
        explorer_actions: other_explorers,
        messages,
        reactions,
        speaker_phones,
    };

    let arrived = Arrived {
        explorer,
        field_size: initial_field.size,
        initial_events,
        stream,
    };
    Ok(Some(arrived))
}

// MARK: explore / update-loop

struct ReadEvents {
    explorer: HashMap<super::ExplorerId, crate::world::Coordinate>,
    message: HashSet<crate::message::MessageId>,
    reaction: HashSet<crate::reaction::ReactionId>,
    speaker_phone: HashSet<crate::speaker_phone::SpeakerPhoneId>,
}

struct ExploreState<'a, Context, E> {
    ctx: &'a Context,
    explorer: super::Explorer,
    field_size: crate::world::Size,
    read_events: ReadEvents,
    field_stream: futures::stream::BoxStream<'a, super::ExplorationField>,
    event_stream: futures::stream::BoxStream<'static, Result<crate::event::Event, E>>,
}

impl<'a, Context, E> ExploreState<'a, Context, E>
where
    Context: ProvideEventService
        + crate::world::ProvideWorldService
        + crate::message::ProvideMessageService
        + crate::speaker_phone::ProvideSpeakerPhoneService
        + super::ProvideExplorerService
        + crate::reaction::ProvideReactionService,
    E: IntoStatus,
    Context::EventService:
        crate::event::EventService<<Context as ProvideEventService>::Context, Error = E>,
{
    fn new(
        ctx: &'a Context,
        explorer: super::Explorer,
        field_size: crate::world::Size,
        initial_events: super::ExplorationFieldEvents,
        field_stream: futures::stream::BoxStream<'a, super::ExplorationField>,
    ) -> Self {
        let super::ExplorationFieldEvents {
            explorer_actions,
            messages,
            speaker_phones,
            reactions,
        } = initial_events;
        let explorer_it = explorer_actions.into_iter().filter_map(|a| match a {
            super::ExplorerAction::Arrive(e) if e.id != explorer.id => Some((e.id, e.position)),
            super::ExplorerAction::Move(e) if e.id != explorer.id => Some((e.id, e.position)),
            super::ExplorerAction::Leave(_) => None,
            _ => None,
        });
        let read_events = ReadEvents {
            explorer: explorer_it.collect(),
            message: messages.into_iter().map(|m| m.id).collect(),
            speaker_phone: speaker_phones.into_iter().map(|sp| sp.id).collect(),
            reaction: reactions.into_iter().map(|r| r.id).collect(),
        };
        let event_stream = ctx.subscribe_events();
        Self {
            ctx,
            explorer,
            field_size,
            read_events,
            field_stream,
            event_stream,
        }
    }

    fn into_stream(
        mut self,
    ) -> impl futures::Stream<Item = Result<super::ExplorationFieldEvents, tonic::Status>>
           + Send
           + use<'a, Context, E> {
        use futures::{future, stream, StreamExt};

        let (tx, rx) = tokio::sync::mpsc::channel(2);
        let generate = async move {
            while let Some(event) = self.yield_next().await? {
                tx.send(Ok(event)).await.unwrap();
            }
            Ok(None)
        };
        let tx_stream = stream::once(generate).filter_map(|g| future::ready(g.transpose()));
        let rx_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        stream::select(tx_stream, rx_stream)
    }

    async fn yield_next(&mut self) -> Result<Option<super::ExplorationFieldEvents>, tonic::Status> {
        use futures::stream::{StreamExt, TryStreamExt};

        enum Update {
            Field(super::ExplorationField),
            Event(crate::event::Event),
        }

        loop {
            let update = tokio::select! {
                field = self.field_stream.next() => field.map(Update::Field),
                event = self.event_stream.try_next() =>
                    event.map_err(IntoStatus::into_status)?.map(Update::Event),
            };
            let flow = match update {
                Some(Update::Field(field)) => self.update_field(field).await?,
                Some(Update::Event(event)) => self.update_event(event).await?,
                None => return Ok(None),
            };
            match flow {
                ControlFlow::Break(e) => return Ok(e),
                ControlFlow::Continue(()) => (),
            }
        }
    }

    // MARK: ExploreState::update_field

    async fn update_field(
        &mut self,
        field: super::ExplorationField,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        let id = self.explorer.id;
        let position = field.position;

        match self
            .ctx
            .check_coordinate(crate::world::CheckCoordinateParams {
                coordinate: position,
            })
            .await
            .map_err(IntoStatus::into_status)?
        {
            crate::world::CheckCoordinateAnswer::Valid(_) => {}
            crate::world::CheckCoordinateAnswer::Invalid => {
                tracing::warn!(?position, "Received invalid position");
                return Ok(ControlFlow::Break(None));
            }
        }

        self.explorer = self
            .ctx
            .update_explorer(super::UpdateExplorerParams { id, position })
            .await
            .map_err(IntoStatus::into_status)?;
        self.field_size = field.size;

        let explorer_actions = self.update_field_explorer_actions().await?;
        let messages = self.update_field_messages().await?;
        let speaker_phones = self.update_field_speaker_phones().await?;
        let reactions = self.update_field_reactions().await?;

        if explorer_actions.is_empty()
            && messages.is_empty()
            && speaker_phones.is_empty()
            && reactions.is_empty()
        {
            return Ok(ControlFlow::Continue(()));
        }

        let events = super::ExplorationFieldEvents {
            explorer_actions,
            messages,
            speaker_phones,
            reactions,
        };
        Ok(ControlFlow::Break(Some(events)))
    }

    async fn update_field_explorer_actions(
        &mut self,
    ) -> Result<Vec<super::ExplorerAction>, tonic::Status> {
        let id = self.explorer.id;
        let center = self.explorer.position;
        let size = self.field_size;

        let explorers = self
            .ctx
            .get_explorers_in_area(super::GetExplorersInAreaParams::Rect { center, size })
            .await
            .map_err(IntoStatus::into_status)?;
        let explorer_actions = explorers.into_iter().filter_map(|e| {
            if e.id == id {
                return None;
            }
            match self.read_events.explorer.get(&e.id) {
                Some(pos) if pos == &e.position => None,
                Some(_) => Some(super::ExplorerAction::Move(e)),
                None => Some(super::ExplorerAction::Arrive(e)),
            }
        });
        let explorer_actions: Vec<_> = explorer_actions.collect();
        self.read_events
            .explorer
            .extend(explorer_actions.iter().map(|e| match e {
                super::ExplorerAction::Arrive(e) => (e.id, e.position),
                super::ExplorerAction::Move(e) => (e.id, e.position),
                super::ExplorerAction::Leave(_) => unreachable!(),
            }));
        Ok(explorer_actions)
    }

    async fn update_field_messages(
        &mut self,
    ) -> Result<Vec<crate::message::Message>, tonic::Status> {
        let center = self.explorer.position;
        let size = self.field_size;

        let messages = self
            .ctx
            .get_messages_in_area(crate::message::GetMessagesInAreaParams { center, size })
            .await
            .map_err(IntoStatus::into_status)?;
        let messages: Vec<_> = messages
            .into_iter()
            .filter(|m| !self.read_events.message.contains(&m.id))
            .collect();
        self.read_events
            .message
            .extend(messages.iter().map(|m| m.id));
        Ok(messages)
    }

    async fn update_field_speaker_phones(
        &mut self,
    ) -> Result<Vec<crate::speaker_phone::SpeakerPhone>, tonic::Status> {
        let center = self.explorer.position;
        let size = self.field_size;

        let speaker_phones = self
            .ctx
            .get_speaker_phones_in_area(crate::speaker_phone::GetSpeakerPhonesInAreaParams {
                center,
                size,
            })
            .await
            .map_err(IntoStatus::into_status)?;
        let speaker_phones: Vec<_> = speaker_phones
            .into_iter()
            .filter(|sp| !self.read_events.speaker_phone.contains(&sp.id))
            .collect();
        self.read_events
            .speaker_phone
            .extend(speaker_phones.iter().map(|sp| sp.id));
        Ok(speaker_phones)
    }

    async fn update_field_reactions(
        &mut self,
    ) -> Result<Vec<crate::reaction::Reaction>, tonic::Status> {
        let center = self.explorer.position;
        let size = self.field_size;

        let reactions = self
            .ctx
            .get_reactions_in_area(crate::reaction::GetReactionsInAreaParams { center, size })
            .await
            .map_err(IntoStatus::into_status)?;
        let reactions: Vec<_> = reactions
            .into_iter()
            .filter(|r| !self.read_events.reaction.contains(&r.id))
            .collect();
        self.read_events
            .reaction
            .extend(reactions.iter().map(|r| r.id));
        Ok(reactions)
    }

    // MARK: ExploreState::update_event

    async fn update_event(
        &mut self,
        event: crate::event::Event,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        use super::ExplorerAction;
        use crate::event::Event;

        match event {
            Event::Explorer(ExplorerAction::Arrive(e))
                if e.id != self.explorer.id && self.is_inside(e.position) =>
            {
                self.update_explorer_arrive(e).await
            }
            Event::Explorer(ExplorerAction::Move(e))
                if e.id != self.explorer.id && self.is_inside(e.position) =>
            {
                self.update_explorer_move(e).await
            }
            Event::Explorer(ExplorerAction::Leave(e))
                if e.id != self.explorer.id && self.is_inside(e.position) =>
            {
                self.update_explorer_leave(e).await
            }
            Event::Message(m)
                if !self.read_events.message.contains(&m.id) && self.is_inside(m.position) =>
            {
                self.update_new_message(m).await
            }
            Event::SpeakerPhone(sp)
                if !self.read_events.speaker_phone.contains(&sp.id)
                    && self.is_inside(sp.position) =>
            {
                self.update_new_speaker_phone(sp).await
            }
            Event::Reaction(r)
                if !self.read_events.reaction.contains(&r.id) && self.is_inside(r.position) =>
            {
                self.update_new_reaction(r).await
            }
            _ => Ok(ControlFlow::Continue(())),
        }
    }

    async fn update_explorer_arrive(
        &mut self,
        explorer: super::Explorer,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        match self
            .read_events
            .explorer
            .insert(explorer.id, explorer.position)
        {
            Some(p) if p == explorer.position => return Ok(ControlFlow::Continue(())),
            _ => (),
        };
        let events = super::ExplorationFieldEvents {
            explorer_actions: vec![super::ExplorerAction::Arrive(explorer)],
            ..Default::default()
        };
        Ok(ControlFlow::Break(Some(events)))
    }

    async fn update_explorer_move(
        &mut self,
        explorer: super::Explorer,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        match self
            .read_events
            .explorer
            .insert(explorer.id, explorer.position)
        {
            Some(p) if p == explorer.position => return Ok(ControlFlow::Continue(())),
            _ => (),
        };
        let events = super::ExplorationFieldEvents {
            explorer_actions: vec![super::ExplorerAction::Move(explorer)],
            ..Default::default()
        };
        Ok(ControlFlow::Break(Some(events)))
    }

    async fn update_explorer_leave(
        &mut self,
        explorer: super::Explorer,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        match self.read_events.explorer.remove(&explorer.id) {
            Some(_) => {
                let events = super::ExplorationFieldEvents {
                    explorer_actions: vec![super::ExplorerAction::Leave(explorer)],
                    ..Default::default()
                };
                Ok(ControlFlow::Break(Some(events)))
            }
            None => Ok(ControlFlow::Continue(())),
        }
    }

    async fn update_new_message(
        &mut self,
        message: crate::message::Message,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        if self.read_events.message.insert(message.id) {
            let events = super::ExplorationFieldEvents {
                messages: vec![message],
                ..Default::default()
            };
            Ok(ControlFlow::Break(Some(events)))
        } else {
            Ok(ControlFlow::Continue(()))
        }
    }

    async fn update_new_speaker_phone(
        &mut self,
        speaker_phone: crate::speaker_phone::SpeakerPhone,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        if self.read_events.speaker_phone.insert(speaker_phone.id) {
            let events = super::ExplorationFieldEvents {
                speaker_phones: vec![speaker_phone],
                ..Default::default()
            };
            Ok(ControlFlow::Break(Some(events)))
        } else {
            Ok(ControlFlow::Continue(()))
        }
    }

    async fn update_new_reaction(
        &mut self,
        reaction: crate::reaction::Reaction,
    ) -> Result<ControlFlow<Option<super::ExplorationFieldEvents>>, tonic::Status> {
        if self.read_events.reaction.insert(reaction.id) {
            let events = super::ExplorationFieldEvents {
                reactions: vec![reaction],
                ..Default::default()
            };
            Ok(ControlFlow::Break(Some(events)))
        } else {
            Ok(ControlFlow::Continue(()))
        }
    }

    fn is_inside(&self, point: crate::world::Coordinate) -> bool {
        let x_min = u32::saturating_sub(self.explorer.position.x, self.field_size.width >> 1);
        let x_max = u32::saturating_add(self.explorer.position.x, self.field_size.width >> 1);
        let y_min = u32::saturating_sub(self.explorer.position.y, self.field_size.height >> 1);
        let y_max = u32::saturating_add(self.explorer.position.y, self.field_size.height >> 1);
        x_min < point.x && point.x < x_max && y_min < point.y && point.y < y_max
    }
}
