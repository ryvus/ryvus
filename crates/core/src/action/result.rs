use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::id::generate_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub run_id: String,
    /// Optional user-friendly key for display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_key: Option<String>,
    /// Optional environment (local, staging, production)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    pub status: ExecutionStatus,

    /// Error information if the pipeline failed.
    pub error: Option<String>,

    /// Optional per-step results.
    #[serde(default)]
    pub steps: Vec<ActionResult>,

    /// Collected output from the pipeline (optional).
    pub result: Option<Value>,

    /// Metadata like timing, step counts, etc.
    pub metrics: ExecutionMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    Success,
    Failed,
    Canceled,
    Skipped,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub duration_ms: u64,
    pub steps_total: usize,
    pub steps_succeeded: usize,
    pub steps_failed: usize,
}

impl ExecutionMetrics {
    pub fn from_steps(started_at: DateTime<Utc>, steps: &[ActionResult]) -> Self {
        let finished_at = Utc::now();
        let duration_ms = (finished_at - started_at).num_milliseconds().max(0) as u64;
        let steps_total = steps.len();
        let steps_succeeded = steps
            .iter()
            .filter(|s| s.status == ExecutionStatus::Success)
            .count();
        let steps_failed = steps
            .iter()
            .filter(|s| s.status == ExecutionStatus::Failed)
            .count();

        Self {
            started_at,
            finished_at,
            duration_ms,
            steps_total,
            steps_succeeded,
            steps_failed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub id: String,
    pub key: String,
    pub action: Option<String>,
    pub status: ExecutionStatus,
    pub output: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<u64>,
}

impl ActionResult {
    pub fn success(output: Value) -> Self {
        Self {
            key: "".to_string(),
            id: generate_id("action_result"),
            action: None,
            status: ExecutionStatus::Success,
            output: Some(output),
            message: None,
            started_at: Some(Utc::now()),
            finished_at: Some(Utc::now()),
            duration_ms: Some(0),
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            key: "".to_string(),

            id: generate_id("action_result"),
            action: None,
            status: ExecutionStatus::Failed,
            output: None,
            message: Some(message.into()),
            started_at: Some(Utc::now()),
            finished_at: Some(Utc::now()),
            duration_ms: Some(0),
        }
    }

    pub fn skipped() -> Self {
        Self {
            key: "".to_string(),
            id: generate_id("action_result"),
            action: None,
            status: ExecutionStatus::Skipped,
            output: None,
            message: None,
            started_at: None,
            finished_at: None,
            duration_ms: None,
        }
    }
}
