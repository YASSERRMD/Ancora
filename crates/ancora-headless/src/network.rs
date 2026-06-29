/// No external network policy for the headless agent.
///
/// Defines and enforces a policy that blocks all external network egress
/// by default, allowing only local socket communication.

use std::collections::HashSet;

/// The network egress policy for the headless agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EgressPolicy {
    /// All external traffic is blocked. Only loopback/socket allowed.
    DenyAll,
    /// Only specified hosts/CIDRs are allowed.
    AllowList(Vec<String>),
    /// All traffic is allowed (not recommended for headless inference OS).
    AllowAll,
}

impl std::fmt::Display for EgressPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EgressPolicy::DenyAll => write!(f, "deny-all"),
            EgressPolicy::AllowList(hosts) => write!(f, "allow-list({})", hosts.join(",")),
            EgressPolicy::AllowAll => write!(f, "allow-all"),
        }
    }
}

impl Default for EgressPolicy {
    fn default() -> Self {
        EgressPolicy::DenyAll
    }
}

/// Network configuration for the headless agent.
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Egress policy applied at service startup.
    pub egress: EgressPolicy,
    /// Whether to enable loopback only mode (no external interfaces).
    pub loopback_only: bool,
    /// Whether DNS resolution is permitted.
    pub allow_dns: bool,
    /// Local socket paths that are permitted.
    pub allowed_sockets: HashSet<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("/run/ancora/agent.sock".to_string());
        NetworkConfig {
            egress: EgressPolicy::DenyAll,
            loopback_only: true,
            allow_dns: false,
            allowed_sockets: allowed,
        }
    }
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_egress(mut self, policy: EgressPolicy) -> Self {
        self.egress = policy;
        self
    }

    pub fn allow_socket(mut self, path: impl Into<String>) -> Self {
        self.allowed_sockets.insert(path.into());
        self
    }

    /// Returns true if external network egress is blocked.
    pub fn is_egress_blocked(&self) -> bool {
        matches!(self.egress, EgressPolicy::DenyAll)
    }

    /// Returns true if the given socket path is permitted.
    pub fn socket_allowed(&self, path: &str) -> bool {
        self.allowed_sockets.contains(path)
    }

    /// Returns true if the given host is reachable under the current policy.
    pub fn host_reachable(&self, host: &str) -> bool {
        match &self.egress {
            EgressPolicy::DenyAll => false,
            EgressPolicy::AllowAll => true,
            EgressPolicy::AllowList(list) => list.iter().any(|h| h == host),
        }
    }
}

/// A network access attempt that was evaluated by the policy.
#[derive(Debug, Clone)]
pub struct AccessAttempt {
    pub destination: String,
    pub port: u16,
    pub protocol: Protocol,
    pub allowed: bool,
}

/// IP protocol for an access attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Unix,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "tcp"),
            Protocol::Udp => write!(f, "udp"),
            Protocol::Unix => write!(f, "unix"),
        }
    }
}

/// Evaluates a network access attempt against the given policy.
pub fn evaluate_access(config: &NetworkConfig, dest: &str, port: u16, proto: Protocol) -> AccessAttempt {
    let allowed = match &proto {
        Protocol::Unix => config.socket_allowed(dest),
        Protocol::Tcp | Protocol::Udp => {
            if config.loopback_only && dest != "127.0.0.1" && dest != "::1" && dest != "localhost" {
                false
            } else {
                config.host_reachable(dest)
            }
        }
    };
    AccessAttempt { destination: dest.to_string(), port, protocol: proto, allowed }
}

/// An audit log of network access decisions.
pub struct NetworkAuditLog {
    entries: Vec<AccessAttempt>,
}

impl NetworkAuditLog {
    pub fn new() -> Self {
        NetworkAuditLog { entries: Vec::new() }
    }

    pub fn record(&mut self, attempt: AccessAttempt) {
        self.entries.push(attempt);
    }

    pub fn blocked_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.allowed).count()
    }

    pub fn allowed_count(&self) -> usize {
        self.entries.iter().filter(|e| e.allowed).count()
    }

    pub fn all_blocked(&self) -> bool {
        !self.entries.is_empty() && self.entries.iter().all(|e| !e.allowed)
    }
}

impl Default for NetworkAuditLog {
    fn default() -> Self {
        Self::new()
    }
}
