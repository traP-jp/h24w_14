use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

use crate::prelude::IntoStatus;
use crate::traq::{bot::ProvideTraqBotService, TraqHost};
use crate::user::ProvideUserService;

impl<Context> super::TraqUserService<Context> for super::TraqUserServiceImpl
where
    Context: AsRef<MySqlPool> + AsRef<TraqHost> + ProvideUserService + ProvideTraqBotService,
{
    type Error = super::Error;

    fn find_traq_user_by_app_user_id<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::FindTraqUserByAppUserIdParams,
    ) -> BoxFuture<'a, Result<Option<super::TraqUser>, Self::Error>> {
        find_traq_user_by_app_user_id(ctx, ctx.as_ref(), params).boxed()
    }

    fn find_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::FindTraqUserParams,
    ) -> BoxFuture<'a, Result<Option<super::TraqUser>, Self::Error>> {
        find_traq_user(ctx, ctx.as_ref(), params.id).boxed()
    }

    fn register_traq_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::RegisterTraqUserParams,
    ) -> BoxFuture<'a, Result<super::TraqUser, Self::Error>> {
        register_traq_user(ctx, ctx, ctx.as_ref(), ctx.as_ref(), params).boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, FromRow)]
struct TraqUserRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bot: bool,
    pub bio: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[tracing::instrument(skip_all)]
async fn find_traq_user_by_app_user_id<U: ProvideUserService>(
    user_service: &U,
    pool: &MySqlPool,
    params: super::FindTraqUserByAppUserIdParams,
) -> Result<Option<super::TraqUser>, super::Error> {
    let traq_user: Option<TraqUserRow> =
        sqlx::query_as(r#"SELECT * FROM `traq_users` WHERE `user_id` = ?"#)
            .bind(params.id.0)
            .fetch_optional(pool)
            .await?;
    let Some(traq_user) = traq_user else {
        return Ok(None);
    };
    let user_id = crate::user::UserId(traq_user.user_id);
    let user = user_service
        .get_user(crate::user::GetUserParams { id: user_id })
        .await
        .map_err(IntoStatus::into_status)?;
    let traq_user = super::TraqUser {
        id: super::TraqUserId(traq_user.id),
        inner: user,
        bot: traq_user.bot,
        bio: traq_user.bio,
        created_at: traq_user.created_at.into(),
        updated_at: traq_user.updated_at.into(),
    };
    Ok(Some(traq_user))
}

#[tracing::instrument(skip_all)]
async fn find_traq_user<U: ProvideUserService>(
    user_service: &U,
    pool: &MySqlPool,
    id: super::TraqUserId,
) -> Result<Option<super::TraqUser>, super::Error> {
    let traq_user: Option<TraqUserRow> =
        sqlx::query_as(r#"SELECT * FROM `traq_users` WHERE id = ?"#)
            .bind(id.0)
            .fetch_optional(pool)
            .await?;
    let Some(traq_user) = traq_user else {
        return Ok(None);
    };
    let user_id = crate::user::UserId(traq_user.user_id);
    let user = user_service
        .get_user(crate::user::GetUserParams { id: user_id })
        .await
        .map_err(IntoStatus::into_status)?;
    let traq_user = super::TraqUser {
        id: super::TraqUserId(traq_user.id),
        inner: user,
        bot: traq_user.bot,
        bio: traq_user.bio,
        created_at: traq_user.created_at.into(),
        updated_at: traq_user.updated_at.into(),
    };
    Ok(Some(traq_user))
}

#[tracing::instrument(skip_all)]
async fn register_traq_user<U: ProvideUserService, B: ProvideTraqBotService>(
    user_service: &U,
    traq_bot_service: &B,
    pool: &MySqlPool,
    traq_host: &TraqHost,
    params: super::RegisterTraqUserParams,
) -> Result<super::TraqUser, super::Error> {
    use super::Error::UnexpectedResponseFromTraq as FailJson;

    let super::RegisterTraqUserParams {
        id: super::TraqUserId(traq_user_id),
    } = params;

    let uri = format!("https://{traq_host}/api/v3/users/{traq_user_id}");
    let request = crate::traq::bot::BuildRequestAsBotParams {
        method: http::Method::GET,
        uri: &uri,
    };
    let request = traq_bot_service
        .build_request_as_bot(request)
        .await
        .map_err(IntoStatus::into_status)?;
    let response: serde_json::Value = request.send().await?.error_for_status()?.json().await?;
    tracing::trace!(value = ?response, "Received response from traQ");

    let response = response.as_object().ok_or(FailJson)?;
    let id: Uuid = response
        .get("id")
        .ok_or(FailJson)?
        .as_str()
        .ok_or(FailJson)?
        .parse()
        .map_err(|e| {
            tracing::warn!(error = &e as &dyn std::error::Error, "Failed to parse UUID");
            FailJson
        })?;
    let name = response
        .get("name")
        .ok_or(FailJson)?
        .as_str()
        .ok_or(FailJson)?
        .to_string();
    let display_name = response
        .get("displayName")
        .ok_or(FailJson)?
        .as_str()
        .ok_or(FailJson)?
        .to_string();
    let bot = response
        .get("bot")
        .ok_or(FailJson)?
        .as_bool()
        .ok_or(FailJson)?;
    let bio = response
        .get("bio")
        .ok_or(FailJson)?
        .as_str()
        .ok_or(FailJson)?;

    let create_user = crate::user::CreateUserParams { name, display_name };
    let user = user_service
        .create_user(create_user)
        .await
        .map_err(IntoStatus::into_status)?;

    sqlx::query(
        r#"
            INSERT INTO `traq_users` (`id`, `user_id`, `bot`, `bio`)
            VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(user.id.0)
    .bind(bot)
    .bind(bio)
    .execute(pool)
    .await?;
    tracing::debug!(traq_user_id = %id, user_id = %user.id.0, "Registered a traQ user");

    find_traq_user(user_service, pool, super::TraqUserId(id))
        .await?
        .ok_or(super::Error::NotFound)
}
