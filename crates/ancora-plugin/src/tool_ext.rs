/// Tool extension point - expose a callable function to agents.

use std::collections::HashMap;

/// JSON-like value type for tool arguments and results.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(s) = self { Some(s) } else { None }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(n) = self { Some(*n) } else { None }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self { Some(*b) } else { None }
    }
}

/// Describes one argument accepted by a tool.
#[derive(Debug, Clone)]
pub struct ArgSchema {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub arg_type: ArgType,
}

/// Primitive argument type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgType {
    String,
    Integer,
    Float,
    Boolean,
    Any,
}

/// Metadata that describes a tool to an agent.
#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub args: Vec<ArgSchema>,
}

/// Input to a tool call.
#[derive(Debug, Clone)]
pub struct ToolInput {
    pub tool_name: String,
    pub args: HashMap<String, Value>,
}

/// Result of a tool call.
#[derive(Debug, Clone)]
pub struct ToolOutput {
    pub value: Value,
    /// Optional human-readable summary.
    pub summary: Option<String>,
}

/// Error from a tool call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    MissingArg(String),
    InvalidArg { name: String, reason: String },
    ExecutionFailed(String),
    Timeout,
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::MissingArg(n) => write!(f, "missing required argument: {n}"),
            ToolError::InvalidArg { name, reason } => {
                write!(f, "invalid argument {name}: {reason}")
            }
            ToolError::ExecutionFailed(s) => write!(f, "tool execution failed: {s}"),
            ToolError::Timeout => write!(f, "tool call timed out"),
        }
    }
}

impl std::error::Error for ToolError {}

/// Trait that tool plugins must implement.
pub trait ToolPlugin: Send + Sync {
    /// Return the tool specification (schema).
    fn spec(&self) -> &ToolSpec;

    /// Execute the tool with the given input.
    fn call(&self, input: ToolInput) -> Result<ToolOutput, ToolError>;
}

/// A simple echo tool that returns its "text" argument.
pub struct EchoTool {
    spec: ToolSpec,
}

impl EchoTool {
    pub fn new() -> Self {
        Self {
            spec: ToolSpec {
                name: "echo".to_string(),
                description: "Returns the provided text unchanged.".to_string(),
                args: vec![ArgSchema {
                    name: "text".to_string(),
                    description: "Text to echo.".to_string(),
                    required: true,
                    arg_type: ArgType::String,
                }],
            },
        }
    }
}

impl Default for EchoTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolPlugin for EchoTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn call(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let text = input
            .args
            .get("text")
            .ok_or_else(|| ToolError::MissingArg("text".into()))?;
        let s = match text {
            Value::Str(s) => s.clone(),
            other => format!("{other:?}"),
        };
        Ok(ToolOutput {
            value: Value::Str(s.clone()),
            summary: Some(s),
        })
    }
}
