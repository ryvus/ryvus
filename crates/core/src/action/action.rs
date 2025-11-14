use async_trait::async_trait;
use serde_json::Value;

use crate::{context::action_context::ActionContext, error::Error};

use super::result::ActionResult;

#[allow(unused_variables)]
#[async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, context: &mut ActionContext) -> Result<ActionResult, Error>;

    fn key(&self) -> &str;

    async fn configure(&mut self, config: Value) -> Result<(), String> {
        Ok(())
    }
}
