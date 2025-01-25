use std::collections::HashMap;

use futures::FutureExt;

use crate::{
    prelude::IntoStatus,
    traq::{
        bot::{BuildRequestAsBotParams, ProvideTraqBotService},
        TraqHost,
    },
};

impl<Context> super::TraqChannelService<Context> for super::TraqChannelServiceImpl
where
    Context: ProvideTraqBotService + AsRef<TraqHost>,
{
    type Error = super::error::Error;

    fn get_all_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetAllChannelsParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::TraqChannel>, Self::Error>> {
        get_all_channels(ctx, params).boxed()
    }
}

async fn get_all_channels<Context>(
    ctx: &Context,
    _params: super::GetAllChannelsParams,
) -> Result<Vec<super::TraqChannel>, super::error::Error>
where
    Context: ProvideTraqBotService + AsRef<TraqHost>,
{
    // send request
    let traq_host: &TraqHost = ctx.as_ref();

    let params = BuildRequestAsBotParams {
        method: http::Method::GET,
        uri: &format!("https://{}/api/v3/channels?include-dm=false", traq_host),
    };

    let result = ctx
        .build_request_as_bot(params)
        .await
        .map_err(IntoStatus::into_status)?
        .send()
        .await
        .map_err(|e| {
            tracing::error!(error = &e as &dyn std::error::Error);
            super::error::Error::RequestSendError
        })?
        .json::<TraqChannelRaw>()
        .await
        .map_err(|e| {
            tracing::warn!(error = &e as &dyn std::error::Error);
            super::error::Error::ParseError
        })?;

    // build full paths
    let channels = result
        .public
        .into_iter()
        .map(|ch| {
            (
                ch.id,
                ChannelNode {
                    is_root: ch.parent_id.is_none(),
                    id: ch.id,
                    name: ch.name,
                    children: ch.children,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let root_channels = channels
        .values()
        .filter(|node| node.is_root)
        .collect::<Vec<_>>();

    Ok(root_channels
        .iter()
        .flat_map(|node| node.dfs("", &channels))
        .collect::<Vec<_>>())
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct TraqChannelRaw {
    public: Vec<PublicChannel>,
    // NOTE: dm=false だから使わない
    // dm: Vec<DmChannel>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct PublicChannel {
    id: super::TraqChannelId,
    parent_id: Option<super::TraqChannelId>,
    archived: bool,
    force: bool,
    topic: String,
    name: String,
    children: Vec<super::TraqChannelId>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct DmChannel {
    id: super::TraqChannelId,
    user_id: crate::traq::user::TraqUserId,
}

#[derive(Clone)]
struct ChannelNode {
    is_root: bool,
    id: super::TraqChannelId,
    name: String,
    children: Vec<super::TraqChannelId>,
}

impl ChannelNode {
    fn dfs(
        &self,
        path: &str,
        channels: &HashMap<super::TraqChannelId, ChannelNode>,
    ) -> Vec<super::TraqChannel> {
        // current channel
        let path = if path.is_empty() {
            format!("#{}", self.name)
        } else {
            format!("{}/{}", path, self.name)
        };

        let mut result = vec![super::TraqChannel {
            id: self.id,
            path: path.clone(),
        }];

        let it = self
            .children
            .iter()
            .flat_map(|child| channels.get(child))
            .flat_map(|child_node| child_node.dfs(&path, channels));
        result.extend(it);

        result
    }
}
