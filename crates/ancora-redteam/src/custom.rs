//! Custom scenario authoring: helpers for building bespoke red-team scenarios.

use crate::scenario::{AdversarialScenario, AttackCategory, ScenarioDataset};

/// Builder for constructing a custom red-team scenario dataset.
pub struct ScenarioBuilder {
    dataset: ScenarioDataset,
    next_id: usize,
}

impl ScenarioBuilder {
    pub fn new() -> Self {
        Self { dataset: ScenarioDataset::default(), next_id: 1 }
    }

    pub fn add_injection(mut self, payload: &str, expected_blocked: bool) -> Self {
        let id = format!("custom-inj-{:03}", self.next_id);
        self.next_id += 1;
        self.dataset.add(AdversarialScenario::new(&id, AttackCategory::Injection, payload, expected_blocked));
        self
    }

    pub fn add_jailbreak(mut self, payload: &str, expected_blocked: bool) -> Self {
        let id = format!("custom-jail-{:03}", self.next_id);
        self.next_id += 1;
        self.dataset.add(AdversarialScenario::new(&id, AttackCategory::Jailbreak, payload, expected_blocked));
        self
    }

    pub fn add_tool_misuse(mut self, tool_name: &str, expected_blocked: bool) -> Self {
        let id = format!("custom-tool-{:03}", self.next_id);
        self.next_id += 1;
        self.dataset.add(AdversarialScenario::new(&id, AttackCategory::ToolMisuse, tool_name, expected_blocked));
        self
    }

    pub fn build(self) -> ScenarioDataset {
        self.dataset
    }
}

impl Default for ScenarioBuilder {
    fn default() -> Self {
        Self::new()
    }
}
