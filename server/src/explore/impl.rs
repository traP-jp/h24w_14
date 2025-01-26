use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use futures::future::{BoxFuture, FutureExt};
use tokio::sync::RwLock;

use futures::StreamExt;

use crate::{
    event::{Event, ProvideEventService},
    message::ProvideMessageService,
    prelude::IntoStatus,
    reaction::ProvideReactionService,
    speaker_phone::ProvideSpeakerPhoneService,
    user::ProvideUserService,
};

use super::ProvideExplorerService;

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

impl<Context> super::ExploreService<Context> for super::ExploreServiceImpl
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService
        + ProvideReactionService,
{
    // type Error = super::error::Error;
    type Error = tonic::Status;

    fn explore<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::ExploreParams<'a>,
    ) -> futures::stream::BoxStream<'a, Result<super::ExplorationFieldEvents, Self::Error>> {
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
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService
        + ProvideReactionService,
{
    async_stream::try_stream! {
        let super::ExploreParams { id, stream: mut exploration_field_stream } = params;
        let event_stream = ctx.subscribe_events();

        let exploration_field_first_value = exploration_field_stream.next()
            .await
            .ok_or(super::error::Error::ExplorationFieldStreamClosed)?;

        // new explorer arrives
        let user = ctx.get_user(crate::user::GetUserParams { id }).await?;
        let explorer = ctx.create_explorer(super::CreateExplorerParams {
            inner: user,
            position: exploration_field_first_value.position,
        }).await?;
        let exploration_field = exploration_field_first_value;

        // create status
        let old_area_messages_cache = ctx.get_messages_in_area(
            crate::message::GetMessagesInAreaParams {
                center: exploration_field.position,
                size: exploration_field.size,
            },
        ).await?;

        let old_area_speaker_phones_cache = ctx.get_speaker_phones_in_area(
            crate::speaker_phone::GetSpeakerPhonesInAreaParams {
                center: exploration_field.position,
                size: exploration_field.size,
            },
        ).await?;

        let old_area_reactions_cache = ctx.get_reactions_in_area(
            crate::reaction::GetReactionsInAreaParams {
                center: exploration_field.position,
                size: exploration_field.size,
            },
        ).await?;

        let old_area_explorers_cache = ctx.get_explorers_in_area(
            crate::explore::GetExplorersInAreaParams::Rect {
                center: exploration_field.position,
                size: exploration_field.size,
            },
        ).await?;

        yield super::ExplorationFieldEvents {
            messages: old_area_messages_cache.clone(),
            speaker_phones: old_area_speaker_phones_cache.clone(),
            reactions: old_area_reactions_cache.clone(),
            explorer_actions: old_area_explorers_cache.iter().map(|e| {
                super::ExplorerAction::Arrive(e.clone())
            }).collect(),
        };

        let mut status = ExplorerStatus {
            ctx,
            explorer: explorer.clone(),
            exploration_field_size: exploration_field.size,
            old_area_messages_cache,
            old_area_speaker_phones_cache,
            old_area_reactions_cache,
            old_area_explorers_cache,
        };

        // main loop
        let mut select = futures::stream::select(exploration_field_stream.map(
            SelectResult::ExplorationField,
        ), event_stream.map(
            |event| {
                let event = match event {
                    Ok(event) => event,
                    Err(e) => {
                        return SelectResult::EventStreamClosed(super::error::Error::Status(e.into()));
                    },
                };
                SelectResult::Event(event)
            },
        ));

        loop {
            let Some(select_result) = select.next().await else {
                // explore leaves
                break;
            };

            if let Some(exploration_field_events) = event_handle(
                select_result,
                &mut status,
            ).await? {
                yield exploration_field_events;
            }
        }

        // explorer leaves
        ctx.delete_explorer(crate::explore::DeleteExplorerParams { id: explorer.id }).await.map_err(IntoStatus::into_status)?;
    }
}

enum SelectResult {
    ExplorationField(super::ExplorationField),
    Event(crate::event::Event),
    EventStreamClosed(super::error::Error),
}

struct ExplorerStatus<'a, Context>
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService,
{
    ctx: &'a Context,
    explorer: super::Explorer,
    exploration_field_size: crate::world::Size,
    old_area_messages_cache: Vec<crate::message::Message>,
    old_area_speaker_phones_cache: Vec<crate::speaker_phone::SpeakerPhone>,
    old_area_reactions_cache: Vec<crate::reaction::Reaction>,
    old_area_explorers_cache: Vec<super::Explorer>,
}

async fn event_handle<Context>(
    select_result: SelectResult,
    status: &mut ExplorerStatus<'_, Context>,
) -> Result<Option<super::ExplorationFieldEvents>, tonic::Status>
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService
        + ProvideReactionService,
{
    match select_result {
        SelectResult::EventStreamClosed(e) => Err(e.into()),
        SelectResult::ExplorationField(exploration_field) => {
            when_exploration_field_moved(exploration_field, status).await
        }
        SelectResult::Event(event) => when_received_event(event, status),
    }
}

