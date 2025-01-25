use std::sync::Arc;

use crate::prelude::IntoStatus;

pub struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State>
where
    State: super::ProvideTraqAuthService,
{
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

#[async_trait::async_trait]
impl<State> schema::auth::auth_service_server::AuthService for ServiceImpl<State>
where
    State: super::ProvideTraqAuthService + crate::session::ProvideSessionService,
{
    async fn auth(
        &self,
        _request: tonic::Request<schema::auth::AuthRequest>,
    ) -> Result<tonic::Response<schema::auth::AuthResponse>, tonic::Status> {
        let uri = self
            .state
            .oauth2_entrypoint_uri(super::OAuth2EntrypointUriParams {})
            .await
            .map_err(IntoStatus::into_status)?;

        let res = schema::auth::AuthResponse { location: uri };

        Ok(tonic::Response::new(res))
    }
}
