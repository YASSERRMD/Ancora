use crate::measurement::MeasurementKind;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub struct BootPolicy {
    pub tenant_id: String,
    pub require_kinds: HashSet<String>,
    pub allowed_digests: HashMap<String, HashSet<String>>,
    pub deny_unknown: bool,
}

impl BootPolicy {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            require_kinds: HashSet::new(),
            allowed_digests: HashMap::new(),
            deny_unknown: true,
        }
    }

    pub fn require_kind(mut self, kind: MeasurementKind) -> Self {
        self.require_kinds.insert(format!("{}", kind));
        self
    }

    pub fn allow_digest(mut self, name: impl Into<String>, digest: impl Into<String>) -> Self {
        self.allowed_digests
            .entry(name.into())
            .or_default()
            .insert(digest.into());
        self
    }

    pub fn allow_unknown(mut self) -> Self {
        self.deny_unknown = false;
        self
    }

    pub fn is_digest_allowed(&self, name: &str, digest: &str) -> bool {
        if !self.deny_unknown {
            return true;
        }
        self.allowed_digests
            .get(name)
            .map_or(false, |s| s.contains(digest))
    }

    pub fn required_kinds_met(&self, present_kinds: &HashSet<String>) -> bool {
        self.require_kinds.is_subset(present_kinds)
    }
}

#[derive(Debug, Clone)]
pub struct PolicyStore {
    policies: HashMap<String, BootPolicy>,
}

impl PolicyStore {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }
    pub fn insert(&mut self, policy: BootPolicy) {
        self.policies.insert(policy.tenant_id.clone(), policy);
    }
    pub fn get(&self, tenant_id: &str) -> Option<&BootPolicy> {
        self.policies.get(tenant_id)
    }
    pub fn remove(&mut self, tenant_id: &str) -> Option<BootPolicy> {
        self.policies.remove(tenant_id)
    }
    pub fn count(&self) -> usize {
        self.policies.len()
    }
}
