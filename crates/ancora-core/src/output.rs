use crate::error::AncoraError;

/// Generate a repair request to send to the model after an invalid output.
pub fn repair_prompt(output: &str, reason: &str) -> String {
    format!(
        "Your previous output was invalid. Reason: {reason}. \
         Previous output: {output}. \
         Please provide a corrected output."
    )
}

/// Validate `output` against an optional JSON Schema string.
///
/// When `schema_json` is empty the output is accepted unconditionally.
/// When non-empty the output must be parseable as a JSON value.
pub fn validate_output(output: &str, schema_json: &str) -> Result<(), String> {
    if schema_json.is_empty() {
        return Ok(());
    }
    serde_json::from_str::<serde_json::Value>(output)
        .map(|_| ())
        .map_err(|e| format!("output is not valid JSON: {e}"))
}
