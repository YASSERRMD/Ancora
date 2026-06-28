use serde_json::Value;
use crate::error::StructuredError;
use crate::extractor::JsonExtractor;
use crate::schema::OutputSchema;
use crate::validator::OutputValidator;

/// One-shot parse: extract JSON from text and validate against schema.
pub fn parse_response(text: &str, schema: &OutputSchema) -> Result<Value, StructuredError> {
    let value = JsonExtractor::extract(text)?;
    OutputValidator::validate(schema, &value)?;
    Ok(value)
}

/// Extract a single string field from a parsed response.
pub fn extract_string_field<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(|v| v.as_str())
}

/// Extract a numeric field as f64.
pub fn extract_number_field(value: &Value, field: &str) -> Option<f64> {
    value.get(field).and_then(|v| v.as_f64())
}
