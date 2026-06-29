use std::collections::HashSet;

/// Edge egress policy: controls what outbound connections are permitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EgressPolicy {
    /// Block all egress by default (allowlist model).
    DenyAll,
    /// Allow all egress by default (not recommended for edge devices).
    AllowAll,
}

/// An allowed egress endpoint (host + port).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EgressEndpoint {
    pub host: String,
    pub port: u16,
}

impl EgressEndpoint {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }
}

/// Edge egress controller: zero egress by default.
///
/// Implements a strict allowlist model: all outbound connections are blocked
/// unless explicitly permitted. This is the recommended posture for edge devices.
pub struct EdgeEgress {
    policy: EgressPolicy,
    allowed: HashSet<EgressEndpoint>,
}

impl EdgeEgress {
    /// Create with default deny-all policy (zero egress).
    pub fn new() -> Self {
        Self {
            policy: EgressPolicy::DenyAll,
            allowed: HashSet::new(),
        }
    }

    /// Create with an explicit policy.
    pub fn with_policy(policy: EgressPolicy) -> Self {
        Self {
            policy,
            allowed: HashSet::new(),
        }
    }

    /// Explicitly allow egress to a host:port.
    pub fn allow(&mut self, host: impl Into<String>, port: u16) {
        self.allowed.insert(EgressEndpoint::new(host, port));
    }

    /// Remove a host:port from the allowlist.
    pub fn deny(&mut self, host: &str, port: u16) {
        self.allowed.remove(&EgressEndpoint {
            host: host.to_string(),
            port,
        });
    }

    /// Returns true if the given host:port is allowed.
    pub fn is_allowed(&self, host: &str, port: u16) -> bool {
        match self.policy {
            EgressPolicy::AllowAll => true,
            EgressPolicy::DenyAll => self.allowed.contains(&EgressEndpoint {
                host: host.to_string(),
                port,
            }),
        }
    }

    /// Number of explicitly allowed endpoints.
    pub fn allowed_count(&self) -> usize {
        self.allowed.len()
    }

    /// Current policy.
    pub fn policy(&self) -> &EgressPolicy {
        &self.policy
    }
}
