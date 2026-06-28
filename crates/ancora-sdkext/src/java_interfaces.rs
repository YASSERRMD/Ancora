/// Java extension interface definitions modelled in Rust.
///
/// Java extensions reach Ancora via a JNI bridge layer.  This module documents
/// and validates the JNI contract from the Rust side, and provides an adapter
/// that presents a Java extension as a Rust `ToolExtension`.

use std::collections::HashMap;

use crate::rs_traits::{ExtensionError, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Java type system mirror
// ---------------------------------------------------------------------------

/// Java types representable in extension argument / return schemas.
#[derive(Debug, Clone, PartialEq)]
pub enum JavaType {
    String,
    Int,
    Long,
    Double,
    Boolean,
    Object,
    Map(Box<JavaType>, Box<JavaType>),
    List(Box<JavaType>),
    Void,
}

impl JavaType {
    /// Return the Java type name.
    pub fn java_name(&self) -> String {
        match self {
            JavaType::String => "String".to_string(),
            JavaType::Int => "int".to_string(),
            JavaType::Long => "long".to_string(),
            JavaType::Double => "double".to_string(),
            JavaType::Boolean => "boolean".to_string(),
            JavaType::Object => "Object".to_string(),
            JavaType::Map(k, v) => {
                format!("Map<{}, {}>", k.java_name(), v.java_name())
            }
            JavaType::List(t) => format!("List<{}>", t.java_name()),
            JavaType::Void => "void".to_string(),
        }
    }

    /// Return the JNI type descriptor character.
    pub fn jni_descriptor(&self) -> String {
        match self {
            JavaType::String => "Ljava/lang/String;".to_string(),
            JavaType::Int => "I".to_string(),
            JavaType::Long => "J".to_string(),
            JavaType::Double => "D".to_string(),
            JavaType::Boolean => "Z".to_string(),
            JavaType::Object => "Ljava/lang/Object;".to_string(),
            JavaType::Map(_, _) => "Ljava/util/Map;".to_string(),
            JavaType::List(_) => "Ljava/util/List;".to_string(),
            JavaType::Void => "V".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Interface description
// ---------------------------------------------------------------------------

/// Describes the `ToolExtension` Java interface.
#[derive(Debug, Clone)]
pub struct JavaInterfaceDescriptor {
    pub interface_name: String,
    pub package: String,
    pub methods: Vec<JavaMethodDescriptor>,
}

#[derive(Debug, Clone)]
pub struct JavaMethodDescriptor {
    pub name: String,
    pub params: Vec<(String, JavaType)>,
    pub return_type: JavaType,
}

/// Return the canonical Java interface descriptor for Ancora tool extensions.
pub fn canonical_java_interface() -> JavaInterfaceDescriptor {
    JavaInterfaceDescriptor {
        interface_name: "ToolExtension".to_string(),
        package: "io.ancora.sdk".to_string(),
        methods: vec![
            JavaMethodDescriptor {
                name: "getMeta".to_string(),
                params: vec![],
                return_type: JavaType::Object,
            },
            JavaMethodDescriptor {
                name: "execute".to_string(),
                params: vec![(
                    "args".to_string(),
                    JavaType::Map(
                        Box::new(JavaType::String),
                        Box::new(JavaType::Object),
                    ),
                )],
                return_type: JavaType::Object,
            },
            JavaMethodDescriptor {
                name: "healthCheck".to_string(),
                params: vec![],
                return_type: JavaType::Void,
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Java extension adapter
// ---------------------------------------------------------------------------

/// Adapter that wraps a Java extension (via JNI) and presents it as a Rust
/// `ToolExtension`.
pub struct JavaExtensionAdapter {
    meta: ToolMeta,
    execute_fn:
        Box<dyn Fn(HashMap<String, Value>) -> Result<Value, ExtensionError> + Send + Sync>,
}

impl JavaExtensionAdapter {
    pub fn new(
        meta: ToolMeta,
        execute_fn: impl Fn(HashMap<String, Value>) -> Result<Value, ExtensionError>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        JavaExtensionAdapter {
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
// JNI descriptor helpers
// ---------------------------------------------------------------------------

/// Build the JNI method descriptor string for a `JavaMethodDescriptor`.
pub fn build_jni_descriptor(method: &JavaMethodDescriptor) -> String {
    let params: String = method
        .params
        .iter()
        .map(|(_, t)| t.jni_descriptor())
        .collect();
    format!("({}){}", params, method.return_type.jni_descriptor())
}

/// Map a Rust `Value` to the closest `JavaType`.
pub fn rust_value_to_java_type(value: &Value) -> JavaType {
    match value {
        Value::Str(_) => JavaType::String,
        Value::Int(_) => JavaType::Long,
        Value::Float(_) => JavaType::Double,
        Value::Bool(_) => JavaType::Boolean,
        Value::Array(_) => JavaType::List(Box::new(JavaType::Object)),
        Value::Map(_) => {
            JavaType::Map(Box::new(JavaType::String), Box::new(JavaType::Object))
        }
        Value::Null => JavaType::Object,
    }
}
