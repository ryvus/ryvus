use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    action::result::{ExecutionMetrics, ExecutionResult},
    environment::Environment,
    prelude::{ActionResult, ExecutionStatus, PipelineStep},
    utils::id::generate_id,
};

/// The runtime context shared across the entire pipeline execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Environment configuration (local, staging, production, etc.)
    pub environment: Environment,

    /// Unique pipeline identifier (auto-generated if not provided)
    pub pipeline_key: String,

    /// Optional run identifier for this execution.
    pub run_id: String,

    /// Shared scratchpad for pipeline-wide data.
    pub data: HashMap<String, Value>,

    /// Results of individual steps (ActionResults)
    pub steps: Vec<ActionResult>,

    #[serde(skip_serializing)]
    pub current_step: Option<PipelineStep>,

    /// Aggregated results for quick lookup.
    pub results: HashMap<String, Value>,

    /// Start and end times for metrics
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,

    /// Optional error at pipeline level
    pub error: Option<String>,
}

impl ExecutionContext {
    pub fn new(pipeline_key: impl Into<String>, environment: Environment) -> Self {
        Self {
            environment,
            pipeline_key: pipeline_key.into(),
            run_id: generate_id("run"),
            data: HashMap::new(),
            steps: Vec::new(),
            results: HashMap::new(),
            started_at: Utc::now(),
            finished_at: None,
            error: None,
            current_step: None,
        }
    }

    /// Mark the context as finished (records final timestamp)
    pub fn finish(&mut self) {
        self.finished_at = Some(Utc::now());
    }

    /// Insert temporary working data (used by actions)
    pub fn insert(&mut self, key: impl Into<String>, value: Value) {
        self.data.insert(key.into(), value);
    }

    /// Insert a step’s result (adds to both results map and step history)
    pub fn insert_result(&mut self, action_id: impl Into<String>, result: ActionResult) {
        if let Some(value) = &result.output {
            self.results.insert(action_id.into(), value.clone());
        }
        self.steps.push(result);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Serialize the entire execution context as JSON
    pub fn as_value(&self) -> Value {
        serde_json::json!({
            "pipeline_key": self.pipeline_key,
            "run_id": self.run_id,
            "environment": self.environment,
            "data": self.data,
            "results": self.results,
            "steps": self.steps,
            "started_at": self.started_at,
            "finished_at": self.finished_at,
            "error": self.error,
        })
    }

    /// Convert into a final `ExecutionResult`
    pub fn into_result(mut self) -> ExecutionResult {
        self.finish();
        let finished_at = self.finished_at.unwrap_or_else(Utc::now);
        let duration_ms = (finished_at - self.started_at).num_milliseconds().max(0) as u64;

        let steps_total = self.steps.len();
        let steps_succeeded = self
            .steps
            .iter()
            .filter(|s| s.status == ExecutionStatus::Success)
            .count();
        let steps_failed = self
            .steps
            .iter()
            .filter(|s| s.status == ExecutionStatus::Failed)
            .count();

        ExecutionResult {
            pipeline_key: self.pipeline_key.into(),
            run_id: self.run_id.clone(),
            status: if self.error.is_none() {
                ExecutionStatus::Success
            } else {
                ExecutionStatus::Failed
            },
            environment: Some("local".to_string()), // TODO make this dynamic
            result: match &self.steps.last() {
                Some(val) => val.output.clone(),
                None => None,
            },
            steps: self.steps,
            error: self.error,
            metrics: ExecutionMetrics {
                started_at: self.started_at,
                finished_at,
                duration_ms,
                steps_total,
                steps_succeeded,
                steps_failed,
            },
        }
    }
}
