use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use futures::future::{BoxFuture, FutureExt};
use tokio::sync::RwLock;

use futures::StreamExt;
use uuid::Uuid;

use crate::{
    event::{Event, ProvideEventService},
    message::ProvideMessageService,
    prelude::IntoStatus,
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
        + ProvideExplorerService,
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
        + ProvideExplorerService,
{
    async_stream::try_stream! {
        let (id, mut exploration_field_stream) = (params.id, params.stream);
        let event_stream = ctx.subscribe_events();

        let exploration_field_first_value = exploration_field_stream.next()
            .await
            .ok_or(super::error::Error::ExplorationFieldStreamClosed)?;

        // new explorer arrives

        let user = ctx.get_user(crate::user::GetUserParams { id }).await?;
        let id = super::ExplorerId(Uuid::now_v7());
        let exploration_field = exploration_field_first_value;

        ctx.publish_event(crate::event::Event::Explorer(super::ExplorerAction::Arrive(
            super::Explorer {
                id,
                inner: user.clone(),
                position: exploration_field.position,
            },
        ))).await?;

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

        let old_area_explorers_cache = ctx.get_explorers_in_area(
            crate::explore::GetExplorersInAreaParams::Rect {
                center: exploration_field.position,
                size: exploration_field.size,
            },
        ).await?;

        // create status
        let mut status = Status {
            ctx,
            id,
            user,
            exploration_field,
            old_area_messages_cache,
            old_area_speaker_phones_cache,
            old_area_explorers_cache,
        };

        loop {
            let Some(select_result) = select.next().await else {
                // explore leaves
                break;
            };

            if let Some(exploration_field_events) = _yield(
                select_result,
                &mut status,
            ).await? {
                yield exploration_field_events;
            }
        }

        // explorer leaves
        ctx.publish_event(crate::event::Event::Explorer(super::ExplorerAction::Leave(
            super::Explorer {
                id,
                inner: status.user.clone(),
                position: status.exploration_field.position,
            },
        ))).await?;
    }
}

enum SelectResult {
    ExplorationField(super::ExplorationField),
    Event(crate::event::Event),
    EventStreamClosed(super::error::Error),
}

struct Status<'a, Context>
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService,
{
    ctx: &'a Context,
    id: super::ExplorerId,
    user: crate::user::User,
    exploration_field: super::ExplorationField,
    old_area_messages_cache: Vec<crate::message::Message>,
    old_area_speaker_phones_cache: Vec<crate::speaker_phone::SpeakerPhone>,
    old_area_explorers_cache: Vec<super::Explorer>,
}

async fn _yield<Context>(
    select_result: SelectResult,
    status: &mut Status<'_, Context>,
) -> Result<Option<super::ExplorationFieldEvents>, tonic::Status>
where
    Context: ProvideEventService
        + ProvideUserService
        + ProvideMessageService
        + ProvideSpeakerPhoneService
        + ProvideExplorerService,
{
    let Status {
        ctx,
        id,
        user,
        exploration_field,
        old_area_messages_cache,
        old_area_speaker_phones_cache,
        old_area_explorers_cache,
    } = status;

    match select_result {
        SelectResult::EventStreamClosed(e) => Err(e.into()),
        SelectResult::ExplorationField(new_exploration_field) => {
            ctx.publish_event(crate::event::Event::Explorer(super::ExplorerAction::Move(
                super::Explorer {
                    id: *id,
                    inner: user.clone(),
                    position: new_exploration_field.position,
                },
            )))
            .await
            .map_err(IntoStatus::into_status)?;

            // collect newly contained events

            // messages
            let new_area_messages = ctx
                .get_messages_in_area(crate::message::GetMessagesInAreaParams {
                    center: new_exploration_field.position,
                    size: new_exploration_field.size,
                })
                .await
                .map_err(IntoStatus::into_status)?;

            let messages = new_area_messages
                .iter()
                .filter_map(|new_message| {
                    if !old_area_messages_cache.contains(new_message) {
                        Some(new_message.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // speaker_phones

            let new_area_speaker_phones = ctx
                .get_speaker_phones_in_area(crate::speaker_phone::GetSpeakerPhonesInAreaParams {
                    center: new_exploration_field.position,
                    size: new_exploration_field.size,
                })
                .await
                .map_err(IntoStatus::into_status)?;

            let speaker_phones = new_area_speaker_phones
                .iter()
                .filter_map(|new_speaker_phone| {
                    if !old_area_speaker_phones_cache.contains(new_speaker_phone) {
                        Some(new_speaker_phone.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // explorers

            let new_area_explorers = ctx
                .get_explorers_in_area(crate::explore::GetExplorersInAreaParams::Rect {
                    center: new_exploration_field.position,
                    size: new_exploration_field.size,
                })
                .await
                .map_err(IntoStatus::into_status)?;

            let explorer = new_area_explorers
                .iter()
                .filter_map(|new_explorer| {
                    if !old_area_explorers_cache.contains(new_explorer) {
                        Some(super::ExplorerAction::Arrive(new_explorer.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // update exploration field / cache

            *exploration_field = new_exploration_field;
            *old_area_messages_cache = new_area_messages;
            *old_area_speaker_phones_cache = new_area_speaker_phones;
            *old_area_explorers_cache = new_area_explorers;

            // exploration field events

            Ok(Some(super::ExplorationFieldEvents {
                messages,
                speaker_phones,
                reactions: vec![],
                explorer_actions: explorer,
            }))
        }
        SelectResult::Event(event) => match event {
            crate::event::Event::Explorer(explorer_action) => {
                if is_inside(
                    exploration_field.position,
                    exploration_field.size,
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
                    exploration_field.position,
                    exploration_field.size,
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
                    exploration_field.position,
                    exploration_field.size,
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
                    exploration_field.position,
                    exploration_field.size,
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
        },
    }
}

fn is_inside(
    center: crate::world::Coordinate,
    size: crate::world::Size,
    position: crate::world::Coordinate,
) -> bool {
    let x_min = center.x - size.width / 2;
    let x_max = center.x + size.width / 2;
    let y_min = center.y - size.height / 2;
    let y_max = center.y + size.height / 2;
    x_min < position.x && position.x < x_max && y_min < position.y && position.y < y_max
}
