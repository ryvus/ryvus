use async_trait::async_trait;

use crate::{
    context::{action_context::ActionContext, execution_context::ExecutionContext},
    error::Error,
};

#[async_trait]
pub trait ActionHook: Send + Sync + 'static {
    async fn before(&self, context: &mut ActionContext);
    async fn after(&self, context: &mut ActionContext);
    async fn error(&self, context: &mut ActionContext, err: &Error);
}

#[async_trait]
pub trait PipelineHook: Send + Sync + 'static {
    async fn completed(&self, context: &mut ExecutionContext);
    async fn failed(&self, context: &mut ExecutionContext);
    async fn canceled(&self, context: &mut ExecutionContext);
    async fn start(&self, context: &mut ExecutionContext);
}
