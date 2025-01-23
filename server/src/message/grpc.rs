use std::sync::Arc;

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
    State: super::ProvideMessageService,
{
    pub(super) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}
