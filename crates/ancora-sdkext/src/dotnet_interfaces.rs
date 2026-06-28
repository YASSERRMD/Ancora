/// .NET extension interface definitions modelled in Rust.
///
/// .NET / C# extensions reach Ancora through a thin C ABI shim compiled from
/// a .NET NativeAOT binary.  This module documents and validates that contract
/// from the Rust side.

use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// .NET type system mirror
// ---------------------------------------------------------------------------

/// .NET types that can appear in extension argument / return schemas.
#[derive(Debug, Clone, PartialEq)]
pub enum DotNetType {
    String,
    Int32,
    Int64,
    Double,
    Bool,
    Object,
    Dictionary(Box<DotNetType>, Box<DotNetType>),
    List(Box<DotNetType>),
    Void,
}

impl DotNetType {
    /// Return the C# type name.
    pub fn csharp_name(&self) -> String {
        match self {
            DotNetType::String => "string".to_string(),
            DotNetType::Int32 => "int".to_string(),
            DotNetType::Int64 => "long".to_string(),
            DotNetType::Double => "double".to_string(),
            DotNetType::Bool => "bool".to_string(),
            DotNetType::Object => "object".to_string(),
            DotNetType::Dictionary(k, v) => {
                format!("Dictionary<{}, {}>", k.csharp_name(), v.csharp_name())
            }
            DotNetType::List(t) => format!("List<{}>", t.csharp_name()),
            DotNetType::Void => "void".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Interface description
// ---------------------------------------------------------------------------

/// Describes the `IToolExtension` C# interface.
#[derive(Debug, Clone)]
pub struct DotNetInterfaceDescriptor {
    pub interface_name: String,
    pub namespace: String,
    pub methods: Vec<DotNetMethodDescriptor>,
}

#[derive(Debug, Clone)]
pub struct DotNetMethodDescriptor {
    pub name: String,
    pub params: Vec<(String, DotNetType)>,
    pub return_type: DotNetType,
    pub is_async: bool,
}

/// Return the canonical C# interface descriptor for Ancora tool extensions.
pub fn canonical_dotnet_interface() -> DotNetInterfaceDescriptor {
    DotNetInterfaceDescriptor {
        interface_name: "IToolExtension".to_string(),
        namespace: "Ancora.Sdk".to_string(),
        methods: vec![
            DotNetMethodDescriptor {
                name: "GetMeta".to_string(),
                params: vec![],
                return_type: DotNetType::Object,
                is_async: false,
            },
            DotNetMethodDescriptor {
                name: "ExecuteAsync".to_string(),
                params: vec![(
                    "args".to_string(),
                    DotNetType::Dictionary(
                        Box::new(DotNetType::String),
                        Box::new(DotNetType::Object),
                    ),
                )],
                return_type: DotNetType::Object,
                is_async: true,
            },
            DotNetMethodDescriptor {
                name: "HealthCheckAsync".to_string(),
                params: vec![],
                return_type: DotNetType::Void,
                is_async: true,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// .NET extension adapter
// ---------------------------------------------------------------------------

/// Adapter that wraps a .NET extension.
pub struct DotNetExtensionAdapter {
    meta: ToolMeta,
    execute_fn:
        Box<dyn Fn(HashMap<String, Value>) -> Result<Value, ExtensionError> + Send + Sync>,
}

impl DotNetExtensionAdapter {
    pub fn new(
        meta: ToolMeta,
        execute_fn: impl Fn(HashMap<String, Value>) -> Result<Value, ExtensionError>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        DotNetExtensionAdapter {
            meta,
            execute_fn: Box::new(execute_fn),
        }
    }

    pub fn meta(&self) -> &ToolMeta {
        &self.meta
    }

    pub fn execute(&self, args: HashMap<String, Value>) -> Result<Value, ExtensionError> {
        (self.execute_fn)(args)
    }
}

// ---------------------------------------------------------------------------
// ABI helpers
// ---------------------------------------------------------------------------

/// Map a Rust `Value` to the closest `DotNetType`.
pub fn rust_value_to_dotnet_type(value: &Value) -> DotNetType {
    match value {
        Value::Str(_) => DotNetType::String,
        Value::Int(_) => DotNetType::Int64,
        Value::Float(_) => DotNetType::Double,
        Value::Bool(_) => DotNetType::Bool,
        Value::Array(_) => DotNetType::List(Box::new(DotNetType::Object)),
        Value::Map(_) => {
            DotNetType::Dictionary(Box::new(DotNetType::String), Box::new(DotNetType::Object))
        }
        Value::Null => DotNetType::Object,
    }
}
