use chrono::{DateTime, Utc};
use futures::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

use crate::{prelude::IntoStatus, traq::TraqHost};

impl<Context> super::TraqMessageService<Context> for super::TraqMessageServiceImpl
where
    Context: AsRef<MySqlPool>
        + AsRef<TraqHost>
        + crate::message::ProvideMessageService
        + crate::traq::auth::ProvideTraqAuthService,
{
    type Error = super::Error;

    fn send_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SendMessageParams,
    ) -> BoxFuture<'a, Result<super::SyncedTraqMessage, Self::Error>> {
        send_message(ctx, ctx.as_ref(), ctx.as_ref(), params).boxed()
    }

    fn recv_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::RecvMessageParams,
    ) -> BoxFuture<'a, Result<super::SyncedTraqMessage, Self::Error>> {
        recv_message(ctx, ctx.as_ref(), params).boxed()
    }

    fn check_message_sent<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CheckMessageSentParams,
    ) -> BoxFuture<'a, Result<Option<super::SyncedTraqMessage>, Self::Error>> {
        let super::CheckMessageSentParams { message } = params;
        check_message_sent(ctx.as_ref(), message).boxed()
    }

    fn check_message_received<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CheckMessageReceivedParams,
    ) -> BoxFuture<'a, Result<Option<super::SyncedTraqMessage>, Self::Error>> {
        let super::CheckMessageReceivedParams { traq_message } = params;
        check_message_received(ctx, ctx.as_ref(), traq_message).boxed()
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
    let id: Uuid = response
        .get("id")
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .as_str()
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .parse()
        .map_err(|_| super::Error::UnexpectedResponseFromTraq)?;
    let user_id: Uuid = response
        .get("userId")
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .as_str()
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
        .parse()
        .map_err(|_| super::Error::UnexpectedResponseFromTraq)?;
    let content = response
        .get("content")
        .ok_or(super::Error::UnexpectedResponseFromTraq)?
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
async fn recv_message(
    message_service: &impl crate::message::ProvideMessageService,
    pool: &MySqlPool,
    params: super::RecvMessageParams,
) -> Result<super::SyncedTraqMessage, super::Error> {
    let super::RecvMessageParams {
        traq_message,
        user_id,
        position,
    } = params;
    let inner = message_service
        .create_message(crate::message::CreateMessageParams {
            user_id,
            position,
            content: traq_message.content,
        })
        .await
        .map_err(|e| {
            tracing::error!(
                error = &e as &dyn std::error::Error,
                "Failed to create app message"
            );
            e.into_status()
        })?;

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
    .bind(traq_message.id.0)
    .bind(inner.id.0)
    .bind(traq_message.channel_id.0)
    .bind(user_id.0)
    .bind(&inner.content)
    .execute(pool)
    .await?;
    tracing::trace!(
        traq_message_id = ?traq_message.id,
        message_id = ?inner.id,
        "Reflected traQ message to app",
    );

    let synced_message = super::SyncedTraqMessage {
        id: traq_message.id,
        channel_id: traq_message.channel_id,
        user_id: traq_message.user_id,
        inner,
    };
    Ok(synced_message)
}

#[tracing::instrument(skip_all)]
async fn check_message_received(
    message_service: &impl crate::message::ProvideMessageService,
    pool: &MySqlPool,
    traq_message: super::TraqMessage,
) -> Result<Option<super::SyncedTraqMessage>, super::Error> {
    let row: Option<TraqMessageRow> =
        sqlx::query_as(r#"SELECT * FROM `traq_messages` WHERE `id` = ?"#)
            .bind(traq_message.id.0)
            .fetch_optional(pool)
            .await?;
    let Some(row) = row else {
        return Ok(None);
    };
    let inner = message_service
        .get_message(crate::message::GetMessageParams {
            id: crate::message::MessageId(row.message_id),
        })
        .await
        .map_err(|e| {
            tracing::error!(
                error = &e as &dyn std::error::Error,
                "Failed to get app message"
            );
            e.into_status()
        })?;
    let synced_message = super::SyncedTraqMessage {
        id: super::TraqMessageId(row.id),
        channel_id: crate::traq::channel::TraqChannelId(row.channel_id),
        user_id: crate::traq::user::TraqUserId(row.user_id),
        inner,
    };
    Ok(Some(synced_message))
}

#[tracing::instrument(skip_all)]
async fn check_message_sent(
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
