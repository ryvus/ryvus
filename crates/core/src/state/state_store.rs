use crate::prelude::ExecutionResult;
use async_trait::async_trait;

#[async_trait]
pub trait StateStore: Send + Sync {
    async fn save_result(&self, result: &ExecutionResult) -> Result<(), String>;

    async fn load_result(&self, run_id: &str) -> Result<Option<ExecutionResult>, String>;

    async fn update_step(
        &self,
        run_id: &str,
        step_name: &str,
        data: serde_json::Value,
    ) -> Result<(), String>;
}
