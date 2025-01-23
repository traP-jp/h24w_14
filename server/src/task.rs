use std::{future::Future, sync::Arc};

use tokio::{
    sync::Mutex,
    task::{JoinError, JoinSet},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct TaskManager(Arc<Inner>);

#[derive(Debug)]
struct Inner {
    cancel: CancellationToken,
    join_set: Mutex<JoinSet<()>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    pub fn new() -> Self {
        let inner = Inner {
            cancel: CancellationToken::new(),
            join_set: Mutex::new(JoinSet::new()),
        };
        Self(Arc::new(inner))
    }

    pub async fn spawn<F, Fut>(&self, func: F) -> CancellationToken
    where
        F: FnOnce(CancellationToken) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let child = self.0.cancel.child_token();
        let fut = func(child.clone());
        self.0.join_set.lock().await.spawn(fut);
        child
    }

    #[tracing::instrument(skip_all)]
    pub async fn graceful_shutdown(&self) -> Result<(), JoinError> {
        self.0.cancel.cancel();
        let mut join_set = self.0.join_set.lock().await;
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(()) => {}
                Err(e) => {
                    tracing::error!(error = &e as &dyn std::error::Error, "join error");
                    return Err(e);
                }
            }
        }
        tracing::info!("Gracefully shut down");
        Ok(())
    }
}
