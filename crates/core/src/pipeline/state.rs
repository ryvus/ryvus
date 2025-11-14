use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineState {
    Pending,
    Running,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionState {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
    Canceled,
}

impl ActionState {
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            ActionState::Success
                | ActionState::Failed
                | ActionState::Skipped
                | ActionState::Canceled
        )
    }
}
