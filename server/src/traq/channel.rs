use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::IntoStatus;

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
pub struct GetAllChannels {}

pub trait TraqChannelService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_all_channels<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetAllChannels,
    ) -> BoxFuture<'a, Result<Vec<TraqChannel>, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideTraqChannelService: Send + Sync + 'static {
    type Context;
    type TraqChannelService: TraqChannelService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn traq_channel_service(&self) -> &Self::TraqChannelService;

    fn get_all_channels(
        &self,
        req: GetAllChannels,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<TraqChannel>,
            <Self::TraqChannelService as TraqChannelService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.traq_channel_service().get_all_channels(ctx, req)
    }
}
