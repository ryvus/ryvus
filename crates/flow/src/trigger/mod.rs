use async_trait::async_trait;
use crate::error::FlowError;

/// Minimal trigger trait; implementations can call into FlowPipelineManager.
#[async_trait]
pub trait Trigger: Send + Sync {
    async fn start(&self) -> Result<(), FlowError>;
    async fn shutdown(&self) -> Result<(), FlowError>;
}

/// No-op trigger useful for tests.
pub struct NoopTrigger;

#[async_trait]
impl Trigger for NoopTrigger {
    async fn start(&self) -> Result<(), FlowError> { Ok(()) }
    async fn shutdown(&self) -> Result<(), FlowError> { Ok(()) }
}
