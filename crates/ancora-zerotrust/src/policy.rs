use crate::device::TrustLevel;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthzDecision {
    Allow,
    Deny(String),
    RequireMfa,
}

#[derive(Debug, Clone)]
pub struct ZeroTrustPolicy {
    pub tenant_id: String,
    pub require_device_trust: bool,
    pub min_device_trust: TrustLevel,
    pub require_mfa_for_groups: HashSet<String>,
    pub denied_resources: HashSet<String>,
}

impl ZeroTrustPolicy {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            require_device_trust: false,
            min_device_trust: TrustLevel::Trusted,
            require_mfa_for_groups: HashSet::new(),
            denied_resources: HashSet::new(),
        }
    }

    pub fn require_device(mut self) -> Self {
        self.require_device_trust = true;
        self
    }
    pub fn min_trust(mut self, t: TrustLevel) -> Self {
        self.min_device_trust = t;
        self
    }
    pub fn mfa_for_group(mut self, group: impl Into<String>) -> Self {
        self.require_mfa_for_groups.insert(group.into());
        self
    }
    pub fn deny_resource(mut self, resource: impl Into<String>) -> Self {
        self.denied_resources.insert(resource.into());
        self
    }

    pub fn resource_denied(&self, resource: &str) -> bool {
        self.denied_resources.contains(resource)
    }

    pub fn needs_mfa_for(&self, groups: &[String]) -> bool {
        groups
            .iter()
            .any(|g| self.require_mfa_for_groups.contains(g))
    }
}
