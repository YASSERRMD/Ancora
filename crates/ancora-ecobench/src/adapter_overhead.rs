//! Framework adapter overhead measurement.
//!
//! Models the cost of translating between Ancora's internal agent representation
//! and the interface expected by an external framework (e.g., LangChain-style
//! tool arrays or OpenAI function-call schemas). No external dependencies are
//! used; the adapter logic operates on plain Rust types.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// An Ancora-native tool descriptor.
#[derive(Debug, Clone)]
pub struct AncoraToolDef {
    /// Unique name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Parameter schema (simplified as name -> type string).
    pub params: HashMap<String, String>,
}

impl AncoraToolDef {
    /// Create a new tool definition.
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            params: HashMap::new(),
        }
    }

    /// Add a parameter to this tool definition.
    pub fn with_param(mut self, name: &str, ty: &str) -> Self {
        self.params.insert(name.to_owned(), ty.to_owned());
        self
    }
}

/// A framework-facing tool representation (e.g., JSON schema-like).
#[derive(Debug, Clone)]
pub struct FrameworkTool {
    /// Tool name as exported.
    pub name: String,
    /// Rendered description.
    pub description: String,
    /// Rendered parameter list (name, type).
    pub parameters: Vec<(String, String)>,
}

/// Result of adapting a set of tool definitions.
#[derive(Debug)]
pub struct AdaptResult {
    /// The adapted tools in framework format.
    pub tools: Vec<FrameworkTool>,
    /// Total elapsed time.
    pub elapsed: Duration,
    /// Number of parameters processed.
    pub param_count: usize,
}

/// Adapt a slice of Ancora tool definitions to the framework representation.
pub fn adapt_tools(defs: &[AncoraToolDef]) -> AdaptResult {
    let start = Instant::now();
    let mut total_params = 0;

    let tools: Vec<FrameworkTool> = defs
        .iter()
        .map(|d| {
            let mut parameters: Vec<(String, String)> = d
                .params
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            // Sort for determinism.
            parameters.sort_by(|a, b| a.0.cmp(&b.0));
            total_params += parameters.len();
            FrameworkTool {
                name: d.name.clone(),
                description: d.description.clone(),
                parameters,
            }
        })
        .collect();

    AdaptResult {
        tools,
        elapsed: start.elapsed(),
        param_count: total_params,
    }
}

/// Regression threshold for adapter overhead per tool in microseconds.
pub const ADAPT_PER_TOOL_TARGET_US: u64 = 100;

/// Returns `true` if adapting `n` tools stayed within the per-tool threshold.
pub fn within_target(result: &AdaptResult, n: usize) -> bool {
    if n == 0 {
        return true;
    }
    let per_tool_us = result.elapsed.as_micros() as u64 / n as u64;
    per_tool_us <= ADAPT_PER_TOOL_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapt_produces_correct_count() {
        let defs = vec![
            AncoraToolDef::new("search", "Search the web"),
            AncoraToolDef::new("calc", "Perform arithmetic"),
        ];
        let r = adapt_tools(&defs);
        assert_eq!(r.tools.len(), 2);
    }

    #[test]
    fn params_are_counted() {
        let defs = vec![AncoraToolDef::new("t", "desc")
            .with_param("x", "string")
            .with_param("y", "integer")];
        let r = adapt_tools(&defs);
        assert_eq!(r.param_count, 2);
    }
}
