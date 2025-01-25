use futures::FutureExt;

use crate::{prelude::IntoStatus, traq::{
    bot::{BuildRequestAsBotParams, ProvideTraqBotService},
    TraqHost,
}};

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

async fn get_all_channels<'a, Context>(
    ctx: &'a Context,
    params: super::GetAllChannelsParams,
) -> Result<Vec<super::TraqChannel>, super::error::Error>
where
    Context: ProvideTraqBotService + AsRef<TraqHost>,
{
    // send request
    let traq_host: &TraqHost = ctx.as_ref();

    let params = BuildRequestAsBotParams {
        method: http::Method::GET,
        uri: &format!("https://{}/api/v3/channels", traq_host),
    };

    let result = ctx
        .build_request_as_bot(params)
        .await
        .map_err(IntoStatus::into_status)?
        .send()
        .await
        .map_err(|_|super::error::Error::RequestSendError)?
        .json::<Vec<super::TraqChannel>>()
        .await
        .map_err(|_|super::error::Error::ParseError)?;

    Ok(result)
}