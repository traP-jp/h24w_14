use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use futures::future::{BoxFuture, FutureExt};
use tokio::sync::RwLock;

use crate::{
    event::{Event, ProvideEventService},
    prelude::IntoStatus,
};

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

use futures::StreamExt;
use uuid::Uuid;

impl<Context> super::ExploreService<Context> for super::ExploreServiceImpl
where
    Context: crate::event::ProvideEventService + crate::user::ProvideUserService,
{
    type Error = super::error::Error;

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
) -> impl futures::Stream<Item = Result<super::ExplorationFieldEvents, super::error::Error>>
       + Send
       + use<'a, Context>
where
    Context: crate::event::ProvideEventService + crate::user::ProvideUserService,
{
    async_stream::stream! {
        let (id, mut exploration_field_stream) = (params.id, params.stream);
        let event_stream = ctx.subscribe_events();

        let Some(exploration_field_first_value) = exploration_field_stream.next().await else {
            yield Err(super::error::Error::ExplorationFieldStreamClosed);
            return;
        };

        // new explorer arrives

        let Ok(user) = ctx.get_user(crate::user::GetUserParams { id }).await else {
            yield Err(super::error::Error::NotFound);
            return;
        };
        let id = super::ExplorerId(Uuid::now_v7());
        let mut exploration_field = exploration_field_first_value;

        match ctx.publish_event(crate::event::Event::Explorer(super::ExplorerAction::Arrive(
            super::Explorer {
                id: id,
                inner: user.clone(),
                position: exploration_field.position,
            },
        ))).await  {
            Ok(_) => (),
            Err(e) => {
                yield Err(super::error::Error::Status(e.into()));
                return;
            }
        };

        // main loop

        let mut select = futures::stream::select(exploration_field_stream.map(
            |exploration_field| SelectResult::ExplorationField(exploration_field),
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

            match select_result {
                SelectResult::EventStreamClosed(e) => {
                    yield Err(e);
                    break;
                },
                SelectResult::ExplorationField(new_exploration_field) => {
                    exploration_field = new_exploration_field;

                    match ctx.publish_event(crate::event::Event::Explorer(super::ExplorerAction::Move(
                        super::Explorer {
                            id: id,
                            inner: user.clone(),
                            position: exploration_field.position,
                        },
                    ))).await {
                        Ok(_) => (),
                        Err(e) => {
                            yield Err(super::error::Error::Status(e.into()));
                            break;
                        }
                    };

                    // todo
                },
                SelectResult::Event(event) => {
                    match event {
                        crate::event::Event::Explorer(explorer_action) => {
                            if explorer_action.explorer().id != id {
                                yield Ok(
                                    super::ExplorationFieldEvents {
                                        messages: vec![],
                                        speaker_phones: vec![],
                                        reactions: vec![],
                                        explorer_actions: vec![explorer_action],
                                    }
                                );
                            }
                        },
                        crate::event::Event::SpkeakerPhone(speaker_phone) => {
                            yield Ok(
                                super::ExplorationFieldEvents {
                                    messages: vec![],
                                    speaker_phones: vec![speaker_phone],
                                    reactions: vec![],
                                    explorer_actions: vec![],
                                }
                            );
                        },
                        crate::event::Event::Message(message) => {
                            yield Ok(
                                super::ExplorationFieldEvents {
                                    messages: vec![message],
                                    speaker_phones: vec![],
                                    reactions: vec![],
                                    explorer_actions: vec![],
                                }
                            );
                        },
                        crate::event::Event::Reaction(reaction) => {
                            yield Ok(
                                super::ExplorationFieldEvents {
                                    messages: vec![],
                                    speaker_phones: vec![],
                                    reactions: vec![reaction],
                                    explorer_actions: vec![],
                                }
                            );
                        },
                    }
                },
            }
        }

        // explorer leaves

        match ctx.publish_event(crate::event::Event::Explorer(super::ExplorerAction::Leave(
            super::Explorer {
                id: id,
                inner: user.clone(),
                position: exploration_field.position,
            },
        ))).await {
            Ok(_) => (),
            Err(e) => {
                yield Err(super::error::Error::Status(e.into()));
            }
        };
    }
}

enum SelectResult {
    ExplorationField(super::ExplorationField),
    Event(crate::event::Event),
    EventStreamClosed(super::error::Error),
}
