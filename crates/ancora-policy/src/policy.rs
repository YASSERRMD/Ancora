use std::collections::HashSet;

/// Declarative governance descriptor attached to an agent or tool.
#[derive(Debug, Clone, Default)]
pub struct Policy {
    /// Endpoints (hostnames or prefixes) that are allowed for egress.
    /// Empty means all endpoints are blocked in air-gapped mode.
    pub allowed_endpoints: HashSet<String>,
    /// Whether PII fields must be detected and redacted before sending.
    pub require_pii_redaction: bool,
    /// Whether every tool call must be written to an audit log.
    pub require_audit: bool,
    /// Permitted tool names; empty means all tools are allowed.
    pub allowed_tools: HashSet<String>,
}

impl Policy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.allowed_endpoints.insert(endpoint.into());
        self
    }

    pub fn allow_tool(mut self, tool: impl Into<String>) -> Self {
        self.allowed_tools.insert(tool.into());
        self
    }
}

/// Associates a `Policy` with a named agent or tool.
#[derive(Debug, Clone)]
pub struct PolicyAttachment {
    pub target_name: String,
    pub policy: Policy,
}
