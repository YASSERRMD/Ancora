/// Rust extension traits and macros for the Ancora SDK.
///
/// Provides the core trait surface that Rust-based extensions must implement,
/// plus lightweight derive-helper utilities so boilerplate stays minimal.
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A plain JSON-like value used for tool arguments and results.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Value {
    /// Convenience constructor for a string value.
    pub fn string(s: impl Into<String>) -> Self {
        Value::Str(s.into())
    }

    /// Return the inner string slice if this is a `Str` variant.
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }

    /// Return the inner i64 if this is an `Int` variant.
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Extension traits
// ---------------------------------------------------------------------------

/// Metadata describing a tool extension.
#[derive(Debug, Clone)]
pub struct ToolMeta {
    pub name: String,
    pub description: String,
    pub version: String,
}

impl ToolMeta {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        ToolMeta {
            name: name.into(),
            description: description.into(),
            version: version.into(),
        }
    }
}

/// The core trait every Ancora tool extension must implement.
pub trait ToolExtension: Send + Sync {
    /// Return static metadata for this extension.
    fn meta(&self) -> ToolMeta;

    /// Execute the tool with the provided arguments.
    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError>;

    /// Optional health-check; returns `Ok(())` if the extension is healthy.
    fn health_check(&self) -> Result<(), ExtensionError> {
        Ok(())
    }
}

/// The core trait for agent-level extensions (multi-step reasoning hooks).
pub trait AgentExtension: Send + Sync {
    fn meta(&self) -> ToolMeta;

    /// Called before each agent step with the current context.
    fn before_step(&self, ctx: &AgentContext) -> Result<(), ExtensionError>;

    /// Called after each agent step.
    fn after_step(&self, ctx: &AgentContext, output: &Value) -> Result<(), ExtensionError>;
}

/// Contextual information passed to agent extension hooks.
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub step: u32,
    pub session_id: String,
    pub metadata: HashMap<String, Value>,
}

impl AgentContext {
    pub fn new(step: u32, session_id: impl Into<String>) -> Self {
        AgentContext {
            step,
            session_id: session_id.into(),
            metadata: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors returned by extension implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionError {
    InvalidArgument(String),
    ExecutionFailed(String),
    NotSupported(String),
    Timeout,
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::InvalidArgument(msg) => write!(f, "InvalidArgument: {msg}"),
            ExtensionError::ExecutionFailed(msg) => write!(f, "ExecutionFailed: {msg}"),
            ExtensionError::NotSupported(msg) => write!(f, "NotSupported: {msg}"),
            ExtensionError::Timeout => write!(f, "Timeout"),
        }
    }
}

impl std::error::Error for ExtensionError {}

// ---------------------------------------------------------------------------
// Macro helpers (declarative)
// ---------------------------------------------------------------------------

/// Declare a simple tool extension struct with name, description, and version.
///
/// ```
/// use ancora_sdkext::simple_tool;
/// simple_tool!(MyTool, "my_tool", "Does a thing", "1.0.0");
/// ```
#[macro_export]
macro_rules! simple_tool {
    ($struct_name:ident, $name:expr, $desc:expr, $ver:expr) => {
        pub struct $struct_name;

        impl $crate::rs_traits::ToolExtension for $struct_name {
            fn meta(&self) -> $crate::rs_traits::ToolMeta {
                $crate::rs_traits::ToolMeta::new($name, $desc, $ver)
            }

            fn execute(
                &self,
                _args: std::collections::HashMap<String, $crate::rs_traits::Value>,
            ) -> Result<$crate::rs_traits::Value, $crate::rs_traits::ExtensionError> {
                Ok($crate::rs_traits::Value::Null)
            }
        }
    };
}
