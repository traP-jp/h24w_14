use crate::prelude::Timestamp;
use chrono::{DateTime, Utc};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

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

impl<Context> super::MessageService<Context> for super::MessageServiceImpl
where
    Context: AsRef<MySqlPool>,
{
    type Error = super::Error;

    fn get_message<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetMessageParams,
    ) -> futures::future::BoxFuture<'a, Result<super::Message, Self::Error>> {
        let pool = ctx.as_ref();
        get_message(pool, req.id.0).boxed()
    }

    fn get_messages_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetMessagesInAreaParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Message>, Self::Error>> {
        let pool = ctx.as_ref();
        get_messages_in_area(pool, req).boxed()
    }

    fn create_message<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::CreateMessageParams,
    ) -> futures::future::BoxFuture<'a, Result<super::Message, Self::Error>> {
        let pool = ctx.as_ref();
        create_message(pool, req).boxed()
    }
}

async fn get_message(pool: &MySqlPool, id: Uuid) -> Result<super::Message, super::Error> {
    sqlx::query_as::<_, MessageRow>("SELECT * FROM `messages` WHERE `id` = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map(|row| row.into())
        .map_err(super::Error::Sqlx)
}

async fn get_messages_in_area(
    pool: &MySqlPool,
    req: super::GetMessagesInAreaParams,
) -> Result<Vec<super::Message>, super::Error> {
    sqlx::query_as::<_, MessageRow>(
        "SELECT * FROM `messages` WHERE `position_x` BETWEEN ? AND ? AND `position_y` BETWEEN ? AND ? ORDER BY `created_at` DESC",
    )
        .bind(req.center.x.saturating_sub(req.size.width) as i32)
        .bind(req.center.x.saturating_add(req.size.width) as i32)
        .bind(req.center.y.saturating_sub(req.size.height) as i32)
        .bind(req.center.y.saturating_add(req.size.height) as i32)
        .fetch_all(pool)
        .await
        .map(|rows| rows.into_iter().map(|row| row.into()).collect())
        .map_err(super::Error::Sqlx)
}

async fn create_message(
    pool: &MySqlPool,
    req: super::CreateMessageParams,
) -> Result<super::Message, super::Error> {
    let id = Uuid::now_v7();
    sqlx::query("INSERT INTO `messages` (`id`, `user_id`, `content`, `position_x`, `position_y`, `expires_at`) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(id)
        .bind(req.user_id.0)
        .bind(req.content)
        .bind(req.position.x)
        .bind(req.position.y)
        .execute(pool)
        .await
        .map_err(super::Error::Sqlx)?;

    sqlx::query_as::<_, MessageRow>("SELECT * FROM `messages` WHERE `id` = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map(|row| row.into())
        .map_err(super::Error::Sqlx)
}
