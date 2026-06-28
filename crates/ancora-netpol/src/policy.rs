use std::collections::HashMap;
use crate::rule::{Effect, NetworkRule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultPosture {
    DenyAll,
    AllowAll,
}

#[derive(Debug)]
pub struct NetworkPolicy {
    pub tenant_id: String,
    pub rules: Vec<NetworkRule>,
    pub default_posture: DefaultPosture,
}

impl NetworkPolicy {
    pub fn new(tenant_id: impl Into<String>, default_posture: DefaultPosture) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            rules: Vec::new(),
            default_posture,
        }
    }

    pub fn deny_by_default(tenant_id: impl Into<String>) -> Self {
        Self::new(tenant_id, DefaultPosture::DenyAll)
    }

    pub fn allow_by_default(tenant_id: impl Into<String>) -> Self {
        Self::new(tenant_id, DefaultPosture::AllowAll)
    }

    pub fn add_rule(&mut self, rule: NetworkRule) {
        self.rules.push(rule);
        self.rules.sort_by_key(|r| r.priority);
    }

    pub fn rule_count(&self) -> usize { self.rules.len() }

    pub fn allow_rules(&self) -> Vec<&NetworkRule> {
        self.rules.iter().filter(|r| r.effect == Effect::Allow).collect()
    }

    pub fn deny_rules(&self) -> Vec<&NetworkRule> {
        self.rules.iter().filter(|r| r.effect == Effect::Deny).collect()
    }

    pub fn bulk_add_rules(&mut self, rules: impl IntoIterator<Item = NetworkRule>) {
        for r in rules { self.rules.push(r); }
        self.rules.sort_by_key(|r| r.priority);
    }

    pub fn remove_rule(&mut self, id: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.id != id);
        self.rules.len() < before
    }

    pub fn replace_rule(&mut self, id: &str, new_rule: NetworkRule) -> bool {
        if let Some(pos) = self.rules.iter().position(|r| r.id == id) {
            self.rules[pos] = new_rule;
            self.rules.sort_by_key(|r| r.priority);
            true
        } else {
            false
        }
    }

    pub fn get_rule(&self, id: &str) -> Option<&NetworkRule> {
        self.rules.iter().find(|r| r.id == id)
    }
}

pub struct PolicyStore {
    policies: HashMap<String, NetworkPolicy>,
}

impl PolicyStore {
    pub fn new() -> Self { Self { policies: HashMap::new() } }

    pub fn insert(&mut self, policy: NetworkPolicy) {
        self.policies.insert(policy.tenant_id.clone(), policy);
    }

    pub fn get(&self, tenant_id: &str) -> Option<&NetworkPolicy> { self.policies.get(tenant_id) }
    pub fn get_mut(&mut self, tenant_id: &str) -> Option<&mut NetworkPolicy> { self.policies.get_mut(tenant_id) }

    pub fn count(&self) -> usize { self.policies.len() }
}

impl Default for PolicyStore {
    fn default() -> Self { Self::new() }
}
