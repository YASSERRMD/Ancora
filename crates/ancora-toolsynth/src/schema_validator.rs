use serde_json::Value;
use crate::error::SynthError;

/// Validates that a generated tool schema is well-formed JSON Schema.
pub struct SchemaValidator;

impl SchemaValidator {
    pub fn validate(schema: &Value) -> Result<(), SynthError> {
        match schema {
            Value::Object(map) => {
                if let Some(t) = map.get("type") {
                    match t {
                        Value::String(s) if ["object", "array", "string", "number", "boolean", "null"].contains(&s.as_str()) => Ok(()),
                        _ => Err(SynthError::InvalidSchema("unknown type".into())),
                    }
                } else {
                    Err(SynthError::InvalidSchema("missing type field".into()))
                }
            }
            _ => Err(SynthError::InvalidSchema("schema must be a JSON object".into())),
        }
    }
}
