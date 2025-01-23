use std::sync::Arc;

use uuid::Uuid;

use crate::prelude::IntoStatus;

// MARK: type conversions

impl From<super::Message> for schema::msg::Message {
    fn from(value: super::Message) -> Self {
        let super::Message {
            id,
            user_id,
            position,
            content,
            created_at,
            updated_at,
            expires_at,
        } = value;
        Self {
            id: id.0.to_string(),
            user_id: user_id.0.to_string(),
            position: Some(position.into()),
            content,
            created_at: Some(created_at.into()),
            updated_at: Some(updated_at.into()),
            expires_at: Some(expires_at.into()),
        }
    }
}

// MARK: ServiceImpl

pub struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State>
where
    State: super::ProvideMessageService,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<State> ServiceImpl<State>
where
    State: super::ProvideMessageService + crate::session::ProvideSessionService,
{
    pub(super) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl<State> schema::msg::message_service_server::MessageService for ServiceImpl<State>
where
    State: super::ProvideMessageService + crate::session::ProvideSessionService,
{
    async fn get_message(
        &self,
        request: tonic::Request<schema::msg::GetMessageRequest>,
    ) -> Result<tonic::Response<schema::msg::GetMessageResponse>, tonic::Status> {
        let (_, _, schema::msg::GetMessageRequest { id }) = request.into_parts();
        let params = super::GetMessageParams {
            id: super::MessageId(
                Uuid::parse_str(&id)
                    .map_err(|_| tonic::Status::invalid_argument("Invalid UUID"))?,
            ),
        };
        let message = self
            .state
            .get_message(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::msg::GetMessageResponse {
            message: Some(message),
        };
        Ok(tonic::Response::new(res))
    }

    async fn create_message(
        &self,
        request: tonic::Request<schema::msg::CreateMessageRequest>,
    ) -> Result<tonic::Response<schema::msg::CreateMessageResponse>, tonic::Status> {
        let (
            meta,
            _,
            schema::msg::CreateMessageRequest {
                content,
                position: Some(position),
            },
        ) = request.into_parts()
        else {
            return Err(tonic::Status::invalid_argument("Invalid request"));
        };

        // TODO: ProvideSesionService と合わせてユーザーとってくる
        let header_map = meta.into_headers();
        let user_id = self
            .state
            .extract(crate::session::ExtractParams(&header_map))
            .await
            .map_err(IntoStatus::into_status)?
            .user_id;

        let params = super::CreateMessageParams {
            user_id,
            position: position.into(),
            content,
        };

        let message = self
            .state
            .create_message(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::msg::CreateMessageResponse {
            message: Some(message),
        };

        Ok(tonic::Response::new(res))
    }
}
