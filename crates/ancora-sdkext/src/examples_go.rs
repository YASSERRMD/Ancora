/// Reference implementation: Go extension examples (Rust-side representation).
///
/// Each struct in this module represents a Go extension wired up through the
/// `GoExtensionAdapter`.  In a real deployment these would be backed by a
/// compiled Go shared library; here the closures simulate the behaviour so
/// integration tests can run without a Go toolchain.
use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Go echo tool example
// ---------------------------------------------------------------------------

/// Produce a `GoExtensionAdapter` that behaves like the canonical Go echo tool.
pub fn make_go_echo_tool() -> impl ToolExtension {
    GoEchoToolAdapter::new()
}

pub struct GoEchoToolAdapter {
    meta: ToolMeta,
}

impl GoEchoToolAdapter {
    pub fn new() -> Self {
        GoEchoToolAdapter {
            meta: ToolMeta::new("go_echo", "Go implementation of the echo tool.", "1.0.0"),
        }
    }
}

impl Default for GoEchoToolAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolExtension for GoEchoToolAdapter {
    fn meta(&self) -> ToolMeta {
        self.meta.clone()
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        // Simulate what the Go extension would do.
        let msg = args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ExtensionError::InvalidArgument("'message' argument is required".to_string())
            })?;
        Ok(Value::string(format!("[go] {msg}")))
    }
}

// ---------------------------------------------------------------------------
// Go word-count tool example
// ---------------------------------------------------------------------------

/// A Go-style word-count extension.
pub struct GoWordCountAdapter {
    meta: ToolMeta,
}

impl GoWordCountAdapter {
    pub fn new() -> Self {
        GoWordCountAdapter {
            meta: ToolMeta::new(
                "go_word_count",
                "Go implementation of a word-count tool.",
                "1.0.0",
            ),
        }
    }
}

impl Default for GoWordCountAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolExtension for GoWordCountAdapter {
    fn meta(&self) -> ToolMeta {
        self.meta.clone()
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        let text = args.get("text").and_then(|v| v.as_str()).ok_or_else(|| {
            ExtensionError::InvalidArgument("'text' argument is required".to_string())
        })?;
        let count = text.split_whitespace().count() as i64;
        Ok(Value::Int(count))
    }
}
