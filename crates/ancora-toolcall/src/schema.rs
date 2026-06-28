use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool definition available for the agent to call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub is_async: bool,
    pub timeout_ms: u64,
}

impl ToolDef {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters: Value::Object(Default::default()),
            is_async: false,
            timeout_ms: 5000,
        }
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn async_tool(mut self) -> Self {
        self.is_async = true;
        self
    }
}

/// A pending tool invocation from the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub call_id: String,
    pub tool_name: String,
    pub arguments: Value,
}

impl ToolCall {
    pub fn new(call_id: &str, tool_name: &str, arguments: Value) -> Self {
        Self {
            call_id: call_id.to_string(),
            tool_name: tool_name.to_string(),
            arguments,
        }
    }
}

/// The result of executing a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub tool_name: String,
    pub output: Value,
    pub is_error: bool,
    pub elapsed_ms: u64,
}

impl ToolResult {
    pub fn ok(call_id: &str, tool_name: &str, output: Value, elapsed_ms: u64) -> Self {
        Self { call_id: call_id.to_string(), tool_name: tool_name.to_string(), output, is_error: false, elapsed_ms }
    }

    pub fn error(call_id: &str, tool_name: &str, reason: &str) -> Self {
        Self {
            call_id: call_id.to_string(),
            tool_name: tool_name.to_string(),
            output: Value::String(reason.to_string()),
            is_error: true,
            elapsed_ms: 0,
        }
    }
}
