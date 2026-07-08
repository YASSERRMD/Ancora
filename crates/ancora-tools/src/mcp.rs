use std::sync::Arc;

use crate::error::ToolError;
use crate::tool::{Tool, ToolEffect};

/// Definition of a tool as returned by an MCP server's `tools/list` endpoint.
#[derive(Debug, Clone)]
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Low-level transport that sends a JSON-RPC request and receives a response.
pub trait McpTransport: Send + Sync {
    fn send(&self, method: &str, params: serde_json::Value)
        -> Result<serde_json::Value, ToolError>;
}

/// Sends JSON-RPC over a subprocess's stdin/stdout.
pub struct StdioTransport {
    pub command: String,
    pub args: Vec<String>,
}

/// Sends JSON-RPC over HTTP to an MCP server.
pub struct HttpTransport {
    pub endpoint: String,
}

impl McpTransport for HttpTransport {
    fn send(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, ToolError> {
        let body = serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": method, "params": params
        });
        let body_str =
            serde_json::to_string(&body).map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        let resp_str = ureq::post(&self.endpoint)
            .set("Content-Type", "application/json")
            .send_string(&body_str)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?
            .into_string()
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        let resp: serde_json::Value = serde_json::from_str(&resp_str)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(resp["result"].clone())
    }
}

/// High-level MCP client that wraps a transport.
pub struct McpClient {
    transport: Box<dyn McpTransport>,
}

impl McpClient {
    pub fn new(transport: Box<dyn McpTransport>) -> Self {
        Self { transport }
    }

    /// Fetch the list of tools from the MCP server.
    pub fn list_tools(&self) -> Result<Vec<McpToolDefinition>, ToolError> {
        let result = self.transport.send("tools/list", serde_json::json!({}))?;
        let tools = result["tools"]
            .as_array()
            .ok_or_else(|| ToolError::ExecutionFailed("missing tools array".into()))?;
        tools
            .iter()
            .map(|t| {
                Ok(McpToolDefinition {
                    name: t["name"].as_str().unwrap_or("").to_owned(),
                    description: t["description"].as_str().unwrap_or("").to_owned(),
                    input_schema: t["inputSchema"].clone(),
                })
            })
            .collect()
    }

    /// Call a tool by name with the given input.
    pub fn call_tool(
        &self,
        name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, ToolError> {
        self.transport.send(
            "tools/call",
            serde_json::json!({ "name": name, "arguments": input }),
        )
    }

    /// Discover all tools from the server and return them as `Arc<dyn Tool>` adapters.
    pub fn discover_tools(self: Arc<Self>) -> Result<Vec<Arc<dyn Tool>>, ToolError> {
        let defs = self.list_tools()?;
        Ok(defs
            .into_iter()
            .map(|def| {
                let adapter: Arc<dyn Tool> = Arc::new(McpToolAdapter {
                    client: Arc::clone(&self),
                    definition: def,
                });
                adapter
            })
            .collect())
    }
}

/// Wraps one MCP tool and presents it as a `Tool` in the local registry.
pub struct McpToolAdapter {
    client: Arc<McpClient>,
    definition: McpToolDefinition,
}

impl Tool for McpToolAdapter {
    fn name(&self) -> &str {
        &self.definition.name
    }
    fn description(&self) -> &str {
        &self.definition.description
    }
    fn input_schema(&self) -> serde_json::Value {
        self.definition.input_schema.clone()
    }
    fn effect(&self) -> ToolEffect {
        ToolEffect::Write
    }
    fn call(&self, input: &serde_json::Value) -> Result<serde_json::Value, ToolError> {
        self.client.call_tool(&self.definition.name, input.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTransport {
        list_response: serde_json::Value,
        call_response: serde_json::Value,
    }

    impl McpTransport for MockTransport {
        fn send(
            &self,
            method: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value, ToolError> {
            match method {
                "tools/list" => Ok(self.list_response.clone()),
                "tools/call" => Ok(self.call_response.clone()),
                _ => Err(ToolError::ExecutionFailed("unknown method".into())),
            }
        }
    }

    #[test]
    fn mcp_tools_appear_and_invoke_through_the_contract() {
        let transport = MockTransport {
            list_response: serde_json::json!({
                "tools": [{ "name": "greet", "description": "say hello", "inputSchema": { "type": "object", "required": ["name"] } }]
            }),
            call_response: serde_json::json!({ "content": "hello world" }),
        };
        let client = Arc::new(McpClient::new(Box::new(transport)));
        let tools = client.discover_tools().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name(), "greet");
        let result = tools[0]
            .call(&serde_json::json!({ "name": "world" }))
            .unwrap();
        assert_eq!(result["content"], "hello world");
    }
}

impl McpTransport for StdioTransport {
    fn send(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, ToolError> {
        use std::io::{BufRead, Write};
        let mut child = std::process::Command::new(&self.command)
            .args(&self.args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        let request = serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": method, "params": params
        });
        let line = serde_json::to_string(&request).unwrap() + "\n";
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(line.as_bytes())
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        drop(child.stdin.take());
        let stdout = child.stdout.take().unwrap();
        let mut reader = std::io::BufReader::new(stdout);
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        let resp: serde_json::Value = serde_json::from_str(&response_line)
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(resp["result"].clone())
    }
}
