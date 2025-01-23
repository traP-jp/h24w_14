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
pub struct GetSpeakerPhone {
    pub id: SpeakerPhoneId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetSpeakerPhonesInArea {
    pub center: crate::world::Coordinate,
    pub size: crate::world::Size,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateSpeakerPhone {
    pub name: String,
    pub position: crate::world::Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetAvailableChannels {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SearchChannels {
    pub name: String,
}

pub trait SpeakerPhoneService<Context>: Send + Sync + 'static {
    type Error: IntoStatus;

    fn get_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetSpeakerPhone,
    ) -> BoxFuture<'a, Result<SpeakerPhone, Self::Error>>;
    fn get_speaker_phones_in_area<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetSpeakerPhonesInArea,
    ) -> BoxFuture<'a, Result<Vec<SpeakerPhone>, Self::Error>>;
    fn create_speaker_phone<'a>(
        &'a self,
        ctx: &'a Context,
        req: CreateSpeakerPhone,
    ) -> BoxFuture<'a, Result<SpeakerPhone, Self::Error>>;
    fn get_available_channels<'a>(
        &'a self,
        ctx: &'a Context,
        req: GetAvailableChannels,
    ) -> BoxFuture<'a, Result<Vec<Channel>, Self::Error>>;
    fn search_channels<'a>(
        &'a self,
        ctx: &'a Context,
        req: SearchChannels,
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
        req: GetSpeakerPhone,
    ) -> BoxFuture<
        '_,
        Result<
            SpeakerPhone,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service().get_speaker_phone(ctx, req)
    }
    fn get_speaker_phones_in_area(
        &self,
        req: GetSpeakerPhonesInArea,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<SpeakerPhone>,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service()
            .get_speaker_phones_in_area(ctx, req)
    }
    fn create_speaker_phone(
        &self,
        req: CreateSpeakerPhone,
    ) -> BoxFuture<
        '_,
        Result<
            SpeakerPhone,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service().create_speaker_phone(ctx, req)
    }
    fn get_available_channels(
        &self,
        req: GetAvailableChannels,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<Channel>,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service()
            .get_available_channels(ctx, req)
    }
    fn search_channels(
        &self,
        req: SearchChannels,
    ) -> BoxFuture<
        '_,
        Result<
            Vec<Channel>,
            <Self::SpeakerPhoneService as SpeakerPhoneService<Self::Context>>::Error,
        >,
    > {
        let ctx = self.context();
        self.speaker_phone_service().search_channels(ctx, req)
    }

    // TODO: build_server(this: Arc<Self>) -> SpeakerPhoneServiceServer<...>
}
