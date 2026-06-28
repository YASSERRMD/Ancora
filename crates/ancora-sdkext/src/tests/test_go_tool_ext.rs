use std::collections::HashMap;
use crate::go_interfaces::{
    canonical_go_interface, validate_go_return, GoCallEnvelope, GoExtensionAdapter, GoMethod,
};
use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_go_canonical_interface_has_three_methods() {
    let iface = canonical_go_interface();
    assert_eq!(iface.interface_name, "ToolExtension");
    assert_eq!(iface.methods.len(), 3);
}

#[test]
fn test_go_method_display() {
    assert_eq!(GoMethod::Meta.to_string(), "meta");
    assert_eq!(GoMethod::Execute.to_string(), "execute");
    assert_eq!(GoMethod::HealthCheck.to_string(), "health_check");
}

#[test]
fn test_go_adapter_dispatch_registered_handler() {
    let meta = ToolMeta::new("go_test_tool", "A test Go tool.", "1.0.0");
    let mut adapter = GoExtensionAdapter::new(meta);
    adapter.register_handler("execute", |payload| {
        Ok(format!("echoed: {payload}"))
    });

    let envelope = GoCallEnvelope {
        extension_name: "go_test_tool".to_string(),
        method: GoMethod::Execute,
        payload: r#"{"message":"hi"}"#.to_string(),
    };

    let result = adapter.dispatch(&envelope);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("echoed"));
}

#[test]
fn test_go_adapter_dispatch_unregistered_handler() {
    let meta = ToolMeta::new("go_no_handler", "No handler.", "1.0.0");
    let adapter = GoExtensionAdapter::new(meta);

    let envelope = GoCallEnvelope {
        extension_name: "go_no_handler".to_string(),
        method: GoMethod::Execute,
        payload: "{}".to_string(),
    };

    let result = adapter.dispatch(&envelope);
    assert!(matches!(result, Err(ExtensionError::NotSupported(_))));
}

#[test]
fn test_validate_go_return_null_ok() {
    assert!(validate_go_return(&Value::Null).is_ok());
}

#[test]
fn test_validate_go_return_nonempty_map_ok() {
    let mut m = HashMap::new();
    m.insert("key".to_string(), Value::string("val"));
    assert!(validate_go_return(&Value::Map(m)).is_ok());
}

#[test]
fn test_validate_go_return_empty_map_err() {
    let result = validate_go_return(&Value::Map(HashMap::new()));
    assert!(result.is_err());
}
