use std::collections::HashMap;
use std::sync::Arc;

use crate::error::ToolError;
use crate::schema::validate_input;
use crate::tool::Tool;

/// Holds registered tools and dispatches calls by name.
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Register a tool under its own name.
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_owned(), tool);
    }

    /// Return all registered tools in arbitrary order.
    pub fn list(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.values().cloned().collect()
    }

    /// Call a tool by name, validating input before execution.
    pub fn call(
        &self,
        name: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, ToolError> {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_owned()))?;
        validate_input(&tool.input_schema(), input)?;
        tool.call(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::ToolEffect;

    struct EchoTool;

    impl Tool for EchoTool {
        fn name(&self) -> &str {
            "echo"
        }
        fn description(&self) -> &str {
            "echoes the input message"
        }
        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({ "type": "object", "required": ["message"] })
        }
        fn effect(&self) -> ToolEffect {
            ToolEffect::ReadOnly
        }
        fn call(&self, input: &serde_json::Value) -> Result<serde_json::Value, ToolError> {
            let msg = input["message"].as_str().unwrap_or("");
            Ok(serde_json::json!({ "output": msg }))
        }
    }

    #[test]
    fn local_tool_executes_with_validated_input() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(EchoTool));
        let input = serde_json::json!({ "message": "hello" });
        let result = registry.call("echo", &input).unwrap();
        assert_eq!(result["output"], "hello");
    }

    #[test]
    fn missing_required_field_fails_validation() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(EchoTool));
        let bad_input = serde_json::json!({});
        let err = registry.call("echo", &bad_input).unwrap_err();
        assert!(matches!(err, ToolError::ValidationFailed(_)));
    }

    #[test]
    fn unknown_tool_returns_not_found_error() {
        let registry = ToolRegistry::new();
        let err = registry.call("ghost", &serde_json::json!({})).unwrap_err();
        assert!(matches!(err, ToolError::NotFound(_)));
    }
}
