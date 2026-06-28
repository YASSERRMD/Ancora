/// TypeScript extension interface definitions modelled in Rust.
///
/// TypeScript / Node.js extensions communicate with Ancora via the `ancora-napi`
/// N-API binding.  This module contains the Rust-side mirror of the TypeScript
/// interface contract plus validation helpers used by the binding layer.

use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// TypeScript type descriptors
// ---------------------------------------------------------------------------

/// TypeScript types representable in extension argument / return schemas.
#[derive(Debug, Clone, PartialEq)]
pub enum TsType {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
    Array(Box<TsType>),
    Record(Box<TsType>, Box<TsType>),
    Unknown,
    Any,
}

impl TsType {
    /// Produce the TypeScript type annotation string for this type.
    pub fn annotation(&self) -> String {
        match self {
            TsType::String => "string".to_string(),
            TsType::Number => "number".to_string(),
            TsType::Boolean => "boolean".to_string(),
            TsType::Null => "null".to_string(),
            TsType::Undefined => "undefined".to_string(),
            TsType::Array(inner) => format!("{}[]", inner.annotation()),
            TsType::Record(k, v) => {
                format!("Record<{}, {}>", k.annotation(), v.annotation())
            }
            TsType::Unknown => "unknown".to_string(),
            TsType::Any => "any".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Interface description
// ---------------------------------------------------------------------------

/// Describes the `IToolExtension` TypeScript interface.
#[derive(Debug, Clone)]
pub struct TsInterfaceDescriptor {
    pub interface_name: String,
    pub properties: Vec<TsPropertyDescriptor>,
    pub methods: Vec<TsMethodDescriptor>,
}

#[derive(Debug, Clone)]
pub struct TsPropertyDescriptor {
    pub name: String,
    pub ts_type: TsType,
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct TsMethodDescriptor {
    pub name: String,
    pub params: Vec<(String, TsType)>,
    pub return_type: TsType,
    pub is_async: bool,
}

/// Return the canonical TypeScript interface descriptor for Ancora tool
/// extensions.
pub fn canonical_ts_interface() -> TsInterfaceDescriptor {
    TsInterfaceDescriptor {
        interface_name: "IToolExtension".to_string(),
        properties: vec![TsPropertyDescriptor {
            name: "meta".to_string(),
            ts_type: TsType::Unknown,
            readonly: true,
        }],
        methods: vec![
            TsMethodDescriptor {
                name: "execute".to_string(),
                params: vec![(
                    "args".to_string(),
                    TsType::Record(
                        Box::new(TsType::String),
                        Box::new(TsType::Unknown),
                    ),
                )],
                return_type: TsType::Unknown,
                is_async: true,
            },
            TsMethodDescriptor {
                name: "healthCheck".to_string(),
                params: vec![],
                return_type: TsType::Unknown,
                is_async: true,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// TypeScript extension adapter
// ---------------------------------------------------------------------------

/// Adapter that wraps a TypeScript extension and exposes it as a Rust
/// `ToolExtension`.  In production the execute closure calls into the N-API
/// binding; in tests we inject a plain closure.
pub struct TsExtensionAdapter {
    meta: ToolMeta,
    execute_fn:
        Box<dyn Fn(HashMap<String, Value>) -> Result<Value, ExtensionError> + Send + Sync>,
}

impl TsExtensionAdapter {
    pub fn new(
        meta: ToolMeta,
        execute_fn: impl Fn(HashMap<String, Value>) -> Result<Value, ExtensionError>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        TsExtensionAdapter {
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
// Validation helpers
// ---------------------------------------------------------------------------

/// Check that a `Value` returned from a TypeScript extension matches the
/// expected TypeScript return type annotation.
pub fn validate_ts_value(value: &Value, expected: &TsType) -> bool {
    match (value, expected) {
        (Value::Str(_), TsType::String) => true,
        (Value::Int(_) | Value::Float(_), TsType::Number) => true,
        (Value::Bool(_), TsType::Boolean) => true,
        (Value::Null, TsType::Null | TsType::Undefined) => true,
        (Value::Array(_), TsType::Array(_)) => true,
        (Value::Map(_), TsType::Record(_, _)) => true,
        (_, TsType::Any | TsType::Unknown) => true,
        _ => false,
    }
}
