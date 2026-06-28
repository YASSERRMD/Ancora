use std::collections::HashMap;
use std::sync::Arc;

use crate::parity::{InteropKit, ParityMatrix};
use crate::registration::{
    register_dotnet_extension, register_go_extension, register_java_extension,
    register_python_extension, register_rust_extension, register_typescript_extension,
    ExtensionRegistry, Language,
};
use crate::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};

// ---------------------------------------------------------------------------
// A trivial compliant extension used across language tests
// ---------------------------------------------------------------------------

struct NullTool {
    lang_prefix: String,
}

impl NullTool {
    fn new(prefix: &str) -> Arc<Self> {
        Arc::new(NullTool {
            lang_prefix: prefix.to_string(),
        })
    }
}

impl ToolExtension for NullTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            format!("{}_null_tool", self.lang_prefix),
            format!("Null tool for {}", self.lang_prefix),
            "1.0.0",
        )
    }

    fn execute(&self, _args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        Ok(Value::Null)
    }
}

// ---------------------------------------------------------------------------
// Interop kit checks per language
// ---------------------------------------------------------------------------

#[test]
fn test_rs_extension_passes_interop_kit() {
    let tool = NullTool::new("rs");
    let results = InteropKit::run_all(tool.as_ref());
    for r in &results {
        assert!(r.passed, "check '{}' failed: {}", r.name, r.message);
    }
}

#[test]
fn test_go_extension_passes_interop_kit() {
    let tool = NullTool::new("go");
    let results = InteropKit::run_all(tool.as_ref());
    for r in &results {
        assert!(r.passed, "check '{}' failed: {}", r.name, r.message);
    }
}

#[test]
fn test_py_extension_passes_interop_kit() {
    let tool = NullTool::new("py");
    let results = InteropKit::run_all(tool.as_ref());
    for r in &results {
        assert!(r.passed, "check '{}' failed: {}", r.name, r.message);
    }
}

#[test]
fn test_ts_extension_passes_interop_kit() {
    let tool = NullTool::new("ts");
    let results = InteropKit::run_all(tool.as_ref());
    for r in &results {
        assert!(r.passed, "check '{}' failed: {}", r.name, r.message);
    }
}

#[test]
fn test_dotnet_extension_passes_interop_kit() {
    let tool = NullTool::new("dotnet");
    let results = InteropKit::run_all(tool.as_ref());
    for r in &results {
        assert!(r.passed, "check '{}' failed: {}", r.name, r.message);
    }
}

#[test]
fn test_java_extension_passes_interop_kit() {
    let tool = NullTool::new("java");
    let results = InteropKit::run_all(tool.as_ref());
    for r in &results {
        assert!(r.passed, "check '{}' failed: {}", r.name, r.message);
    }
}

// ---------------------------------------------------------------------------
// Registry and parity matrix
// ---------------------------------------------------------------------------

#[test]
fn test_registry_register_and_dispatch() {
    let registry = ExtensionRegistry::new();
    let tool = NullTool::new("reg");
    register_rust_extension(&registry, tool).unwrap();
    let result = registry.dispatch("reg_null_tool", HashMap::new());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Null);
}

#[test]
fn test_registry_duplicate_rejected() {
    let registry = ExtensionRegistry::new();
    let t1 = NullTool::new("dup");
    let t2 = NullTool::new("dup");
    register_rust_extension(&registry, t1).unwrap();
    let err = register_rust_extension(&registry, t2);
    assert!(err.is_err());
}

#[test]
fn test_registry_list() {
    let registry = ExtensionRegistry::new();
    assert_eq!(registry.len(), 0);
    register_rust_extension(&registry, NullTool::new("list1")).unwrap();
    register_go_extension(&registry, NullTool::new("list2")).unwrap();
    assert_eq!(registry.len(), 2);
    let metas = registry.list();
    assert_eq!(metas.len(), 2);
}

#[test]
fn test_parity_matrix_records_and_queries() {
    let tool = NullTool::new("parity");
    let results = InteropKit::run_all(tool.as_ref());
    let mut matrix = ParityMatrix::new();
    matrix.record("parity_null_tool", Language::Rust, &results);
    matrix.record("parity_null_tool", Language::Go, &results);
    assert!(matrix.all_pass("parity_null_tool"));
    let langs = matrix.passing_languages("parity_null_tool");
    assert_eq!(langs.len(), 2);
}

#[test]
fn test_per_language_registration_helpers() {
    let registry = ExtensionRegistry::new();
    register_rust_extension(&registry, NullTool::new("lang_rs")).unwrap();
    register_go_extension(&registry, NullTool::new("lang_go")).unwrap();
    register_python_extension(&registry, NullTool::new("lang_py")).unwrap();
    register_typescript_extension(&registry, NullTool::new("lang_ts")).unwrap();
    register_dotnet_extension(&registry, NullTool::new("lang_dotnet")).unwrap();
    register_java_extension(&registry, NullTool::new("lang_java")).unwrap();
    assert_eq!(registry.len(), 6);
}
