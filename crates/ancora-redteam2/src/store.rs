use crate::scenario::{RedTeamScenario, ScenarioKind, ScenarioStatus};
use std::collections::HashMap;

pub struct ScenarioStore {
    scenarios: HashMap<String, RedTeamScenario>,
}

impl Default for ScenarioStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ScenarioStore {
    pub fn new() -> Self {
        Self {
            scenarios: HashMap::new(),
        }
    }
    pub fn insert(&mut self, s: RedTeamScenario) {
        self.scenarios.insert(s.id.clone(), s);
    }
    pub fn get(&self, id: &str) -> Option<&RedTeamScenario> {
        self.scenarios.get(id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut RedTeamScenario> {
        self.scenarios.get_mut(id)
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a RedTeamScenario> {
        self.scenarios
            .values()
            .filter(|s| s.tenant_id == tenant_id)
            .collect()
    }
    pub fn active(&self) -> Vec<&RedTeamScenario> {
        self.scenarios.values().filter(|s| s.is_active()).collect()
    }
    pub fn by_kind<'a>(&'a self, kind: &ScenarioKind) -> Vec<&'a RedTeamScenario> {
        self.scenarios
            .values()
            .filter(|s| &s.kind == kind)
            .collect()
    }
    pub fn by_status<'a>(&'a self, status: &ScenarioStatus) -> Vec<&'a RedTeamScenario> {
        self.scenarios
            .values()
            .filter(|s| &s.status == status)
            .collect()
    }
    pub fn count(&self) -> usize {
        self.scenarios.len()
    }
}
