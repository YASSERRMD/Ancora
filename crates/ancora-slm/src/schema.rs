//! Schema-guided generation for small models.
//!
//! Provides lightweight JSON schema validation and prompt augmentation that
//! guide the model to produce output matching a target schema.  No external
//! schema libraries are required — we implement a minimal validator covering
//! the subset of JSON Schema used in practice.

use serde_json::Value;
use std::collections::HashSet;

/// A minimal JSON schema descriptor.
#[derive(Debug, Clone)]
pub enum Schema {
    /// Accept any JSON value.
    Any,
    /// Must be a JSON object with the given required keys.
    Object {
        /// Required property names.
        required: Vec<String>,
        /// Optional property schemas, keyed by property name.
        properties: Vec<(String, Schema)>,
    },
    /// Must be a JSON array whose items match `item_schema`.
    Array { item_schema: Box<Schema> },
    /// Must be a JSON string.
    String,
    /// Must be a JSON number.
    Number,
    /// Must be a JSON boolean.
    Boolean,
    /// Must be one of the listed string values.
    Enum(Vec<String>),
}

/// A schema validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// JSON pointer path to the offending value.
    pub path: String,
    /// Human-readable description of the violation.
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}: {}", self.path, self.message)
    }
}

/// Validate a [`Value`] against a [`Schema`], collecting all violations.
pub fn validate(value: &Value, schema: &Schema) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    validate_at(value, schema, "$", &mut errors);
    errors
}

fn validate_at(value: &Value, schema: &Schema, path: &str, errors: &mut Vec<ValidationError>) {
    match schema {
        Schema::Any => {}
        Schema::String => {
            if !value.is_string() {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("expected string, got {:?}", value),
                });
            }
        }
        Schema::Number => {
            if !value.is_number() {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("expected number, got {:?}", value),
                });
            }
        }
        Schema::Boolean => {
            if !value.is_boolean() {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("expected boolean, got {:?}", value),
                });
            }
        }
        Schema::Enum(variants) => {
            let s = value.as_str().unwrap_or("");
            if !variants.contains(&s.to_string()) {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("expected one of {:?}, got {:?}", variants, value),
                });
            }
        }
        Schema::Array { item_schema } => match value.as_array() {
            None => errors.push(ValidationError {
                path: path.to_string(),
                message: format!("expected array, got {:?}", value),
            }),
            Some(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    validate_at(item, item_schema, &format!("{}/{}", path, i), errors);
                }
            }
        },
        Schema::Object {
            required,
            properties,
        } => {
            match value.as_object() {
                None => errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("expected object, got {:?}", value),
                }),
                Some(obj) => {
                    // Check required keys.
                    for key in required {
                        if !obj.contains_key(key) {
                            errors.push(ValidationError {
                                path: path.to_string(),
                                message: format!("missing required property '{}'", key),
                            });
                        }
                    }
                    // Validate property schemas.
                    let prop_keys: HashSet<&str> =
                        properties.iter().map(|(k, _)| k.as_str()).collect();
                    for (k, sub) in properties {
                        if let Some(v) = obj.get(k) {
                            validate_at(v, sub, &format!("{}/{}", path, k), errors);
                        }
                        // Missing optional properties are fine.
                    }
                    // Warn about extra keys not in properties.
                    for key in obj.keys() {
                        if !prop_keys.contains(key.as_str()) {
                            // Extra keys are allowed by default (open content model).
                        }
                    }
                }
            }
        }
    }
}

/// Generate a compact schema description string suitable for embedding in an
/// SLM prompt.  Small models respond better to concrete examples than abstract
/// JSON Schema syntax.
pub fn schema_to_prompt_hint(schema: &Schema) -> String {
    match schema {
        Schema::Any => "any JSON value".to_string(),
        Schema::String => "a JSON string".to_string(),
        Schema::Number => "a JSON number".to_string(),
        Schema::Boolean => "true or false".to_string(),
        Schema::Enum(vs) => format!(
            "one of: {}",
            vs.iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Schema::Array { item_schema } => {
            format!(
                "a JSON array where each item is {}",
                schema_to_prompt_hint(item_schema)
            )
        }
        Schema::Object {
            required,
            properties,
        } => {
            let fields: Vec<String> = required
                .iter()
                .map(|k| {
                    let type_hint = properties
                        .iter()
                        .find(|(pk, _)| pk == k)
                        .map(|(_, s)| schema_to_prompt_hint(s))
                        .unwrap_or_else(|| "any".to_string());
                    format!("\"{}\" ({})", k, type_hint)
                })
                .collect();
            format!("a JSON object with fields: {}", fields.join(", "))
        }
    }
}

/// Augment a prompt with the schema hint so the model knows the expected shape.
pub fn augment_prompt_with_schema(prompt: &str, schema: &Schema) -> String {
    format!(
        "{}\n\nOutput format: {}.",
        prompt,
        schema_to_prompt_hint(schema)
    )
}
