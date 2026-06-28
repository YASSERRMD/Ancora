use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookStep {
    pub step: u32,
    pub action: String,
    pub expected_outcome: String,
    pub on_failure: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub name: String,
    pub trigger: String,
    pub steps: Vec<PlaybookStep>,
}

impl Playbook {
    pub fn new(name: &str, trigger: &str) -> Self {
        Self { name: name.to_string(), trigger: trigger.to_string(), steps: vec![] }
    }

    pub fn add_step(mut self, action: &str, expected: &str, on_failure: &str) -> Self {
        let step = self.steps.len() as u32 + 1;
        self.steps.push(PlaybookStep {
            step,
            action: action.to_string(),
            expected_outcome: expected.to_string(),
            on_failure: on_failure.to_string(),
        });
        self
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}
