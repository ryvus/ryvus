use std::sync::Arc;

use async_trait::async_trait;
use ryvus_core::{action::result::ExecutionResult, prelude::pipeline::Pipeline};
use ryvus_engine::engine::EngineApi;

use crate::{
    pipeline::loader::PipelineLoader,
    resolver::{
        config_resolver::resolve_config, env_resolver::EnvResolver, variable::ChainedResolver,
    },
    FlowError,
};

#[async_trait]
pub trait FlowExecutor {
    async fn start_pipeline(
        &self,
        pipeline: String,
        input: serde_json::Value,
    ) -> Result<ExecutionResult, FlowError>;
}

pub struct EngineAdapter {
    pub engine: Arc<dyn EngineApi + 'static>,
}

impl EngineAdapter {
    pub fn new(engine: impl EngineApi + 'static) -> Self {
        Self {
            engine: Arc::new(engine),
        }
    }
}

#[async_trait]
impl FlowExecutor for EngineAdapter {
    async fn start_pipeline(
        &self,
        pipeline: String,
        input: serde_json::Value,
    ) -> Result<ExecutionResult, FlowError> {
        // Try to load as file first
        let pipeline_def_result = PipelineLoader::from_file(&pipeline);

        // If file not found, treat as inline JSON
        let mut pipeline_def = match pipeline_def_result {
            Ok(def) => def,
            Err(_) => {
                // Try to parse as inline JSON string
                let parsed = serde_json::from_str(&pipeline)
                    .map_err(|e| FlowError::Loader(format!("Invalid pipeline JSON: {}", e)))?;

                parsed
            }
        };

        let resolver = ChainedResolver::new(vec![Box::new(EnvResolver)]);

        // Resolve all vars
        resolve_config(&mut pipeline_def, &resolver);

        // Build pipeline
        let pipeline =
            Pipeline::try_from(pipeline_def).map_err(|e| FlowError::Loader(e.to_string()))?;

        // Execute pipeline
        self.engine
            .execute_pipeline(pipeline, input)
            .await
            .map_err(|e| FlowError::Loader(e.to_string()))
    }
}
