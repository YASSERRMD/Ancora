/// Plugin permission scoping.

use std::collections::HashSet;

/// A permission scope token (e.g. "llm:generate", "memory:read").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope(pub String);

impl Scope {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A set of granted scopes for a particular plugin instance.
#[derive(Debug, Clone)]
pub struct ScopeGrant {
    pub plugin_id: String,
    granted: HashSet<Scope>,
}

impl ScopeGrant {
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            granted: HashSet::new(),
        }
    }

    /// Grant a scope to this plugin instance.
    pub fn grant(&mut self, scope: Scope) {
        self.granted.insert(scope);
    }

    /// Revoke a previously granted scope.
    pub fn revoke(&mut self, scope: &Scope) {
        self.granted.remove(scope);
    }

    /// Check whether a scope has been granted.
    pub fn has(&self, scope: &Scope) -> bool {
        self.granted.contains(scope)
    }

    /// Return all granted scopes.
    pub fn all(&self) -> &HashSet<Scope> {
        &self.granted
    }
}

/// Error type for permission checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionError {
    /// The plugin requested a scope that was not granted.
    Denied { plugin_id: String, scope: Scope },
    /// The manifest declares required scopes that were never granted.
    MissingScopes { plugin_id: String, scopes: Vec<Scope> },
}

impl std::fmt::Display for PermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionError::Denied { plugin_id, scope } => {
                write!(f, "plugin {plugin_id} was denied scope: {scope}")
            }
            PermissionError::MissingScopes { plugin_id, scopes } => {
                let list: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();
                write!(
                    f,
                    "plugin {plugin_id} is missing required scopes: {}",
                    list.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for PermissionError {}

/// Verify that all scopes declared in a manifest's `required_scopes` have been granted.
pub fn enforce_required_scopes(
    plugin_id: &str,
    required: &[String],
    grant: &ScopeGrant,
) -> Result<(), PermissionError> {
    let missing: Vec<Scope> = required
        .iter()
        .map(|s| Scope::new(s.clone()))
        .filter(|s| !grant.has(s))
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(PermissionError::MissingScopes {
            plugin_id: plugin_id.to_string(),
            scopes: missing,
        })
    }
}

/// Assert that a plugin has a specific scope at call-time.
pub fn require_scope(
    plugin_id: &str,
    scope: Scope,
    grant: &ScopeGrant,
) -> Result<(), PermissionError> {
    if grant.has(&scope) {
        Ok(())
    } else {
        Err(PermissionError::Denied {
            plugin_id: plugin_id.to_string(),
            scope,
        })
    }
}
