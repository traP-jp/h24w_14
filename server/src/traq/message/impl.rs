use chrono::{DateTime, Utc};
use futures::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

use crate::{prelude::IntoStatus, traq::TraqHost};

impl<Context> super::TraqMessageService<Context> for super::TraqMessageServiceImpl
where
    Context: AsRef<MySqlPool> + AsRef<TraqHost> + crate::traq::auth::ProvideTraqAuthService,
{
    type Error = super::Error;

    fn send_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SendMessageParams,
    ) -> BoxFuture<'a, Result<super::SyncedTraqMessage, Self::Error>> {
        send_message(ctx, ctx.as_ref(), ctx.as_ref(), params).boxed()
    }

    fn check_message_synced<'a>(
        &'a self,
        ctx: &'a Context,
        message: crate::message::Message,
    ) -> BoxFuture<'a, Result<Option<super::SyncedTraqMessage>, Self::Error>> {
        check_message_synced(ctx.as_ref(), message).boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, FromRow)]
struct TraqMessageRow {
    pub id: Uuid,
    pub message_id: Uuid,
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[tracing::instrument(skip_all)]
async fn send_message(
    traq_auth_service: &impl crate::traq::auth::ProvideTraqAuthService,
    traq_host: &crate::traq::TraqHost,
    pool: &MySqlPool,
    params: super::SendMessageParams,
) -> Result<super::SyncedTraqMessage, super::Error> {
    let super::SendMessageParams {
        inner: message,
        channel_id,
        user_id,
    } = params;

    let authorized_user = traq_auth_service
        .check_authorized(user_id)
        .await
        .map_err(IntoStatus::into_status)?
        .ok_or(super::Error::Unauthorized)?;

    let channel_id = channel_id.0;
    let uri = format!("https://{traq_host}/api/v3/channels/{channel_id}/messages");
    let params = crate::traq::auth::BuildRequestAsAuthorizedUserParams {
        user: &authorized_user,
        uri: &uri,
        method: http::Method::POST,
    };
    let request = traq_auth_service
        .build_request_as_authorized_user(params)
        .await
        .map_err(IntoStatus::into_status)?
        .json(&serde_json::json!({
            "content": message.content,
            "embed": true,
        }));
    let response: serde_json::Value = request.send().await?.error_for_status()?.json().await?;
    tracing::trace!(value = ?response, "Message sent");

    let response = response
        .as_object()
        .ok_or(super::Error::UnexpectedResponseFromTraq)?;
    let id: Uuid = response["id"]
        .as_str()
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .parse()
        .map_err(|_| super::Error::UnexpectedResponseFromTraq)?;
    let user_id: Uuid = response["userId"]
        .as_str()
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .parse()
        .map_err(|_| super::Error::UnexpectedResponseFromTraq)?;
    let content = response["content"]
        .as_str()
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .to_string();

    sqlx::query(
        r#"
            INSERT INTO `traq_messages` (
                `id`,
                `message_id`,
                `channel_id`,
                `user_id`,
                `content`
            ) VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(message.id.0)
    .bind(channel_id)
    .bind(user_id)
    .bind(content)
    .execute(pool)
    .await?;

    Ok(super::SyncedTraqMessage {
        id: crate::traq::message::TraqMessageId(id),
        inner: message,
        channel_id: crate::traq::channel::TraqChannelId(channel_id),
        user_id: crate::traq::user::TraqUserId(user_id),
    })
}

#[tracing::instrument(skip_all)]
async fn check_message_synced(
    pool: &MySqlPool,
    message: crate::message::Message,
) -> Result<Option<super::SyncedTraqMessage>, super::Error> {
    let message_id = message.id.0;
    let row: Option<TraqMessageRow> =
        sqlx::query_as(r#"SELECT * FROM `traq_messages` WHERE `message_id` = ?"#)
            .bind(message_id)
            .fetch_optional(pool)
            .await?;
    let row = row.map(|row| super::SyncedTraqMessage {
        id: crate::traq::message::TraqMessageId(row.id),
        inner: message,
        channel_id: crate::traq::channel::TraqChannelId(row.channel_id),
        user_id: crate::traq::user::TraqUserId(row.user_id),
    });
    Ok(row)
}
