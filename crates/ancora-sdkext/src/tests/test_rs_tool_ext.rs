use crate::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// A concrete Rust extension for testing
// ---------------------------------------------------------------------------

struct GreetTool;

impl ToolExtension for GreetTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("greet", "Produces a greeting.", "1.0.0")
    }

    fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'name' required".to_string()))?;
        Ok(Value::string(format!("Hello, {name}!")))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_meta_fields() {
    let tool = GreetTool;
    let meta = tool.meta();
    assert_eq!(meta.name, "greet");
    assert_eq!(meta.version, "1.0.0");
    assert!(!meta.description.is_empty());
}

#[test]
fn test_execute_success() {
    let tool = GreetTool;
    let mut args = HashMap::new();
    args.insert("name".to_string(), Value::string("Alice"));
    let result = tool.execute(args).unwrap();
    assert_eq!(result, Value::string("Hello, Alice!"));
}

#[test]
fn test_execute_missing_arg() {
    let tool = GreetTool;
    let result = tool.execute(HashMap::new());
    assert!(matches!(result, Err(ExtensionError::InvalidArgument(_))));
}

#[test]
fn test_health_check_default_ok() {
    let tool = GreetTool;
    assert!(tool.health_check().is_ok());
}

#[test]
fn test_value_constructors() {
    let s = Value::string("hello");
    assert_eq!(s.as_str(), Some("hello"));

    let n = Value::Int(42);
    assert_eq!(n.as_int(), Some(42));
}

#[test]
fn test_extension_error_display() {
    let e = ExtensionError::InvalidArgument("bad arg".to_string());
    assert!(e.to_string().contains("bad arg"));

    let e2 = ExtensionError::Timeout;
    assert!(e2.to_string().contains("Timeout"));
}
