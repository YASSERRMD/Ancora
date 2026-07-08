//! Adversarial scenario format for red-team harness.

/// The category of adversarial attack a scenario exercises.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackCategory {
    Injection,
    ToolMisuse,
    DataExfiltration,
    PrivilegeEscalation,
    Jailbreak,
}

/// A single adversarial test scenario.
#[derive(Debug, Clone)]
pub struct AdversarialScenario {
    pub id: String,
    pub category: AttackCategory,
    pub payload: String,
    pub expected_blocked: bool,
}

impl AdversarialScenario {
    pub fn new(id: &str, category: AttackCategory, payload: &str, expected_blocked: bool) -> Self {
        Self {
            id: id.to_string(),
            category,
            payload: payload.to_string(),
            expected_blocked,
        }
    }
}

/// A dataset of adversarial scenarios grouped by category.
#[derive(Debug, Default)]
pub struct ScenarioDataset {
    pub scenarios: Vec<AdversarialScenario>,
}

impl ScenarioDataset {
    pub fn add(&mut self, scenario: AdversarialScenario) {
        self.scenarios.push(scenario);
    }

    pub fn by_category(&self, cat: &AttackCategory) -> Vec<&AdversarialScenario> {
        self.scenarios
            .iter()
            .filter(|s| &s.category == cat)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.scenarios.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scenarios.is_empty()
    }
}