async fn when_exploration_field_moved<Context>(
    new_exploration_field: super::ExplorationField,
    status: &mut ExplorerStatus<'_, Context>,
) -> Result<Option<super::ExplorationFieldEvents>, tonic::Status>
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService
        + ProvideReactionService,
{
    // publish explorer move event
    status
        .ctx
        .update_explorer(crate::explore::UpdateExplorerParams {
            id: status.explorer.id,
            position: status.explorer.position,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    // collect newly contained events

    // messages
    let new_area_messages = status
        .ctx
        .get_messages_in_area(crate::message::GetMessagesInAreaParams {
            center: new_exploration_field.position,
            size: new_exploration_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    let messages = new_area_messages
        .iter()
        .filter_map(|new_message| {
            if !status.old_area_messages_cache.contains(new_message) {
                Some(new_message.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // speaker_phones
    let new_area_speaker_phones = status
        .ctx
        .get_speaker_phones_in_area(crate::speaker_phone::GetSpeakerPhonesInAreaParams {
            center: new_exploration_field.position,
            size: new_exploration_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    let speaker_phones = new_area_speaker_phones
        .iter()
        .filter_map(|new_speaker_phone| {
            if !status
                .old_area_speaker_phones_cache
                .contains(new_speaker_phone)
            {
                Some(new_speaker_phone.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // reactions
    let new_area_reactions = status
        .ctx
        .get_reactions_in_area(crate::reaction::GetReactionsInAreaParams {
            center: new_exploration_field.position,
            size: new_exploration_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    let reactions = new_area_reactions
        .iter()
        .filter_map(|new_reaction| {
            if !status.old_area_reactions_cache.contains(new_reaction) {
                Some(new_reaction.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // explorers
    let new_area_explorers = status
        .ctx
        .get_explorers_in_area(crate::explore::GetExplorersInAreaParams::Rect {
            center: new_exploration_field.position,
            size: new_exploration_field.size,
        })
        .await
        .map_err(IntoStatus::into_status)?;

    let explorer_actions = new_area_explorers
        .iter()
        .filter_map(|new_explorer| {
            if !status.old_area_explorers_cache.contains(new_explorer) {
                Some(super::ExplorerAction::Arrive(new_explorer.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // update exploration field / cache
    status.explorer.position = new_exploration_field.position;
    status.exploration_field_size = new_exploration_field.size;
    status.old_area_messages_cache = new_area_messages;
    status.old_area_speaker_phones_cache = new_area_speaker_phones;
    status.old_area_explorers_cache = new_area_explorers;

    // exploration field events
    Ok(Some(super::ExplorationFieldEvents {
        messages,
        speaker_phones,
        reactions,
        explorer_actions,
    }))
}

fn when_received_event<Context>(
    event: crate::event::Event,
    status: &mut ExplorerStatus<'_, Context>,
) -> Result<Option<super::ExplorationFieldEvents>, tonic::Status>
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService
        + ProvideReactionService,
{
    match event {
        crate::event::Event::Explorer(explorer_action) => {
            if is_inside(
                status.explorer.position,
                status.exploration_field_size,
                explorer_action.explorer().position,
            ) {
                Ok(Some(super::ExplorationFieldEvents {
                    messages: vec![],
                    speaker_phones: vec![],
                    reactions: vec![],
                    explorer_actions: vec![explorer_action],
                }))
            } else {
                Ok(None)
            }
        }
        crate::event::Event::SpeakerPhone(speaker_phone) => {
            if is_inside(
                status.explorer.position,
                status.exploration_field_size,
                speaker_phone.position,
            ) {
                Ok(Some(super::ExplorationFieldEvents {
                    messages: vec![],
                    speaker_phones: vec![speaker_phone],
                    reactions: vec![],
                    explorer_actions: vec![],
                }))
            } else {
                Ok(None)
            }
        }
        crate::event::Event::Message(message) => {
            if is_inside(
                status.explorer.position,
                status.exploration_field_size,
                message.position,
            ) {
                Ok(Some(super::ExplorationFieldEvents {
                    messages: vec![message],
                    speaker_phones: vec![],
                    reactions: vec![],
                    explorer_actions: vec![],
                }))
            } else {
                Ok(None)
            }
        }
        crate::event::Event::Reaction(reaction) => {
            if is_inside(
                status.explorer.position,
                status.exploration_field_size,
                reaction.position,
            ) {
                Ok(Some(super::ExplorationFieldEvents {
                    messages: vec![],
                    speaker_phones: vec![],
                    reactions: vec![reaction],
                    explorer_actions: vec![],
                }))
            } else {
                Ok(None)
            }
        }
    }
}

fn is_inside(
    center: crate::world::Coordinate,
    size: crate::world::Size,
    position: crate::world::Coordinate,
) -> bool {
    let x_min = center.x.saturating_sub(size.width >> 1);
    let x_max = center.x.saturating_add(size.width >> 1);
    let y_min = center.y.saturating_sub(size.height >> 1);
    let y_max = center.y.saturating_add(size.height >> 1);
    x_min < position.x && position.x < x_max && y_min < position.y && position.y < y_max
}
