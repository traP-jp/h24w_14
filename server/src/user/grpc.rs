use std::sync::Arc;

use schema::user as schema;

use crate::prelude::IntoStatus;
use crate::session::ProvideSessionService;

// MARK: type conversions

impl From<super::User> for schema::User {
    fn from(value: super::User) -> Self {
        let super::User {
            id,
            name,
            display_name,
            created_at,
            updated_at: _,
        } = value;
        Self {
            id: id.0.to_string(),
            name,
            display_name,
            created_at: Some(created_at.into()),
        }
    }
}

// MARK: ServiceImpl

pub struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State>
where
    State: super::ProvideUserService,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<State> ServiceImpl<State>
where
    State: super::ProvideUserService,
{
    pub(super) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait::async_trait]
impl<State> schema::user_service_server::UserService for ServiceImpl<State>
where
    State: super::ProvideUserService + ProvideSessionService,
{
    async fn get_user(
        &self,
        request: tonic::Request<schema::GetUserRequest>,
    ) -> Result<tonic::Response<schema::GetUserResponse>, tonic::Status> {
        let (_, _, schema::GetUserRequest { id }) = request.into_parts();
        let params = super::GetUserParams {
            id: super::UserId(
                uuid::Uuid::parse_str(&id)
                    .map_err(|_| tonic::Status::invalid_argument("Invalid UUID"))?,
            ),
        };
        let user = self
            .state
            .get_user(params)
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::GetUserResponse { user: Some(user) };
        Ok(tonic::Response::new(res))
    }

    async fn get_me(
        &self,
        request: tonic::Request<schema::GetMeRequest>,
    ) -> Result<tonic::Response<schema::GetMeResponse>, tonic::Status> {
        let (meta, _, schema::GetMeRequest {}) = request.into_parts();
        let headers = meta.into_headers();
        let session = self
            .state
            .extract(crate::session::ExtractParams(&headers))
            .await
            .map_err(IntoStatus::into_status)?;
        let user = self
            .state
            .get_user(super::GetUserParams {
                id: session.user_id,
            })
            .await
            .map_err(IntoStatus::into_status)?
            .into();
        let res = schema::GetMeResponse { user: Some(user) };
        Ok(tonic::Response::new(res))
    }
}
