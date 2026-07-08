use crate::py_classes::{
    validate_manifest, PyArgSpec, PyDecoratorDescriptor, PyExtensionAdapter, PyExtensionManifest,
    PyType,
};
use crate::rs_traits::{ExtensionError, Value};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_manifest_to_meta() {
    let manifest =
        PyExtensionManifest::new("py_tool", "A Python tool.", "2.0.0", "mypkg.tools.PyTool");
    let meta = manifest.to_meta();
    assert_eq!(meta.name, "py_tool");
    assert_eq!(meta.version, "2.0.0");
}

#[test]
fn test_manifest_with_requirement() {
    let manifest = PyExtensionManifest::new("t", "d", "1", "m")
        .with_requirement("numpy>=1.20")
        .with_requirement("pandas");
    assert_eq!(manifest.requirements.len(), 2);
}

#[test]
fn test_validate_manifest_valid() {
    let manifest = PyExtensionManifest::new("tool", "desc", "1.0.0", "pkg.Tool");
    let errors = validate_manifest(&manifest);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn test_validate_manifest_missing_fields() {
    let manifest = PyExtensionManifest {
        name: "".to_string(),
        description: "ok".to_string(),
        version: "".to_string(),
        class_path: "".to_string(),
        requirements: vec![],
    };
    let errors = validate_manifest(&manifest);
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("name")));
    assert!(errors.iter().any(|e| e.contains("version")));
    assert!(errors.iter().any(|e| e.contains("class_path")));
}

#[test]
fn test_adapter_execute_success() {
    let manifest = PyExtensionManifest::new("py_echo", "Echo", "1.0.0", "pkg.Echo");
    let adapter = PyExtensionAdapter::new(manifest, |args| {
        let msg = args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("missing message".to_string()))?;
        Ok(Value::string(msg))
    });

    let mut args = HashMap::new();
    args.insert("message".to_string(), Value::string("world"));
    let result = adapter.execute(args).unwrap();
    assert_eq!(result, Value::string("world"));
}

#[test]
fn test_py_type_hints() {
    assert_eq!(PyType::Str.type_hint(), "str");
    assert_eq!(PyType::Dict.type_hint(), "dict");
    assert_eq!(PyType::Any.type_hint(), "Any");
}

#[test]
fn test_decorator_descriptor_fields() {
    let mut schema = HashMap::new();
    schema.insert(
        "text".to_string(),
        PyArgSpec {
            py_type: PyType::Str,
            required: true,
            description: "Input text.".to_string(),
        },
    );
    let desc = PyDecoratorDescriptor {
        tool_name: "my_tool".to_string(),
        func_name: "run".to_string(),
        module: "mymod".to_string(),
        arg_schema: schema,
    };
    assert!(desc.arg_schema.contains_key("text"));
}
