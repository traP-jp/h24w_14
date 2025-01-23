//! `speaker_phone.proto`

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::{IntoStatus, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SpeakerPhoneId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Channel(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SpeakerPhone {
    pub id: SpeakerPhoneId,
    pub position: crate::world::Coordinate,
    pub receive_range: u32,
    // NOTE: reserved fields
    // r#type: SpeakerPhoneType,
    // name_type: SpeakerPhoneNameType,
    pub name: Channel,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetSpeakerPhoneParams {
    pub id: SpeakerPhoneId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetSpeakerPhonesInAreaParams {
    pub center: crate::world::Coordinate,
    pub size: crate::world::Size,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateSpeakerPhoneParams {
    pub name: String,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetAvailableChannelsParams {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SearchChannelsParams {
    pub name: String,
}

pub trait SpeakerPhoneService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetSpeakerPhoneParams,
    ) -> BoxFuture<'a, Result<SpeakerPhone, Self::Error>>;
    fn get_speaker_phones_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetSpeakerPhonesInAreaParams,
    ) -> BoxFuture<'a, Result<Vec<SpeakerPhone>, Self::Error>>;
    fn create_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        params: CreateSpeakerPhoneParams,
    ) -> BoxFuture<'a, Result<SpeakerPhone, Self::Error>>;
    fn get_available_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetAvailableChannelsParams,
    ) -> BoxFuture<'a, Result<Vec<Channel>, Self::Error>>;
    fn search_channels<'a>(
        &'a self,
        ctx: &'a Context,
        params: SearchChannelsParams,
    ) -> BoxFuture<'a, Result<Vec<Channel>, Self::Error>>;
}

#[allow(clippy::type_complexity)]
pub trait ProvideSpeakerPhone: Send + Sync + 'static {
    type Context;
    type SpeakerPhoneService: SpeakerPhoneService<Self::Context>;

    fn context(&self) -> &Self::Context;
    fn speaker_phone_service(&self) -> &Self::SpeakerPhoneService;

    fn get_speaker_phone(
        &self,
        params: GetSpeakerPhoneParams,
    ) -> BoxFuture<
        '_,
        Result<
            SpeakerPhone,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service().get_speaker_phone(ctx, params)
    }
    fn get_speaker_phones_in_area(
        &self,
        params: GetSpeakerPhonesInAreaParams,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<SpeakerPhone>,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service()
            .get_speaker_phones_in_area(ctx, params)
    }
    fn create_speaker_phone(
        &self,
        params: CreateSpeakerPhoneParams,
    ) -> BoxFuture<
        '_,
        Result<
            SpeakerPhone,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service()
            .create_speaker_phone(ctx, params)
    }
    fn get_available_channels(
        &self,
        params: GetAvailableChannelsParams,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<Channel>,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service()
            .get_available_channels(ctx, params)
    }
    fn search_channels(
        &self,
        params: SearchChannelsParams,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<Channel>,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service().search_channels(ctx, params)
    }

    // TODO: build_server(this: Arc<Self>) -> SpeakerPhoneServiceServer<...>
}
