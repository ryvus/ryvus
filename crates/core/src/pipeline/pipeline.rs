use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub key: String,
    pub steps: Vec<PipelineStep>,
}

pub struct PipelineBuilder {
    pipeline: Pipeline,
}

impl Pipeline {
    pub fn builder(key: impl Into<String>) -> PipelineBuilder {
        PipelineBuilder {
            pipeline: Pipeline {
                key: key.into(),
                steps: vec![],
            },
        }
    }
}

impl PipelineBuilder {
    pub fn step(mut self, step: PipelineStep) -> Self {
        self.pipeline.steps.push(step);
        self
    }

    pub fn steps(mut self, steps: Vec<PipelineStep>) -> Self {
        self.pipeline.steps = steps;
        self
    }

    pub fn build(self) -> Pipeline {
        self.pipeline
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    pub key: String,
    pub action: String,
    #[serde(default = "empty_json_object")]
    pub config: Value,
    pub params: serde_json::Value,

    #[serde(default)]
    pub next_when: Vec<ConditionalNext>,

    /// Default path when no condition matches
    #[serde(default)]
    pub otherwise: Option<String>, // use r#else because `else` is reserved in Rust

    /// Error path if this step fails
    #[serde(default)]
    pub on_error: Option<String>,

    /// Optional fallback linear path
    #[serde(default)]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalNext {
    pub when: String,
    pub next: String,
}

pub struct PipelineStepBuilder {
    step: PipelineStep,
}

impl PipelineStep {
    pub fn new(
        key: impl Into<String>,
        action: impl Into<String>,
        config: serde_json::Value,
        params: serde_json::Value,
    ) -> Self {
        Self {
            key: key.into(),
            action: action.into(),
            config: config,
            params,
            next: None,
            on_error: None,
            next_when: vec![],
            otherwise: None,
        }
    }

    pub fn builder(key: &str, action: &str) -> PipelineStepBuilder {
        PipelineStepBuilder {
            step: PipelineStep {
                key: key.into(),
                action: action.into(),
                config: serde_json::json!({}),
                params: serde_json::json!({}),
                next_when: vec![],
                otherwise: None,
                on_error: None,
                next: None,
            },
        }
    }
}

impl PipelineStepBuilder {
    pub fn config(mut self, cfg: Value) -> Self {
        self.step.config = cfg;
        self
    }

    pub fn params(mut self, params: Value) -> Self {
        self.step.params = params;
        self
    }

    pub fn next(mut self, next: impl Into<String>) -> Self {
        self.step.next = Some(next.into());
        self
    }

    pub fn otherwise(mut self, else_key: impl Into<String>) -> Self {
        self.step.otherwise = Some(else_key.into());
        self
    }

    pub fn on_error(mut self, key: impl Into<String>) -> Self {
        self.step.on_error = Some(key.into());
        self
    }

    pub fn when(mut self, condition: impl Into<String>, next: impl Into<String>) -> Self {
        self.step.next_when.push(ConditionalNext {
            when: condition.into(),
            next: next.into(),
        });
        self
    }

    pub fn build(self) -> PipelineStep {
        self.step
    }
}

fn empty_json_object() -> Value {
    serde_json::json!({})
}
