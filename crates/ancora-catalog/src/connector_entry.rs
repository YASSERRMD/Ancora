use crate::metadata::Metadata;

/// How the MCP server is launched / connected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpTransport {
    /// Process launched with the given binary and arguments.
    Stdio { command: String, args: Vec<String> },
    /// Connected over HTTP/SSE at the given URL.
    Http { url: String },
}

/// MCP connection configuration embedded in a connector entry.
#[derive(Debug, Clone)]
pub struct McpConfig {
    pub transport: McpTransport,
    /// Environment variables required by the server.
    pub env_vars: Vec<String>,
}

impl McpConfig {
    pub fn stdio(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            transport: McpTransport::Stdio {
                command: command.into(),
                args,
            },
            env_vars: Vec::new(),
        }
    }

    pub fn http(url: impl Into<String>) -> Self {
        Self {
            transport: McpTransport::Http { url: url.into() },
            env_vars: Vec::new(),
        }
    }

    pub fn with_env_var(mut self, var: impl Into<String>) -> Self {
        self.env_vars.push(var.into());
        self
    }
}

/// A catalog entry describing an MCP connector.
#[derive(Debug, Clone)]
pub struct ConnectorEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub mcp_config: McpConfig,
    pub metadata: Metadata,
}

impl ConnectorEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        mcp_config: McpConfig,
        metadata: Metadata,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            mcp_config,
            metadata,
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.id.is_empty() || self.name.is_empty() {
            return false;
        }
        match &self.mcp_config.transport {
            McpTransport::Stdio { command, .. } => !command.is_empty(),
            McpTransport::Http { url } => !url.is_empty(),
        }
    }
}
