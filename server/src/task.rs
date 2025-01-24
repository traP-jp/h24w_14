use std::{future::Future, sync::Arc};

use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[derive(Debug, Clone)]
pub struct TaskManager(Arc<Inner>);

#[derive(Debug)]
struct Inner {
    cancel: CancellationToken,
    task_tracker: TaskTracker,
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
            task_tracker: TaskTracker::new(),
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
        self.0.task_tracker.spawn(fut);
        child
    }

    #[tracing::instrument(skip_all)]
    pub async fn graceful_shutdown(&self) {
        self.0.cancel.cancel();
        self.0.task_tracker.close();
        self.0.task_tracker.wait().await;
        tracing::info!("Gracefully shut down");
    }
}
