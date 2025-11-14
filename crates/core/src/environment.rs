use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnvironmentKind {
    Local,
    Test,
    Staging,
    Production,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub kind: EnvironmentKind,
    pub metadata: HashMap<String, String>,
}

impl Environment {
    pub fn new(name: &str, kind: EnvironmentKind) -> Self {
        Self {
            name: name.to_string(),
            kind,
            metadata: HashMap::new(),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new("local", EnvironmentKind::Local)
    }
}
