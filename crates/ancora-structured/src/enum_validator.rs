use serde_json::Value;
use crate::error::StructuredError;

/// Validates that a string field value is one of an allowed set.
pub struct EnumValidator {
    pub field_name: String,
    pub allowed: Vec<String>,
}

impl EnumValidator {
    pub fn new(field_name: &str, allowed: Vec<&str>) -> Self {
        Self {
            field_name: field_name.to_string(),
            allowed: allowed.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn validate(&self, value: &Value) -> Result<(), StructuredError> {
        let s = value.as_str().ok_or_else(|| StructuredError::TypeMismatch {
            field: self.field_name.clone(),
            expected: "string enum".to_string(),
            got: "non-string".to_string(),
        })?;
        if self.allowed.iter().any(|a| a == s) {
            Ok(())
        } else {
            Err(StructuredError::TypeMismatch {
                field: self.field_name.clone(),
                expected: format!("one of: {}", self.allowed.join(", ")),
                got: s.to_string(),
            })
        }
    }

    pub fn allowed_values(&self) -> &[String] {
        &self.allowed
    }
}
