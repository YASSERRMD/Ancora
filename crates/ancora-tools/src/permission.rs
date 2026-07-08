use std::collections::{HashMap, HashSet};

use crate::error::ToolError;

/// Remove patterns from tool descriptions that could be used for prompt injection.
pub fn sanitize_description(desc: &str) -> String {
    desc.replace("ignore previous instructions", "")
        .replace("system prompt", "")
        .replace("<|", "")
        .replace("|>", "")
}

/// Coarse-grained scopes that tools may require.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PermissionScope {
    ReadOnly,
    Write,
    Network,
    Filesystem,
    Custom(String),
}

/// Associates a tool name with its required permission scope.
#[derive(Debug, Clone)]
pub struct ToolPermission {
    pub tool_name: String,
    pub required_scope: PermissionScope,
}

/// Enforces tool scopes and MCP server authentication.
pub struct PermissionBroker {
    granted_scopes: HashSet<PermissionScope>,
    tool_scopes: HashMap<String, PermissionScope>,
    mcp_auth_tokens: HashMap<String, String>,
}

impl PermissionBroker {
    pub fn new() -> Self {
        Self {
            granted_scopes: HashSet::new(),
            tool_scopes: HashMap::new(),
            mcp_auth_tokens: HashMap::new(),
        }
    }
}

impl Default for PermissionBroker {
    fn default() -> Self {
        Self::new()
    }
}

impl PermissionBroker {
    pub fn grant_scope(&mut self, scope: PermissionScope) {
        self.granted_scopes.insert(scope);
    }

    pub fn require_scope_for(&mut self, tool_name: impl Into<String>, scope: PermissionScope) {
        self.tool_scopes.insert(tool_name.into(), scope);
    }

    pub fn register_mcp_auth(&mut self, server_id: impl Into<String>, token: impl Into<String>) {
        self.mcp_auth_tokens.insert(server_id.into(), token.into());
    }

    /// Deny a tool call if its required scope is not in the granted set.
    pub fn check_call(&self, tool_name: &str) -> Result<(), ToolError> {
        if let Some(scope) = self.tool_scopes.get(tool_name) {
            if !self.granted_scopes.contains(scope) {
                return Err(ToolError::ValidationFailed(format!(
                    "tool '{tool_name}' requires scope {scope:?} which is not granted"
                )));
            }
        }
        Ok(())
    }

    /// Deny access to an MCP server if no auth token is registered for it.
    pub fn check_mcp_auth(&self, server_id: &str) -> Result<&str, ToolError> {
        self.mcp_auth_tokens
            .get(server_id)
            .map(|t| t.as_str())
            .ok_or_else(|| {
                ToolError::ValidationFailed(format!(
                    "mcp server '{server_id}' has no registered auth token"
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_tool_call_is_blocked() {
        let mut broker = PermissionBroker::new();
        broker.require_scope_for("rm_file", PermissionScope::Filesystem);
        let err = broker.check_call("rm_file").unwrap_err();
        assert!(matches!(err, ToolError::ValidationFailed(_)));
    }

    #[test]
    fn authorized_tool_call_is_allowed() {
        let mut broker = PermissionBroker::new();
        broker.require_scope_for("rm_file", PermissionScope::Filesystem);
        broker.grant_scope(PermissionScope::Filesystem);
        assert!(broker.check_call("rm_file").is_ok());
    }

    #[test]
    fn unauthenticated_mcp_server_is_refused() {
        let broker = PermissionBroker::new();
        let err = broker.check_mcp_auth("my-mcp").unwrap_err();
        assert!(matches!(err, ToolError::ValidationFailed(_)));
    }

    #[test]
    fn authenticated_mcp_server_returns_token() {
        let mut broker = PermissionBroker::new();
        broker.register_mcp_auth("my-mcp", "secret");
        assert_eq!(broker.check_mcp_auth("my-mcp").unwrap(), "secret");
    }
}
