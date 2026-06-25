use std::collections::HashSet;

/// Declarative governance descriptor attached to an agent or tool.
#[derive(Debug, Clone, Default)]
pub struct Policy {
    /// When true, all outbound network calls are unconditionally blocked.
    /// `allowed_endpoints` is ignored in this mode.
    pub air_gapped: bool,
    /// Endpoints (hostnames or prefixes) that are allowed for egress.
    /// Only checked when `air_gapped` is false. Empty means allow all.
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

    /// Block all outbound egress unconditionally.
    pub fn air_gapped(mut self) -> Self {
        self.air_gapped = true;
        self
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
