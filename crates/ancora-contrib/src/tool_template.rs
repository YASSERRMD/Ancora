/// ancora-contrib: tool template
///
/// Copy this module as the starting point for a new agentic tool.
/// Rename `MyTool` and implement `execute`.
use std::collections::HashMap;

/// JSON-like value that can be passed as a tool argument or returned as output.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    Text(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

/// Schema for a single tool parameter.
#[derive(Debug, Clone)]
pub struct ParamSchema {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub kind: ParamKind,
}

/// Supported parameter kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamKind {
    String,
    Number,
    Boolean,
}

/// Errors a tool call may produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    MissingArgument(String),
    InvalidArgument { name: String, reason: String },
    ExecutionFailed(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::MissingArgument(n) => write!(f, "missing required argument: {n}"),
            ToolError::InvalidArgument { name, reason } => {
                write!(f, "invalid argument '{name}': {reason}")
            }
            ToolError::ExecutionFailed(s) => write!(f, "tool execution failed: {s}"),
        }
    }
}

impl std::error::Error for ToolError {}

/// Trait all tool plugins must implement.
pub trait ToolPlugin: Send + Sync {
    /// Stable, lowercase, hyphenated identifier (e.g. "web-search").
    fn tool_id(&self) -> &str;

    /// Human-readable description for the agent to read.
    fn description(&self) -> &str;

    /// Parameter schema; the framework validates inputs before calling `execute`.
    fn params(&self) -> Vec<ParamSchema>;

    /// Execute the tool with the given arguments.
    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ToolError>;
}

// ---------------------------------------------------------------------------
// Template implementation
// ---------------------------------------------------------------------------

/// Template tool: returns the concatenation of its `left` and `right` args.
pub struct MyTool;

impl ToolPlugin for MyTool {
    fn tool_id(&self) -> &str {
        // TODO: replace with a unique identifier.
        "my-tool"
    }

    fn description(&self) -> &str {
        // TODO: write a clear, agent-readable description.
        "Concatenates two strings and returns the result."
    }

    fn params(&self) -> Vec<ParamSchema> {
        vec![
            ParamSchema {
                name: "left".to_string(),
                description: "First string".to_string(),
                required: true,
                kind: ParamKind::String,
            },
            ParamSchema {
                name: "right".to_string(),
                description: "Second string".to_string(),
                required: true,
                kind: ParamKind::String,
            },
        ]
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ToolError> {
        let left = args
            .get("left")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::MissingArgument("left".to_string()))?;
        let right = args
            .get("right")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::MissingArgument("right".to_string()))?;
        // TODO: replace with real tool logic.
        Ok(Value::Text(format!("{left}{right}")))
    }
}
