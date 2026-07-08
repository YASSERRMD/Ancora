/// Plugin permission system - defines permission scopes and enforces them
/// before a plugin command is allowed to execute.
use std::collections::{HashMap, HashSet};

use crate::interface::PluginError;

/// A single permission scope that can be granted or denied.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PermissionScope {
    /// May read from the local file system.
    FsRead,
    /// May write to the local file system.
    FsWrite,
    /// May make outbound network requests.
    Network,
    /// May spawn sub-processes.
    Exec,
    /// May read environment variables.
    EnvRead,
    /// May modify the CLI configuration.
    ConfigWrite,
    /// A custom named scope for extension.
    Custom(String),
}

impl PermissionScope {
    /// Parse a permission scope from its canonical string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "fs:read" => Some(PermissionScope::FsRead),
            "fs:write" => Some(PermissionScope::FsWrite),
            "network" => Some(PermissionScope::Network),
            "exec" => Some(PermissionScope::Exec),
            "env:read" => Some(PermissionScope::EnvRead),
            "config:write" => Some(PermissionScope::ConfigWrite),
            other => Some(PermissionScope::Custom(other.to_string())),
        }
    }

    /// Return the canonical string representation.
    pub fn as_str(&self) -> &str {
        match self {
            PermissionScope::FsRead => "fs:read",
            PermissionScope::FsWrite => "fs:write",
            PermissionScope::Network => "network",
            PermissionScope::Exec => "exec",
            PermissionScope::EnvRead => "env:read",
            PermissionScope::ConfigWrite => "config:write",
            PermissionScope::Custom(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for PermissionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The set of permissions granted to a specific plugin.
#[derive(Debug, Clone, Default)]
pub struct PermissionGrant {
    granted: HashSet<PermissionScope>,
}

impl PermissionGrant {
    /// Create an empty grant (no permissions).
    pub fn new() -> Self {
        Self {
            granted: HashSet::new(),
        }
    }

    /// Create an all-permissive grant (use with caution).
    pub fn all() -> Self {
        let mut g = Self::new();
        g.grant(PermissionScope::FsRead);
        g.grant(PermissionScope::FsWrite);
        g.grant(PermissionScope::Network);
        g.grant(PermissionScope::Exec);
        g.grant(PermissionScope::EnvRead);
        g.grant(PermissionScope::ConfigWrite);
        g
    }

    /// Add a permission to the grant.
    pub fn grant(&mut self, scope: PermissionScope) {
        self.granted.insert(scope);
    }

    /// Remove a permission from the grant.
    pub fn revoke(&mut self, scope: &PermissionScope) {
        self.granted.remove(scope);
    }

    /// Return true if the scope is granted.
    pub fn is_granted(&self, scope: &PermissionScope) -> bool {
        self.granted.contains(scope)
    }

    /// Iterate over all granted scopes.
    pub fn iter(&self) -> impl Iterator<Item = &PermissionScope> {
        self.granted.iter()
    }
}

/// The enforcement engine that maps plugin ids to their grants.
#[derive(Debug, Default)]
pub struct PermissionEnforcer {
    grants: HashMap<String, PermissionGrant>,
}

impl PermissionEnforcer {
    /// Create an enforcer with no grants.
    pub fn new() -> Self {
        Self {
            grants: HashMap::new(),
        }
    }

    /// Set the permission grant for a plugin, replacing any existing grant.
    pub fn set_grant(&mut self, plugin_id: impl Into<String>, grant: PermissionGrant) {
        self.grants.insert(plugin_id.into(), grant);
    }

    /// Grant a single scope to a plugin (creates the entry if absent).
    pub fn grant(&mut self, plugin_id: &str, scope: PermissionScope) {
        self.grants
            .entry(plugin_id.to_string())
            .or_insert_with(PermissionGrant::new)
            .grant(scope);
    }

    /// Revoke a single scope from a plugin.
    pub fn revoke(&mut self, plugin_id: &str, scope: &PermissionScope) {
        if let Some(g) = self.grants.get_mut(plugin_id) {
            g.revoke(scope);
        }
    }

    /// Check whether a plugin has a given permission.
    ///
    /// Returns `Ok(())` if granted, or a [`PluginError::PermissionDenied`] if not.
    pub fn check(&self, plugin_id: &str, scope: &PermissionScope) -> Result<(), PluginError> {
        let granted = self
            .grants
            .get(plugin_id)
            .map(|g| g.is_granted(scope))
            .unwrap_or(false);

        if granted {
            Ok(())
        } else {
            Err(PluginError::PermissionDenied(format!(
                "plugin '{}' lacks permission '{}'",
                plugin_id,
                scope.as_str()
            )))
        }
    }

    /// Return the grant for a plugin, if any.
    pub fn get_grant(&self, plugin_id: &str) -> Option<&PermissionGrant> {
        self.grants.get(plugin_id)
    }
}
