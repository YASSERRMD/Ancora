/// Per-plugin capability grants.
///
/// Capabilities are named permissions beyond basic compute access.  The host
/// checks whether a requested capability is present in the plugin's grant set
/// before forwarding the operation.
use std::collections::HashSet;

/// A named capability that may be granted to a plugin.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Access to host environment variables.
    EnvRead,
    /// Ability to write host environment variables.
    EnvWrite,
    /// Access to the host's secret store.
    Secrets,
    /// Inter-plugin communication channel.
    Ipc,
    /// Ability to spawn new agent tasks.
    SpawnTask,
    /// Access to the host's metrics sink.
    Metrics,
    /// A custom, plugin-defined capability identified by name.
    Custom(String),
}

/// The set of capabilities granted to a plugin instance.
#[derive(Debug, Clone)]
pub struct CapabilityGrants {
    granted: HashSet<Capability>,
}

impl CapabilityGrants {
    /// No capabilities granted (most restrictive).
    pub fn none() -> Self {
        Self {
            granted: HashSet::new(),
        }
    }

    /// All standard capabilities granted (least restrictive - use with care).
    pub fn all_standard() -> Self {
        let mut s = Self::none();
        s.grant(Capability::EnvRead);
        s.grant(Capability::Metrics);
        s
    }

    /// Grant a single capability.
    pub fn grant(&mut self, cap: Capability) {
        self.granted.insert(cap);
    }

    /// Revoke a single capability.
    pub fn revoke(&mut self, cap: &Capability) {
        self.granted.remove(cap);
    }

    /// Check whether a capability is present.
    pub fn has(&self, cap: &Capability) -> bool {
        self.granted.contains(cap)
    }

    /// Returns an iterator over all granted capabilities.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.granted.iter()
    }

    /// Returns the number of granted capabilities.
    pub fn len(&self) -> usize {
        self.granted.len()
    }

    /// Returns `true` when no capabilities are granted.
    pub fn is_empty(&self) -> bool {
        self.granted.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_grants_nothing() {
        let g = CapabilityGrants::none();
        assert!(!g.has(&Capability::EnvRead));
        assert!(g.is_empty());
    }

    #[test]
    fn grant_and_revoke() {
        let mut g = CapabilityGrants::none();
        g.grant(Capability::Ipc);
        assert!(g.has(&Capability::Ipc));
        g.revoke(&Capability::Ipc);
        assert!(!g.has(&Capability::Ipc));
    }

    #[test]
    fn custom_capability_roundtrip() {
        let mut g = CapabilityGrants::none();
        g.grant(Capability::Custom("billing:read".into()));
        assert!(g.has(&Capability::Custom("billing:read".into())));
        assert!(!g.has(&Capability::Custom("billing:write".into())));
    }
}
