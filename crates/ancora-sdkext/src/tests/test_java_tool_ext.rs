use std::collections::HashMap;
use crate::java_interfaces::{
    build_jni_descriptor, canonical_java_interface, rust_value_to_java_type,
    JavaExtensionAdapter, JavaMethodDescriptor, JavaType,
};
use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_java_type_names() {
    assert_eq!(JavaType::String.java_name(), "String");
    assert_eq!(JavaType::Int.java_name(), "int");
    assert_eq!(JavaType::Long.java_name(), "long");
    assert_eq!(JavaType::Boolean.java_name(), "boolean");
    assert_eq!(JavaType::Void.java_name(), "void");
    assert_eq!(
        JavaType::Map(Box::new(JavaType::String), Box::new(JavaType::Object)).java_name(),
        "Map<String, Object>"
    );
    assert_eq!(
        JavaType::List(Box::new(JavaType::String)).java_name(),
        "List<String>"
    );
}

#[test]
fn test_jni_descriptors() {
    assert_eq!(JavaType::String.jni_descriptor(), "Ljava/lang/String;");
    assert_eq!(JavaType::Int.jni_descriptor(), "I");
    assert_eq!(JavaType::Long.jni_descriptor(), "J");
    assert_eq!(JavaType::Double.jni_descriptor(), "D");
    assert_eq!(JavaType::Boolean.jni_descriptor(), "Z");
    assert_eq!(JavaType::Void.jni_descriptor(), "V");
}

#[test]
fn test_build_jni_descriptor_no_params() {
    let method = JavaMethodDescriptor {
        name: "healthCheck".to_string(),
        params: vec![],
        return_type: JavaType::Void,
    };
    assert_eq!(build_jni_descriptor(&method), "()V");
}

#[test]
fn test_build_jni_descriptor_with_params() {
    let method = JavaMethodDescriptor {
        name: "execute".to_string(),
        params: vec![
            ("args".to_string(), JavaType::Map(Box::new(JavaType::String), Box::new(JavaType::Object)))
        ],
        return_type: JavaType::Object,
    };
    let desc = build_jni_descriptor(&method);
    assert!(desc.starts_with('('));
    assert!(desc.ends_with("Ljava/lang/Object;"));
}

#[test]
fn test_canonical_java_interface() {
    let iface = canonical_java_interface();
    assert_eq!(iface.interface_name, "ToolExtension");
    assert_eq!(iface.package, "io.ancora.sdk");
    assert_eq!(iface.methods.len(), 3);
}

#[test]
fn test_rust_value_to_java_type() {
    assert_eq!(rust_value_to_java_type(&Value::string("x")), JavaType::String);
    assert_eq!(rust_value_to_java_type(&Value::Int(1)), JavaType::Long);
    assert_eq!(rust_value_to_java_type(&Value::Float(1.0)), JavaType::Double);
    assert_eq!(rust_value_to_java_type(&Value::Bool(false)), JavaType::Boolean);
    assert_eq!(rust_value_to_java_type(&Value::Null), JavaType::Object);
}

#[test]
fn test_java_adapter_execute() {
    let meta = ToolMeta::new("java_len", "Returns string length.", "1.0.0");
    let adapter = JavaExtensionAdapter::new(meta, |args| {
        let s = args
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'text' required".to_string()))?;
        Ok(Value::Int(s.len() as i64))
    });

    let mut args = HashMap::new();
    args.insert("text".to_string(), Value::string("hello"));
    let result = adapter.execute(args).unwrap();
    assert_eq!(result, Value::Int(5));
}
