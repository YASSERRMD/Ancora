use crate::error::StructuredError;
use crate::schema::{FieldSchema, JsonType, OutputSchema};
use serde_json::Value;

pub struct OutputValidator;

impl OutputValidator {
    pub fn validate(schema: &OutputSchema, value: &Value) -> Result<(), StructuredError> {
        let obj = value
            .as_object()
            .ok_or_else(|| StructuredError::NotAnObject)?;

        for field in &schema.fields {
            if field.required && !obj.contains_key(&field.name) {
                return Err(StructuredError::MissingField {
                    field: field.name.clone(),
                });
            }
            if let Some(v) = obj.get(&field.name) {
                Self::check_type(field, v)?;
            }
        }
        Ok(())
    }

    fn check_type(field: &FieldSchema, value: &Value) -> Result<(), StructuredError> {
        let ok = match field.json_type {
            JsonType::String => value.is_string(),
            JsonType::Number => value.is_number(),
            JsonType::Boolean => value.is_boolean(),
            JsonType::Array => value.is_array(),
            JsonType::Object => value.is_object(),
            JsonType::Null => value.is_null(),
        };
        if !ok {
            return Err(StructuredError::TypeMismatch {
                field: field.name.clone(),
                expected: format!("{:?}", field.json_type),
                got: value_type_name(value).to_string(),
            });
        }
        Ok(())
    }
}

fn value_type_name(v: &Value) -> &'static str {
    match v {
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "boolean",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Null => "null",
    }
}
