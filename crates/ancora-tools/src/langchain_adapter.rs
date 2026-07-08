use std::sync::Arc;

use crate::error::ToolError;
use crate::tool::{Tool, ToolEffect};

/// Minimal representation of a LangChain-style tool definition.
///
/// LangChain tools expose `name`, `description`, and a `run(input) -> output`
/// callable.  This struct captures that shape so existing LangChain tool
/// definitions can be wrapped as Ancora `Tool` implementations without
/// rewriting their logic.
pub struct LangchainTool {
    pub name: String,
    pub description: String,
    pub args_schema: Option<serde_json::Value>,
    pub run: Box<dyn Fn(&str) -> Result<String, String> + Send + Sync>,
}

impl LangchainTool {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        run: impl Fn(&str) -> Result<String, String> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            args_schema: None,
            run: Box::new(run),
        }
    }

    pub fn with_schema(mut self, schema: serde_json::Value) -> Self {
        self.args_schema = Some(schema);
        self
    }
}

/// Wrap a `LangchainTool` as an Ancora `Tool`.
///
/// LangChain tools take a plain string argument (`input`).  The adapter
/// expects the Ancora input JSON to carry that string in a field named
/// `input`.  If the field is missing, the entire JSON is serialised as the
/// input string.
struct LangchainToolAdapter {
    inner: Arc<LangchainTool>,
}

impl Tool for LangchainToolAdapter {
    fn name(&self) -> &str {
        &self.inner.name
    }

    fn description(&self) -> &str {
        &self.inner.description
    }

    fn input_schema(&self) -> serde_json::Value {
        self.inner.args_schema.clone().unwrap_or_else(|| {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "input": { "type": "string", "description": "The tool input." }
                },
                "required": ["input"]
            })
        })
    }

    fn effect(&self) -> ToolEffect {
        ToolEffect::Write
    }

    fn call(&self, input: &serde_json::Value) -> Result<serde_json::Value, ToolError> {
        let arg = if let Some(s) = input.get("input").and_then(|v| v.as_str()) {
            s.to_owned()
        } else {
            serde_json::to_string(input).map_err(|e| ToolError::ExecutionFailed(e.to_string()))?
        };

        let output = (self.inner.run)(&arg).map_err(ToolError::ExecutionFailed)?;

        Ok(serde_json::json!({ "output": output }))
    }
}

/// Convert a `LangchainTool` into an Ancora `Arc<dyn Tool>`.
pub fn from_langchain(tool: LangchainTool) -> Arc<dyn Tool> {
    Arc::new(LangchainToolAdapter {
        inner: Arc::new(tool),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::ToolRegistry;

    #[test]
    fn langchain_tool_wraps_as_ancora_tool() {
        let lc = LangchainTool::new("greet", "greets the user", |input| {
            Ok(format!("Hello, {}!", input))
        });
        let tool = from_langchain(lc);
        assert_eq!(tool.name(), "greet");
        assert_eq!(tool.description(), "greets the user");
    }

    #[test]
    fn langchain_tool_default_schema_requires_input_field() {
        let lc = LangchainTool::new("t", "desc", |_| Ok("ok".into()));
        let tool = from_langchain(lc);
        let schema = tool.input_schema();
        assert_eq!(schema["required"][0], "input");
    }

    #[test]
    fn langchain_tool_custom_schema_is_preserved() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["query"],
            "properties": { "query": { "type": "string" } }
        });
        let lc = LangchainTool::new("search", "searches", |input| Ok(input.to_owned()))
            .with_schema(schema.clone());
        let tool = from_langchain(lc);
        assert_eq!(tool.input_schema()["required"][0], "query");
    }

    #[test]
    fn langchain_tool_call_extracts_input_field() {
        let lc = LangchainTool::new("echo", "echoes", |s| Ok(s.to_owned()));
        let tool = from_langchain(lc);
        let result = tool.call(&serde_json::json!({ "input": "world" })).unwrap();
        assert_eq!(result["output"], "world");
    }

    #[test]
    fn langchain_tool_call_falls_back_to_full_json() {
        let lc = LangchainTool::new("echo", "echoes", |s| Ok(s.to_owned()));
        let tool = from_langchain(lc);
        let result = tool.call(&serde_json::json!({ "query": "hello" })).unwrap();
        assert!(result["output"].as_str().unwrap().contains("query"));
    }

    #[test]
    fn langchain_tool_error_propagates_as_execution_failed() {
        let lc = LangchainTool::new("failing", "always fails", |_| Err("boom".into()));
        let tool = from_langchain(lc);
        let err = tool.call(&serde_json::json!({ "input": "x" })).unwrap_err();
        assert!(matches!(err, ToolError::ExecutionFailed(_)));
    }

    #[test]
    fn langchain_tool_registers_in_ancora_registry() {
        let mut registry = ToolRegistry::new();
        let lc = LangchainTool::new("calc", "calculates", |input| {
            Ok(format!("result: {}", input.len()))
        });
        registry.register(from_langchain(lc));
        let result = registry
            .call("calc", &serde_json::json!({ "input": "abc" }))
            .unwrap();
        assert_eq!(result["output"], "result: 3");
    }
}
