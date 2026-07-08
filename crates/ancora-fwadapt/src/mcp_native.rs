//! Native import of Model Context Protocol (MCP) tool definitions into Ancora.
//!
//! MCP tools are described by a JSON-schema-like structure. This module
//! provides an in-process representation and a registry without any network I/O.

#[derive(Debug, Clone, PartialEq)]
pub struct McpParamDef {
    pub name: String,
    pub param_type: McpParamType,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum McpParamType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

impl std::fmt::Display for McpParamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Object => "object",
            Self::Array => "array",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct McpToolDef {
    pub name: String,
    pub description: String,
    pub params: Vec<McpParamDef>,
}

#[derive(Debug, Clone)]
pub struct McpToolRegistry {
    tools: Vec<McpToolDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum McpRegistryError {
    DuplicateTool(String),
    ToolNotFound(String),
}

impl std::fmt::Display for McpRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateTool(n) => write!(f, "duplicate tool: {}", n),
            Self::ToolNotFound(n) => write!(f, "tool not found: {}", n),
        }
    }
}

impl McpToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    /// Register an MCP tool, rejecting duplicates.
    pub fn register(&mut self, tool: McpToolDef) -> Result<(), McpRegistryError> {
        if self.tools.iter().any(|t| t.name == tool.name) {
            return Err(McpRegistryError::DuplicateTool(tool.name));
        }
        self.tools.push(tool);
        Ok(())
    }

    /// Look up a tool by name.
    pub fn get(&self, name: &str) -> Result<&McpToolDef, McpRegistryError> {
        self.tools
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| McpRegistryError::ToolNotFound(name.to_string()))
    }

    /// List all registered tool names.
    pub fn tool_names(&self) -> Vec<&str> {
        self.tools.iter().map(|t| t.name.as_str()).collect()
    }

    /// Validate that all required parameters are present in a call map.
    pub fn validate_call(&self, name: &str, args: &[(&str, &str)]) -> Result<(), McpRegistryError> {
        let tool = self.get(name)?;
        let provided: std::collections::HashSet<&str> = args.iter().map(|(k, _)| *k).collect();
        for param in &tool.params {
            if param.required && !provided.contains(param.name.as_str()) {
                return Err(McpRegistryError::ToolNotFound(format!(
                    "missing required param: {}",
                    param.name
                )));
            }
        }
        Ok(())
    }
}

impl Default for McpToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_lookup() {
        let mut reg = McpToolRegistry::new();
        reg.register(McpToolDef {
            name: "read_file".into(),
            description: "Read a file from disk".into(),
            params: vec![McpParamDef {
                name: "path".into(),
                param_type: McpParamType::String,
                required: true,
                description: "File path".into(),
            }],
        })
        .unwrap();
        assert_eq!(reg.get("read_file").unwrap().name, "read_file");
        assert!(matches!(
            reg.register(McpToolDef {
                name: "read_file".into(),
                description: "dup".into(),
                params: vec![],
            }),
            Err(McpRegistryError::DuplicateTool(_))
        ));
    }
}
