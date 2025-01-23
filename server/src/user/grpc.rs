use std::sync::Arc;

use schema::user as schema;

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
    State: super::ProvideUserService,
{
    async fn get_user(
        &self,
        request: tonic::Request<schema::GetUserRequest>,
    ) -> Result<tonic::Response<schema::GetUserResponse>, tonic::Status> {
        unimplemented!()
    }
}
