use futures::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

use crate::prelude::IntoStatus;

const RECEIVE_RANGE: u32 = 100;

impl<Context> super::SpeakerPhoneService<Context> for super::SpeakerPhoneServiceImpl
where
    Context: AsRef<MySqlPool>
        + AsRef<crate::task::TaskManager>
        + crate::event::ProvideEventService
        + crate::traq::channel::ProvideTraqChannelService,
{
    type Error = super::Error;

    fn get_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetSpeakerPhoneParams,
    ) -> futures::future::BoxFuture<'a, Result<super::SpeakerPhone, Self::Error>> {
        get_speaker_phone(ctx.as_ref(), params).boxed()
    }

    fn get_speaker_phones_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetSpeakerPhonesInAreaParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::SpeakerPhone>, Self::Error>> {
        get_speaker_phones_in_area(ctx.as_ref(), params).boxed()
    }

    fn create_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::CreateSpeakerPhoneParams,
    ) -> futures::future::BoxFuture<'a, Result<super::SpeakerPhone, Self::Error>> {
        create_speaker_phone(ctx, ctx.as_ref(), params).boxed()
    }

    fn load_all_speaker_phones<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::LoadAllSpeakerPhonesParams,
    ) -> futures::future::BoxFuture<'a, Result<(), Self::Error>> {
        load_all_speaker_phones(ctx.as_ref(), ctx.as_ref(), params).boxed()
    }

    fn get_available_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::GetAvailableChannelsParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Channel>, Self::Error>> {
        get_available_channels(ctx, params).boxed()
    }

    fn search_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: super::SearchChannelsParams,
    ) -> futures::future::BoxFuture<'a, Result<Vec<super::Channel>, Self::Error>> {
        search_channels(ctx, params).boxed()
    }
}

// MARK: DB operations

#[derive(Debug, Clone, Hash, Deserialize, Serialize, FromRow)]
struct SpeakerPhoneRow {
    pub id: uuid::Uuid,
    pub position_x: u32,
    pub position_y: u32,
    pub receive_range: u32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<SpeakerPhoneRow> for super::SpeakerPhone {
    fn from(value: SpeakerPhoneRow) -> Self {
        Self {
            id: super::SpeakerPhoneId(value.id),
            position: crate::world::Coordinate {
                x: value.position_x,
                y: value.position_y,
            },
            receive_range: value.receive_range,
            name: super::Channel(value.name),
            created_at: super::Timestamp(value.created_at),
            updated_at: super::Timestamp(value.updated_at),
        }
    }
}

async fn get_speaker_phone(
    pool: &MySqlPool,
    params: super::GetSpeakerPhoneParams,
) -> Result<super::SpeakerPhone, super::Error> {
    let super::GetSpeakerPhoneParams {
        id: super::SpeakerPhoneId(id),
    } = params;
    let speaker_phone: Option<SpeakerPhoneRow> =
        sqlx::query_as(r#"SELECT * FROM `speaker_phones` WHERE `id` = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
    speaker_phone
        .map(|row| row.into())
        .ok_or(super::Error::NotFound)
}

async fn get_speaker_phones_in_area(
    pool: &MySqlPool,
    params: super::GetSpeakerPhonesInAreaParams,
) -> Result<Vec<super::SpeakerPhone>, super::Error> {
    let super::GetSpeakerPhonesInAreaParams { center, size } = params;
    // TODO: SpeakerPhoneの中央はAreaにないが範囲が被っているやつも含めたいね
    let speaker_phones: Vec<SpeakerPhoneRow> = sqlx::query_as(
        r#"
            SELECT * FROM `speaker_phones`
            WHERE
                `position_x` BETWEEN ? AND ?
                AND `position_y` BETWEEN ? AND ?
        "#,
    )
    .bind(center.x.saturating_sub(size.width / 2))
    .bind(center.x.saturating_add(size.width / 2))
    .bind(center.y.saturating_sub(size.height / 2))
    .bind(center.y.saturating_add(size.height / 2))
    .fetch_all(pool)
    .await?;
    Ok(speaker_phones.into_iter().map(Into::into).collect())
}

async fn create_speaker_phone(
    event_service: &impl crate::event::ProvideEventService,
    pool: &MySqlPool,
    params: super::CreateSpeakerPhoneParams,
) -> Result<super::SpeakerPhone, super::Error> {
    let super::CreateSpeakerPhoneParams { position, name } = params;
    let speaker_phone = SpeakerPhoneRow {
        id: uuid::Uuid::now_v7(),
        position_x: position.x,
        position_y: position.y,
        receive_range: RECEIVE_RANGE,
        name,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    sqlx::query(
        r#"
            INSERT INTO `speaker_phones` (`id`, `position_x`, `position_y`, `receive_range`, `name`, `created_at`, `updated_at`)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(speaker_phone.id)
    .bind(speaker_phone.position_x)
    .bind(speaker_phone.position_y)
    .bind(speaker_phone.receive_range)
    .bind(speaker_phone.name)
    .bind(speaker_phone.created_at)
    .bind(speaker_phone.updated_at)
    .execute(pool)
    .await?;
    tracing::info!(id = %speaker_phone.id, "Created a speaker phone");
    let speaker_phone = get_speaker_phone(
        pool,
        super::GetSpeakerPhoneParams {
            id: super::SpeakerPhoneId(speaker_phone.id),
        },
    )
    .await?;

    event_service
        .publish_event(crate::event::Event::SpeakerPhone(speaker_phone.clone()))
        .await
        .map_err(crate::prelude::IntoStatus::into_status)?;

    Ok(speaker_phone)
}

async fn load_all_speaker_phones(
    pool: &MySqlPool,
    task: &crate::task::TaskManager,
    _params: super::LoadAllSpeakerPhonesParams,
) -> Result<(), super::Error> {
    let _speaker_phones: Vec<SpeakerPhoneRow> = sqlx::query_as(r#"SELECT * FROM `speaker_phones`"#)
        .fetch_all(pool)
        .await?;
    task.spawn(|_cancellation_token| async {
        todo!("messagesをsubscribeしてtraQに投げる");
    })
    .await;
    Ok(())
}

async fn get_available_channels(
    traq_channel_service: &impl crate::traq::channel::ProvideTraqChannelService,
    _params: super::GetAvailableChannelsParams,
) -> Result<Vec<super::Channel>, super::Error> {
    let params = crate::traq::channel::GetAllChannelsParams {};
    let channels = traq_channel_service
        .get_all_channels(params)
        .await
        .map_err(IntoStatus::into_status)?
        .into_iter()
        .map(|channel| super::Channel(channel.path))
        .collect();
    Ok(channels)
}

async fn search_channels(
    traq_channel_service: &impl crate::traq::channel::ProvideTraqChannelService,
    params: super::SearchChannelsParams,
) -> Result<Vec<super::Channel>, super::Error> {
    let super::SearchChannelsParams { name } = params;
    let params = crate::traq::channel::GetAllChannelsParams {};
    let channels = traq_channel_service
        .get_all_channels(params)
        .await
        .map_err(IntoStatus::into_status)?
        .into_iter()
        .map(|channel| super::Channel(channel.path));
    let hits = channels
        .into_iter()
        .filter(|channel| channel.0.contains(&name))
        .collect();
    Ok(hits)
}
