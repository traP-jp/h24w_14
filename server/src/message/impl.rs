use crate::prelude::Timestamp;
use chrono::{DateTime, Utc};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

// TODO: 値はてきとう
fn calculate_expires_at(created_at: DateTime<Utc>) -> DateTime<Utc> {
    created_at + chrono::Duration::days(1)
}

impl<Context> super::MessageService<Context> for super::MessageServiceImpl
where
    Context: AsRef<MySqlPool> + crate::event::ProvideEventService,
{
    type Error = super::Error;

    fn get_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetMessageParams,
    ) -> futures::future::BoxFuture<'a, Result<super::Message, Self::Error>> {
        let pool = ctx.as_ref();
        get_message(pool, params).boxed()
    }

    fn get_messages_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetMessagesInAreaParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Message>, Self::Error>> {
        let pool = ctx.as_ref();
        get_messages_in_area(pool, params).boxed()
    }

    fn create_message<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CreateMessageParams,
    ) -> futures::future::BoxFuture<'a, Result<super::Message, Self::Error>> {
        let event_service = ctx;
        let pool = ctx.as_ref();
        create_message(event_service, pool, params).boxed()
    }
}

// MARK: DB operations

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct MessageRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub position_x: u32,
    pub position_y: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
impl From<MessageRow> for super::Message {
    fn from(row: MessageRow) -> Self {
        Self {
            id: super::MessageId(row.id),
            user_id: crate::user::UserId(row.user_id),
            position: crate::world::Coordinate {
                x: row.position_x,
                y: row.position_y,
            },
            content: row.content,
            created_at: Timestamp(row.created_at),
            updated_at: Timestamp(row.updated_at),
            expires_at: Timestamp(row.expires_at),
        }
    }
}

async fn get_message(
    pool: &MySqlPool,
    params: super::GetMessageParams,
) -> Result<super::Message, super::Error> {
    let super::GetMessageParams { id } = params;
    sqlx::query_as::<_, MessageRow>("SELECT * FROM `messages` WHERE `id` = ?")
        .bind(id.0)
        .fetch_optional(pool)
        .await
        .map_err(super::Error::Sqlx)?
        .ok_or(super::Error::NotFound)
        .map(|row| row.into())
}

async fn get_messages_in_area(
    pool: &MySqlPool,
    params: super::GetMessagesInAreaParams,
) -> Result<Vec<super::Message>, super::Error> {
    sqlx::query_as::<_, MessageRow>(
        r#"
            SELECT * FROM `messages`
            WHERE
                `position_x` BETWEEN ? AND ?
            AND
                `position_y` BETWEEN ? AND ?
            AND
                `expires_at` > NOW()
            ORDER BY `created_at` DESC
        "#,
    )
    .bind(params.center.x.saturating_sub(params.size.width / 2) as i32)
    .bind(params.center.x.saturating_add(params.size.width / 2) as i32)
    .bind(params.center.y.saturating_sub(params.size.height / 2) as i32)
    .bind(params.center.y.saturating_add(params.size.height / 2) as i32)
    .fetch_all(pool)
    .await
    .map(|rows| rows.into_iter().map(|row| row.into()).collect())
    .map_err(super::Error::Sqlx)
}

async fn create_message<P: crate::event::ProvideEventService>(
    event_service: &P,
    pool: &MySqlPool,
    params: super::CreateMessageParams,
) -> Result<super::Message, super::Error> {
    let id = Uuid::now_v7();
    sqlx::query("INSERT INTO `messages` (`id`, `user_id`, `content`, `position_x`, `position_y`, `expires_at`) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(id)
        .bind(params.user_id.0)
        .bind(params.content)
        .bind(params.position.x)
        .bind(params.position.y)
        .bind(calculate_expires_at(Utc::now()))
        .execute(pool)
        .await
        .map_err(super::Error::Sqlx)?;

    let message = get_message(
        pool,
        super::GetMessageParams {
            id: super::MessageId(id),
        },
    )
    .await?;

    event_service
        .publish_event(crate::event::Event::Message(message.clone()))
        .await
        .map_err(crate::prelude::IntoStatus::into_status)?;

    Ok(message)
}
