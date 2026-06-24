use crate::error::AncoraError;

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
