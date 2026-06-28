/// Per-plugin network policy.
///
/// Determines which outbound and inbound connections a plugin is allowed to
/// make. The policy is evaluated at connection time by the host's network
/// interceptor before the syscall is forwarded.

/// An IP address range in CIDR notation (stored as a string for simplicity).
pub type CidrRange = String;

/// A TCP/UDP port number.
pub type Port = u16;

/// Action taken when a network request does not match any explicit rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultNetworkAction {
    Allow,
    Deny,
}

/// A single rule that allows or denies connections to a specific host/port.
#[derive(Debug, Clone)]
pub struct NetworkRule {
    /// Destination hostname or IP range (None = any).
    pub host: Option<String>,
    /// Destination port (None = any).
    pub port: Option<Port>,
    /// Whether this rule allows or denies the matched traffic.
    pub action: NetworkRuleAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRuleAction {
    Allow,
    Deny,
}

/// Complete network policy for a plugin.
#[derive(Debug, Clone)]
pub struct NetworkPolicy {
    /// Ordered list of rules evaluated from first to last.
    pub rules: Vec<NetworkRule>,
    /// Action applied when no rule matches.
    pub default_action: DefaultNetworkAction,
}

impl NetworkPolicy {
    /// Deny all outbound traffic (most restrictive).
    pub fn deny_all() -> Self {
        Self {
            rules: vec![],
            default_action: DefaultNetworkAction::Deny,
        }
    }

    /// Allow all outbound traffic (least restrictive).
    pub fn allow_all() -> Self {
        Self {
            rules: vec![],
            default_action: DefaultNetworkAction::Allow,
        }
    }

    /// Add an explicit allow rule for the given host and optional port.
    pub fn allow_host(&mut self, host: impl Into<String>, port: Option<Port>) {
        self.rules.push(NetworkRule {
            host: Some(host.into()),
            port,
            action: NetworkRuleAction::Allow,
        });
    }

    /// Add an explicit deny rule for the given host and optional port.
    pub fn deny_host(&mut self, host: impl Into<String>, port: Option<Port>) {
        self.rules.push(NetworkRule {
            host: Some(host.into()),
            port,
            action: NetworkRuleAction::Deny,
        });
    }

    /// Evaluate a connection attempt.
    ///
    /// Returns `true` when the connection is permitted by policy.
    pub fn permits(&self, host: &str, port: Port) -> bool {
        for rule in &self.rules {
            let host_matches = rule.host.as_deref().map_or(true, |h| h == host);
            let port_matches = rule.port.map_or(true, |p| p == port);
            if host_matches && port_matches {
                return rule.action == NetworkRuleAction::Allow;
            }
        }
        self.default_action == DefaultNetworkAction::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_all_blocks_everything() {
        let policy = NetworkPolicy::deny_all();
        assert!(!policy.permits("example.com", 443));
        assert!(!policy.permits("192.168.1.1", 80));
    }

    #[test]
    fn allow_all_permits_everything() {
        let policy = NetworkPolicy::allow_all();
        assert!(policy.permits("example.com", 443));
    }

    #[test]
    fn explicit_allow_rule_overrides_deny_default() {
        let mut policy = NetworkPolicy::deny_all();
        policy.allow_host("api.example.com", Some(443));
        assert!(policy.permits("api.example.com", 443));
        assert!(!policy.permits("evil.com", 443));
    }

    #[test]
    fn explicit_deny_rule_overrides_allow_default() {
        let mut policy = NetworkPolicy::allow_all();
        policy.deny_host("blocked.internal", None);
        assert!(!policy.permits("blocked.internal", 80));
        assert!(policy.permits("safe.example.com", 80));
    }
}
