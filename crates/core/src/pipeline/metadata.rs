use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Descriptive metadata about a full pipeline run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    pub run_id: String,
    pub name: Option<String>,
    pub started_at: Option<SystemTime>,
    pub finished_at: Option<SystemTime>,
    pub duration: Option<Duration>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// Descriptive metadata about a single action execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMetadata {
    pub id: String,
    pub action_type: String,
    pub started_at: Option<SystemTime>,
    pub finished_at: Option<SystemTime>,
    pub duration: Option<Duration>,
    pub message: Option<String>,
}

impl ActionMetadata {
    pub fn compute_duration(&mut self) {
        if let (Some(start), Some(end)) = (self.started_at, self.finished_at) {
            if let Ok(d) = end.duration_since(start) {
                self.duration = Some(d);
            }
        }
    }
}
