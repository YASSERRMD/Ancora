/// Conformance kit for tool extensions.

use std::collections::HashMap;

/// The input/output schema for a tool.
#[derive(Debug, Clone)]
pub struct ToolSchema {
    pub input_fields: Vec<String>,
    pub output_fields: Vec<String>,
}

/// Trait that every tool extension must satisfy.
pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> ToolSchema;
    fn call(&self, args: HashMap<String, String>) -> Result<HashMap<String, String>, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against a [`Tool`].
pub struct ToolKit;

impl ToolKit {
    pub fn new() -> Self {
        ToolKit
    }

    pub fn run<T: Tool>(&self, tool: &T) -> Vec<CheckResult> {
        vec![
            self.check_name(tool),
            self.check_description(tool),
            self.check_schema(tool),
            self.check_call(tool),
        ]
    }

    fn check_name<T: Tool>(&self, tool: &T) -> CheckResult {
        if tool.name().is_empty() {
            CheckResult {
                name: "tool_name_nonempty".into(),
                passed: false,
                message: "Tool name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "tool_name_nonempty".into(),
                passed: true,
                message: format!("Tool name: {}", tool.name()),
            }
        }
    }

    fn check_description<T: Tool>(&self, tool: &T) -> CheckResult {
        if tool.description().is_empty() {
            CheckResult {
                name: "tool_description_nonempty".into(),
                passed: false,
                message: "Tool description must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "tool_description_nonempty".into(),
                passed: true,
                message: "Description present".into(),
            }
        }
    }

    fn check_schema<T: Tool>(&self, tool: &T) -> CheckResult {
        let schema = tool.schema();
        if schema.input_fields.is_empty() {
            CheckResult {
                name: "tool_schema_has_inputs".into(),
                passed: false,
                message: "Tool schema must declare at least one input field".into(),
            }
        } else {
            CheckResult {
                name: "tool_schema_has_inputs".into(),
                passed: true,
                message: format!("{} input field(s)", schema.input_fields.len()),
            }
        }
    }

    fn check_call<T: Tool>(&self, tool: &T) -> CheckResult {
        let schema = tool.schema();
        let mut args = HashMap::new();
        for field in &schema.input_fields {
            args.insert(field.clone(), "test_value".into());
        }
        match tool.call(args) {
            Ok(_) => CheckResult {
                name: "tool_call_succeeds".into(),
                passed: true,
                message: "call() returned Ok".into(),
            },
            Err(e) => CheckResult {
                name: "tool_call_succeeds".into(),
                passed: false,
                message: format!("call() errored: {e}"),
            },
        }
    }
}

impl Default for ToolKit {
    fn default() -> Self {
        Self::new()
    }
}
