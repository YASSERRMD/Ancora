/// Reference implementation: Python extension examples (Rust-side representation).
///
/// Each struct wraps a simulated Python extension through the `PyExtensionAdapter`.
/// In production the adapter calls into the PyO3 runtime; here closures stand in
/// so tests can run without a Python interpreter.
use std::collections::HashMap;

use crate::py_classes::{PyExtensionAdapter, PyExtensionManifest};
use crate::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Python echo tool example
// ---------------------------------------------------------------------------

/// Produce a `PyExtensionAdapter` that simulates the canonical Python echo tool.
pub fn make_py_echo_tool() -> PyExtensionAdapter {
    let manifest = PyExtensionManifest::new(
        "py_echo",
        "Python implementation of the echo tool.",
        "1.0.0",
        "ancora_examples.echo.EchoTool",
    );
    PyExtensionAdapter::new(manifest, |args| {
        let msg = args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'message' is required".to_string()))?;
        Ok(Value::string(format!("[python] {msg}")))
    })
}

// ---------------------------------------------------------------------------
// Python sentiment tool example
// ---------------------------------------------------------------------------

/// A Python-style sentiment classifier extension (simulated).
pub fn make_py_sentiment_tool() -> PyExtensionAdapter {
    let manifest = PyExtensionManifest::new(
        "py_sentiment",
        "Python sentiment classifier.",
        "1.0.0",
        "ancora_examples.sentiment.SentimentTool",
    )
    .with_requirement("transformers>=4.0");

    PyExtensionAdapter::new(manifest, |args| {
        let text = args
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'text' is required".to_string()))?;

        // Naively simulate sentiment by checking for common positive/negative words.
        let lower = text.to_lowercase();
        let label =
            if lower.contains("great") || lower.contains("good") || lower.contains("excellent") {
                "positive"
            } else if lower.contains("bad") || lower.contains("terrible") || lower.contains("poor")
            {
                "negative"
            } else {
                "neutral"
            };

        let mut result = HashMap::new();
        result.insert("label".to_string(), Value::string(label));
        result.insert("score".to_string(), Value::Float(0.95));
        Ok(Value::Map(result))
    })
}

// ---------------------------------------------------------------------------
// Thin ToolExtension wrappers for registry integration
// ---------------------------------------------------------------------------

/// A thin `ToolExtension` wrapper around a `PyExtensionAdapter`.
pub struct PyToolWrapper {
    adapter: PyExtensionAdapter,
}

impl PyToolWrapper {
    pub fn new(adapter: PyExtensionAdapter) -> Self {
        PyToolWrapper { adapter }
    }
}

impl ToolExtension for PyToolWrapper {
    fn meta(&self) -> ToolMeta {
        self.adapter.meta()
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        self.adapter.execute(args)
    }
}
