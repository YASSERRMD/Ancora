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
