use std::sync::Arc;

use schema::speaker_phone as schema;

use crate::prelude::IntoStatus;

// MARK: type conversions

impl From<super::SpeakerPhone> for schema::SpeakerPhone {
    fn from(value: super::SpeakerPhone) -> Self {
        let super::SpeakerPhone {
            id,
            position,
            receive_range,
            name,
            created_at,
            updated_at,
        } = value;
        Self {
            id: id.0.to_string(),
            position: Some(position.into()),
            receive_range,
            name: name.0,
            created_at: Some(created_at.into()),
            updated_at: Some(updated_at.into()),
        }
    }
}

// MARK: ServiceImpl

pub struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State>
where
    State: super::ProvideSpeakerPhoneService,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<State> ServiceImpl<State>
where
    State: super::ProvideSpeakerPhoneService + crate::session::ProvideSessionService,
{
    pub(super) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl<State> schema::speaker_phone_service_server::SpeakerPhoneService for ServiceImpl<State>
where
    State: super::ProvideSpeakerPhoneService + crate::session::ProvideSessionService,
{
    async fn get_speaker_phone(
        &self,
        request: tonic::Request<schema::GetSpeakerPhoneRequest>,
    ) -> Result<tonic::Response<schema::GetSpeakerPhoneResponse>, tonic::Status> {
        let (_, _, schema::GetSpeakerPhoneRequest { id }) = request.into_parts();
        let params = super::GetSpeakerPhoneParams {
            id: super::SpeakerPhoneId(
                uuid::Uuid::parse_str(&id)
                    .map_err(|_| tonic::Status::invalid_argument("Invalid UUID"))?,
            ),
        };
        let sperker_phone = self
            .state
            .get_speaker_phone(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::GetSpeakerPhoneResponse {
            speaker_phone: Some(sperker_phone),
        };
        Ok(tonic::Response::new(res))
    }

    async fn create_speaker_phone(
        &self,
        request: tonic::Request<schema::CreateSpeakerPhoneRequest>,
    ) -> Result<tonic::Response<schema::CreateSpeakerPhoneResponse>, tonic::Status> {
        let (_, _, schema::CreateSpeakerPhoneRequest { position, name }) = request.into_parts();
        let Some(position) = position else {
            return Err(tonic::Status::invalid_argument("Position is required"));
        };

        let params = super::CreateSpeakerPhoneParams {
            name,
            position: position.into(),
        };
        let speaker_phone = self
            .state
            .create_speaker_phone(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::CreateSpeakerPhoneResponse {
            speaker_phone: Some(speaker_phone),
        };
        Ok(tonic::Response::new(res))
    }

    async fn get_available_channels(
        &self,
        _request: tonic::Request<schema::GetAvailableChannelsRequest>,
    ) -> Result<tonic::Response<schema::GetAvailableChannelsResponse>, tonic::Status> {
        let params = super::GetAvailableChannelsParams {};
        let channels = self
            .state
            .get_available_channels(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into_iter()
            .map(|channel| channel.0)
            .collect();
        let res = schema::GetAvailableChannelsResponse { channels };
        Ok(tonic::Response::new(res))
    }

    async fn search_channels(
        &self,
        request: tonic::Request<schema::SearchChannelsRequest>,
    ) -> Result<tonic::Response<schema::SearchChannelsResponse>, tonic::Status> {
        let (_, _, schema::SearchChannelsRequest { name }) = request.into_parts();
        let params = super::SearchChannelsParams { name };
        let hits = self
            .state
            .search_channels(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into_iter()
            .map(|channel| channel.0)
            .collect();
        let res = schema::SearchChannelsResponse { hits };
        Ok(tonic::Response::new(res))
    }
}
