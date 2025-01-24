use std::sync::Arc;

use schema::reaction as schema;

use crate::prelude::IntoStatus;

// MARK: type conversions

impl From<super::Reaction> for schema::Reaction {
    fn from(value: super::Reaction) -> Self {
        let super::Reaction {
            id,
            user_id,
            position,
            kind,
            created_at,
            updated_at: _,
        } = value;
        Self {
            id: id.0.to_string(),
            user_id: user_id.0.to_string(),
            position: Some(position.into()),
            kind,
            created_at: Some(created_at.into()),
            // TODO: duration設定
            expires_at: Some(super::Timestamp(created_at.0 + chrono::Duration::seconds(10)).into()),
        }
    }
}

// MARK: ServiceImpl

pub struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State>
where
    State: super::ProvideReactionService,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<State> ServiceImpl<State>
where
    State: super::ProvideReactionService,
{
    pub(super) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl<State> schema::reaction_service_server::ReactionService for ServiceImpl<State>
where
    State: super::ProvideReactionService + crate::session::ProvideSessionService,
{
    async fn get_reaction(
        &self,
        request: tonic::Request<schema::GetReactionRequest>,
    ) -> Result<tonic::Response<schema::GetReactionResponse>, tonic::Status> {
        let (_, _, schema::GetReactionRequest { id }) = request.into_parts();
        let params = super::GetReactionParams {
            id: super::ReactionId(
                uuid::Uuid::parse_str(&id)
                    .map_err(|_| tonic::Status::invalid_argument("Invalid UUID"))?,
            ),
        };
        let reaction = self
            .state
            .get_reaction(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::GetReactionResponse {
            reaction: Some(reaction),
        };
        Ok(tonic::Response::new(res))
    }

    async fn create_reaction(
        &self,
        request: tonic::Request<schema::CreateReactionRequest>,
    ) -> Result<tonic::Response<schema::CreateReactionResponse>, tonic::Status> {
        let (meta, _, schema::CreateReactionRequest { kind, position }) = request.into_parts();
        let Some(position) = position else {
            return Err(tonic::Status::invalid_argument("Position is required"));
        };

        let header_map = meta.into_headers();
        let user_id = self
            .state
            .extract(crate::session::ExtractParams(&header_map))
            .await
            .map_err(IntoStatus::into_status)?
            .user_id;

        let params = super::CreateReactionParams {
            user_id,
            kind,
            position: position.into(),
        };
        let reaction = self
            .state
            .create_reaction(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::CreateReactionResponse {
            reaction: Some(reaction),
        };
        Ok(tonic::Response::new(res))
    }
}
