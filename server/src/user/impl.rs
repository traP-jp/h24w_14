use crate::prelude::Timestamp;
use futures::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

impl<Context> super::UserService<Context> for super::UserServiceImpl
where
    Context: AsRef<MySqlPool>,
{
    type Error = super::Error;

    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::GetUser,
    ) -> BoxFuture<'a, Result<super::User, Self::Error>> {
        get_user(ctx.as_ref(), req).boxed()
    }

    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        req: super::CreateUser,
    ) -> BoxFuture<'a, Result<super::User, Self::Error>> {
        create_user(ctx.as_ref(), req).boxed()
    }
}

// MARK: DB operations

#[derive(Debug, Clone, Hash, Deserialize, Serialize, FromRow)]
struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for super::User {
    fn from(value: UserRow) -> Self {
        Self {
            id: super::UserId(value.id),
            name: value.name,
            display_name: value.display_name,
            updated_at: Timestamp(value.updated_at),
            created_at: Timestamp(value.created_at),
        }
    }
}

async fn get_user(pool: &MySqlPool, request: super::GetUser) -> Result<super::User, super::Error> {
    let super::GetUser {
        id: super::UserId(id),
    } = request;
    let user: Option<UserRow> = sqlx::query_as(r#"SELECT * FROM `users` WHERE `id` = ?"#)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    user.map_or(Err(super::Error::NotFound), |user| Ok(user.into()))
}

async fn create_user(
    pool: &MySqlPool,
    request: super::CreateUser,
) -> Result<super::User, super::Error> {
    let super::CreateUser { name, display_name } = request;
    let id = Uuid::now_v7();
    sqlx::query(
        r#"
            INSERT INTO `users` (`id`, `name`, `display_name`)
            VALUES (?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(display_name)
    .execute(pool)
    .await?;

    let user = get_user(
        pool,
        super::GetUser {
            id: super::UserId(id),
        },
    )
    .await?;
    Ok(user)
}
