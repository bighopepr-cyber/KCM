use kcm_core::types::*;
use tokio::task;

pub struct AsyncExecutor {
    runtime: tokio::runtime::Runtime,
}

impl AsyncExecutor {
    pub fn new() -> Result<Self, KcmError> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| KcmError::Io(e.to_string()))?;

        Ok(AsyncExecutor { runtime })
    }

    pub fn block_on<F>(&self, f: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.runtime.block_on(f)
    }
}

impl Default for AsyncExecutor {
    fn default() -> Self {
        Self::new().expect("AsyncExecutor::new() should not fail with valid system config")
    }
}

pub async fn async_insert(
    db: std::sync::Arc<parking_lot::Mutex<crate::database::KnowledgeDatabase>>,
    fact: Fact,
) -> Result<RowID, KcmError> {
    task::spawn_blocking(move || db.lock().insert(&fact))
        .await
        .map_err(|e| KcmError::Io(e.to_string()))?
}

pub async fn async_query_all(
    db: std::sync::Arc<parking_lot::Mutex<crate::database::KnowledgeDatabase>>,
) -> Result<Vec<Fact>, KcmError> {
    task::spawn_blocking(move || db.lock().query().execute())
        .await
        .map_err(|e| KcmError::Io(e.to_string()))?
}

pub async fn async_fact_count(
    db: std::sync::Arc<parking_lot::Mutex<crate::database::KnowledgeDatabase>>,
) -> usize {
    task::spawn_blocking(move || db.lock().fact_count())
        .await
        .unwrap_or(0)
}
