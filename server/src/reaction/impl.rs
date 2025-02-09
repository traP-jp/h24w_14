use futures::{future, FutureExt};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::prelude::IntoStatus;

impl<Context> super::ReactionService<Context> for super::ReactionServiceImpl
where
    Context:
        AsRef<MySqlPool> + crate::event::ProvideEventService + crate::world::ProvideWorldService,
{
    type Error = super::Error;

    fn get_reaction<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetReactionParams,
    ) -> future::BoxFuture<'a, Result<super::Reaction, Self::Error>> {
        get_reaction(ctx.as_ref(), params).boxed()
    }

    fn get_reactions_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetReactionsInAreaParams,
    ) -> future::BoxFuture<'a, Result<Vec<super::Reaction>, Self::Error>> {
        get_reactions_in_area(ctx.as_ref(), params).boxed()
    }

    fn create_reaction<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CreateReactionParams,
    ) -> future::BoxFuture<'a, Result<super::Reaction, Self::Error>> {
        create_reaction(ctx, ctx, ctx.as_ref(), params).boxed()
    }
}

// MARK: DB operations

#[derive(Debug, Clone, Hash, Deserialize, Serialize, sqlx::FromRow)]
struct ReactionRow {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub position_x: u32,
    pub position_y: u32,
    pub kind: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl From<ReactionRow> for super::Reaction {
    fn from(value: ReactionRow) -> Self {
        Self {
            id: super::ReactionId(value.id),
            user_id: crate::user::UserId(value.user_id),
            position: crate::world::Coordinate {
                x: value.position_x,
                y: value.position_y,
            },
            kind: value.kind,
            created_at: super::Timestamp(value.created_at),
            updated_at: super::Timestamp(value.updated_at),
            expires_at: super::Timestamp(value.expires_at),
        }
    }
}

async fn get_reaction(
    pool: &MySqlPool,
    params: super::GetReactionParams,
) -> Result<super::Reaction, super::Error> {
    let super::GetReactionParams {
        id: super::ReactionId(id),
    } = params;
    let reaction: Option<ReactionRow> =
        sqlx::query_as(r#"SELECT * FROM `reactions` WHERE `id` = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
    reaction.map(Into::into).ok_or(super::Error::NotFound)
}

async fn get_reactions_in_area(
    pool: &MySqlPool,
    params: super::GetReactionsInAreaParams,
) -> Result<Vec<super::Reaction>, super::Error> {
    let super::GetReactionsInAreaParams { center, size } = params;
    let reactions: Vec<ReactionRow> = sqlx::query_as(
        r#"
            SELECT * FROM `reactions`
            WHERE
                `position_x` BETWEEN ? AND ?
            AND
                `position_y` BETWEEN ? AND ?
            AND
                `expires_at` > NOW()
        "#,
    )
    .bind(center.x.saturating_sub(size.width / 2))
    .bind(center.x.saturating_add(size.width / 2))
    .bind(center.y.saturating_sub(size.height / 2))
    .bind(center.y.saturating_add(size.height / 2))
    .fetch_all(pool)
    .await?;
    Ok(reactions.into_iter().map(Into::into).collect())
}

async fn create_reaction<
    P: crate::event::ProvideEventService,
    W: crate::world::ProvideWorldService,
>(
    event_service: &P,
    world_service: &W,
    pool: &MySqlPool,
    params: super::CreateReactionParams,
) -> Result<super::Reaction, super::Error> {
    let super::CreateReactionParams {
        user_id,
        position,
        kind,
    } = params;

    // check if the position is in the world
    let crate::world::CheckCoordinateAnswer::Valid(position) = world_service
        .check_coordinate(crate::world::CheckCoordinateParams {
            coordinate: position,
        })
        .await
        .map_err(IntoStatus::into_status)?
    else {
        return Err(super::Error::ReactionNotInWorld);
    };

    let reaction = ReactionRow {
        id: uuid::Uuid::now_v7(),
        user_id: user_id.0,
        position_x: position.x,
        position_y: position.y,
        kind,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::seconds(10),
    };
    sqlx::query(
        r#"
            INSERT INTO `reactions`
            (`id`, `user_id`, `position_x`, `position_y`, `kind`, `created_at`, `updated_at`, `expires_at`)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(reaction.id)
    .bind(reaction.user_id)
    .bind(reaction.position_x)
    .bind(reaction.position_y)
    .bind(reaction.kind)
    .bind(reaction.created_at)
    .bind(reaction.updated_at)
    .bind(reaction.expires_at)
    .execute(pool)
    .await?;
    tracing::info!(id = %reaction.id, "Created a reaction");
    let reaction = get_reaction(
        pool,
        super::GetReactionParams {
            id: super::ReactionId(reaction.id),
        },
    )
    .await?;

    event_service
        .publish_event(crate::event::Event::Reaction(reaction.clone()))
        .await
        .map_err(crate::prelude::IntoStatus::into_status)?;

    Ok(reaction)
}
