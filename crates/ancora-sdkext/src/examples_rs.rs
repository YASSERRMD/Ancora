/// Reference implementation: Rust extension examples.
///
/// These concrete extensions demonstrate the idiomatic way to write Ancora
/// tool extensions in Rust.  They are intentionally simple so they can serve
/// as copy-paste starting points.

use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Echo tool
// ---------------------------------------------------------------------------

/// A simple tool that echoes its `message` argument back to the caller.
pub struct EchoTool;

impl ToolExtension for EchoTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("echo", "Echoes the input message.", "1.0.0")
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        let message = args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ExtensionError::InvalidArgument("'message' argument is required".to_string())
            })?;
        Ok(Value::string(message))
    }
}

// ---------------------------------------------------------------------------
// Arithmetic tool
// ---------------------------------------------------------------------------

/// A tool that adds two integers together.
pub struct AddTool;

impl ToolExtension for AddTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("add", "Adds two integers (a + b).", "1.0.0")
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        let a = args
            .get("a")
            .and_then(|v| v.as_int())
            .ok_or_else(|| ExtensionError::InvalidArgument("'a' must be an integer".to_string()))?;
        let b = args
            .get("b")
            .and_then(|v| v.as_int())
            .ok_or_else(|| ExtensionError::InvalidArgument("'b' must be an integer".to_string()))?;
        Ok(Value::Int(a + b))
    }
}

// ---------------------------------------------------------------------------
// Key-value store tool
// ---------------------------------------------------------------------------

/// An in-memory key-value store tool that supports `get` and `set` operations.
/// Note: state is intentionally not shared between instances - this is an
/// example, not a production store.
pub struct KvStoreTool {
    store: std::sync::Mutex<HashMap<String, String>>,
}

impl KvStoreTool {
    pub fn new() -> Self {
        KvStoreTool {
            store: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

impl Default for KvStoreTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolExtension for KvStoreTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("kv_store", "Simple in-memory key-value store.", "1.0.0")
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        let op = args
            .get("op")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ExtensionError::InvalidArgument("'op' argument required (get|set)".to_string())
            })?;

        match op {
            "set" => {
                let key = args
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ExtensionError::InvalidArgument("'key' required for set".to_string())
                    })?;
                let val = args
                    .get("value")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ExtensionError::InvalidArgument("'value' required for set".to_string())
                    })?;
                self.store
                    .lock()
                    .map_err(|_| {
                        ExtensionError::ExecutionFailed("lock poisoned".to_string())
                    })?
                    .insert(key.to_string(), val.to_string());
                Ok(Value::Bool(true))
            }
            "get" => {
                let key = args
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ExtensionError::InvalidArgument("'key' required for get".to_string())
                    })?;
                let result = self
                    .store
                    .lock()
                    .map_err(|_| {
                        ExtensionError::ExecutionFailed("lock poisoned".to_string())
                    })?
                    .get(key)
                    .cloned()
                    .map(Value::Str)
                    .unwrap_or(Value::Null);
                Ok(result)
            }
            other => Err(ExtensionError::InvalidArgument(format!(
                "unknown op '{other}'; expected get|set"
            ))),
        }
    }

    fn health_check(&self) -> Result<(), ExtensionError> {
        let _guard = self.store
            .lock()
            .map_err(|_| ExtensionError::ExecutionFailed("lock poisoned".to_string()))?;
        Ok(())
    }
}
