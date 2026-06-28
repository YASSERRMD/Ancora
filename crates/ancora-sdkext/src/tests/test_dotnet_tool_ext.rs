use std::collections::HashMap;
use crate::dotnet_interfaces::{
    canonical_dotnet_interface, rust_value_to_dotnet_type, DotNetExtensionAdapter, DotNetType,
};
use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_dotnet_type_csharp_names() {
    assert_eq!(DotNetType::String.csharp_name(), "string");
    assert_eq!(DotNetType::Int32.csharp_name(), "int");
    assert_eq!(DotNetType::Int64.csharp_name(), "long");
    assert_eq!(DotNetType::Bool.csharp_name(), "bool");
    assert_eq!(DotNetType::Void.csharp_name(), "void");
    assert_eq!(
        DotNetType::Dictionary(Box::new(DotNetType::String), Box::new(DotNetType::Object))
            .csharp_name(),
        "Dictionary<string, object>"
    );
    assert_eq!(
        DotNetType::List(Box::new(DotNetType::Int32)).csharp_name(),
        "List<int>"
    );
}

#[test]
fn test_canonical_dotnet_interface() {
    let iface = canonical_dotnet_interface();
    assert_eq!(iface.interface_name, "IToolExtension");
    assert_eq!(iface.namespace, "Ancora.Sdk");
    assert_eq!(iface.methods.len(), 3);

    let async_methods: Vec<_> = iface.methods.iter().filter(|m| m.is_async).collect();
    assert_eq!(async_methods.len(), 2, "ExecuteAsync and HealthCheckAsync should be async");
}

#[test]
fn test_rust_value_to_dotnet_type() {
    assert_eq!(rust_value_to_dotnet_type(&Value::string("x")), DotNetType::String);
    assert_eq!(rust_value_to_dotnet_type(&Value::Int(1)), DotNetType::Int64);
    assert_eq!(rust_value_to_dotnet_type(&Value::Float(1.0)), DotNetType::Double);
    assert_eq!(rust_value_to_dotnet_type(&Value::Bool(true)), DotNetType::Bool);
    assert_eq!(rust_value_to_dotnet_type(&Value::Null), DotNetType::Object);
}

#[test]
fn test_dotnet_adapter_execute_success() {
    let meta = ToolMeta::new("dotnet_upper", "Uppercases a string.", "1.0.0");
    let adapter = DotNetExtensionAdapter::new(meta, |args| {
        let text = args
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'text' required".to_string()))?;
        Ok(Value::string(text.to_uppercase()))
    });

    let mut args = HashMap::new();
    args.insert("text".to_string(), Value::string("hello"));
    let result = adapter.execute(args).unwrap();
    assert_eq!(result, Value::string("HELLO"));
}

#[test]
fn test_dotnet_adapter_meta() {
    let meta = ToolMeta::new("dotnet_tool", "desc", "1.0.0");
    let adapter = DotNetExtensionAdapter::new(meta, |_| Ok(Value::Null));
    assert_eq!(adapter.meta().name, "dotnet_tool");
}
