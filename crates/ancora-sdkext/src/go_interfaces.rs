/// Go extension interface definitions modelled in Rust.
///
/// This module contains the Rust-side representation of the interface contract
/// that Go extensions must satisfy when linked via the FFI bridge. The actual
/// Go code lives in the `sdk/go/` tree; this module documents and validates
/// the binary protocol from the Rust side.

use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Protocol envelope
// ---------------------------------------------------------------------------

/// Wire format used when invoking a Go extension over the FFI boundary.
#[derive(Debug, Clone)]
pub struct GoCallEnvelope {
    /// The fully-qualified extension name, e.g. `"com.example.echo"`.
    pub extension_name: String,
    /// Method to invoke on the extension object.
    pub method: GoMethod,
    /// Serialised arguments (JSON-encoded on the Go side).
    pub payload: String,
}

/// The set of methods that every Go extension must export.
#[derive(Debug, Clone, PartialEq)]
pub enum GoMethod {
    Meta,
    Execute,
    HealthCheck,
}

impl std::fmt::Display for GoMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoMethod::Meta => write!(f, "meta"),
            GoMethod::Execute => write!(f, "execute"),
            GoMethod::HealthCheck => write!(f, "health_check"),
        }
    }
}

// ---------------------------------------------------------------------------
// Go extension adapter
// ---------------------------------------------------------------------------

/// Adapter that wraps a Go extension exposed over the FFI boundary and presents
/// it as a Rust `ToolExtension`.
pub struct GoExtensionAdapter {
    meta: ToolMeta,
    /// Simulated dispatch table; in production this would hold function
    /// pointers resolved from a shared library.
    dispatch: HashMap<String, fn(String) -> Result<String, String>>,
}

impl GoExtensionAdapter {
    pub fn new(meta: ToolMeta) -> Self {
        GoExtensionAdapter {
            meta,
            dispatch: HashMap::new(),
        }
    }

    /// Register a handler for a given method name.  Used in tests to inject
    /// mock implementations without a real shared library.
    pub fn register_handler(
        &mut self,
        method: &str,
        handler: fn(String) -> Result<String, String>,
    ) {
        self.dispatch.insert(method.to_string(), handler);
    }

    /// Dispatch a call to the registered handler or return an error.
    pub fn dispatch(
        &self,
        envelope: &GoCallEnvelope,
    ) -> Result<String, ExtensionError> {
        let key = envelope.method.to_string();
        if let Some(handler) = self.dispatch.get(&key) {
            handler(envelope.payload.clone())
                .map_err(|e| ExtensionError::ExecutionFailed(e))
        } else {
            Err(ExtensionError::NotSupported(format!(
                "no handler registered for method '{key}'"
            )))
        }
    }

    pub fn meta(&self) -> &ToolMeta {
        &self.meta
    }
}

// ---------------------------------------------------------------------------
// Interface spec (documentation artefact)
// ---------------------------------------------------------------------------

/// Human-readable description of the Go interface contract.  Returned by the
/// `describe_go_interface` helper for documentation tooling.
pub struct GoInterfaceSpec {
    pub interface_name: String,
    pub methods: Vec<GoMethodSpec>,
}

pub struct GoMethodSpec {
    pub name: String,
    pub signature: String,
    pub description: String,
}

/// Return the canonical Go interface spec that every Ancora Go extension must
/// implement.
pub fn canonical_go_interface() -> GoInterfaceSpec {
    GoInterfaceSpec {
        interface_name: "ToolExtension".to_string(),
        methods: vec![
            GoMethodSpec {
                name: "Meta".to_string(),
                signature: "Meta() ToolMeta".to_string(),
                description: "Return static metadata (name, description, version).".to_string(),
            },
            GoMethodSpec {
                name: "Execute".to_string(),
                signature: "Execute(args map[string]any) (any, error)".to_string(),
                description: "Execute the tool with the provided arguments.".to_string(),
            },
            GoMethodSpec {
                name: "HealthCheck".to_string(),
                signature: "HealthCheck() error".to_string(),
                description: "Optional; return nil if healthy.".to_string(),
            },
        ],
    }
}

/// Validate that a `Value` returned from a Go extension is well-formed.
pub fn validate_go_return(value: &Value) -> Result<(), ExtensionError> {
    match value {
        Value::Null => Ok(()),
        Value::Map(m) => {
            if m.is_empty() {
                Err(ExtensionError::ExecutionFailed(
                    "Go extension returned empty map".to_string(),
                ))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}
