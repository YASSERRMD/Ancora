/// Python extension base-class definitions modelled in Rust.
///
/// Python extensions call into Ancora via the `ancora-py` PyO3 crate.  This
/// module holds the Rust-side mirror of the ABC (abstract base class) contract
/// that Python extension authors sub-class, plus helpers for validating Python
/// extension manifests.
use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Python extension manifest
// ---------------------------------------------------------------------------

/// The manifest emitted by a Python extension when it is first loaded.
/// Python extensions serialise this as JSON; this struct is the deserialised
/// form validated on the Rust side.
#[derive(Debug, Clone)]
pub struct PyExtensionManifest {
    pub name: String,
    pub description: String,
    pub version: String,
    /// Fully-qualified Python class name, e.g. `"mypackage.tools.EchoTool"`.
    pub class_path: String,
    /// List of Python packages required by this extension.
    pub requirements: Vec<String>,
}

impl PyExtensionManifest {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
        class_path: impl Into<String>,
    ) -> Self {
        PyExtensionManifest {
            name: name.into(),
            description: description.into(),
            version: version.into(),
            class_path: class_path.into(),
            requirements: Vec::new(),
        }
    }

    pub fn with_requirement(mut self, req: impl Into<String>) -> Self {
        self.requirements.push(req.into());
        self
    }

    /// Convert the manifest into a `ToolMeta`.
    pub fn to_meta(&self) -> ToolMeta {
        ToolMeta::new(&self.name, &self.description, &self.version)
    }
}

// ---------------------------------------------------------------------------
// Decorator descriptor
// ---------------------------------------------------------------------------

/// Represents the `@ancora_tool` decorator applied to a Python function or
/// class method.  The Python binding emits this struct when registering a
/// decorated callable.
#[derive(Debug, Clone)]
pub struct PyDecoratorDescriptor {
    pub tool_name: String,
    pub func_name: String,
    pub module: String,
    pub arg_schema: HashMap<String, PyArgSpec>,
}

/// Specification for a single argument declared via `@ancora_tool`.
#[derive(Debug, Clone)]
pub struct PyArgSpec {
    pub py_type: PyType,
    pub required: bool,
    pub description: String,
}

/// Python types that can appear in extension argument schemas.
#[derive(Debug, Clone, PartialEq)]
pub enum PyType {
    Str,
    Int,
    Float,
    Bool,
    List,
    Dict,
    Any,
}

impl PyType {
    pub fn type_hint(&self) -> &'static str {
        match self {
            PyType::Str => "str",
            PyType::Int => "int",
            PyType::Float => "float",
            PyType::Bool => "bool",
            PyType::List => "list",
            PyType::Dict => "dict",
            PyType::Any => "Any",
        }
    }
}

// ---------------------------------------------------------------------------
// Python extension adapter
// ---------------------------------------------------------------------------

type ExecuteFn = Box<dyn Fn(HashMap<String, Value>) -> Result<Value, ExtensionError> + Send + Sync>;

/// Wraps a Python extension (loaded via PyO3) and presents it as a Rust
/// `ToolExtension`.  In production the `execute_fn` field holds a PyO3
/// `Py<PyAny>` callable; in tests we use a plain Rust closure.
pub struct PyExtensionAdapter {
    manifest: PyExtensionManifest,
    execute_fn: ExecuteFn,
}

impl PyExtensionAdapter {
    pub fn new(
        manifest: PyExtensionManifest,
        execute_fn: impl Fn(HashMap<String, Value>) -> Result<Value, ExtensionError>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        PyExtensionAdapter {
            manifest,
            execute_fn: Box::new(execute_fn),
        }
    }

    pub fn meta(&self) -> ToolMeta {
        self.manifest.to_meta()
    }

    pub fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        (self.execute_fn)(args)
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Validate a `PyExtensionManifest`; return all validation errors found.
pub fn validate_manifest(manifest: &PyExtensionManifest) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.name.is_empty() {
        errors.push("name must not be empty".to_string());
    }
    if manifest.version.is_empty() {
        errors.push("version must not be empty".to_string());
    }
    if manifest.class_path.is_empty() {
        errors.push("class_path must not be empty".to_string());
    }
    errors
}
