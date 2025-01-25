use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

pub mod error;
mod r#impl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TraqChannelId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TraqChannel {
    pub id: TraqChannelId,
    /// フルパス
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetAllChannelsParams {}

pub trait TraqChannelService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_all_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetAllChannelsParams,
    ) -> BoxFuture<'a, Result<Vec<TraqChannel>, Self::Error>>;
}

pub struct TraqChannelServiceImpl;

#[allow(clippy::type_complexity)]
pub trait ProvideTraqChannelService: Send + Sync + 'static {
    type Context;
    type TraqChannelService: TraqChannelService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn traq_channel_service(&self) -> &Self::TraqChannelService;

    fn get_all_channels(
        &self,
        params: GetAllChannelsParams,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<TraqChannel>,
            <Self::TraqChannelService as TraqChannelService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_channel_service().get_all_channels(ctx, params)
    }
}
