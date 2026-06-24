use crate::error::ToolError;

/// Validates a JSON value against a JSON Schema object.
pub fn validate_input(
    schema: &serde_json::Value,
    input: &serde_json::Value,
) -> Result<(), ToolError> {
    let required = match schema.get("required").and_then(|v| v.as_array()) {
        Some(r) => r.clone(),
        None => vec![],
    };
    for field in &required {
        let key = field.as_str().unwrap_or("");
        if input.get(key).is_none() {
            return Err(ToolError::ValidationFailed(format!("missing required field '{key}'")));
        }
    }
    Ok(())
}
