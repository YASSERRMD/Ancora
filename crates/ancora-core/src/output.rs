use crate::error::AncoraError;

/// Generate a repair request to send to the model after an invalid output.
pub fn repair_prompt(output: &str, reason: &str) -> String {
    format!(
        "Your previous output was invalid. Reason: {reason}. \
         Previous output: {output}. \
         Please provide a corrected output."
    )
}

/// Run validation and, on failure, call `repair_fn` up to `max_attempts` times.
///
/// Returns the validated output string or `AncoraError::OutputValidation` when
/// the attempt budget is exhausted before a valid output is produced.
pub fn validate_with_repair<F>(
    output: String,
    schema_json: &str,
    max_attempts: u32,
    mut repair_fn: F,
) -> Result<String, AncoraError>
where
    F: FnMut(&str, &str) -> Result<String, AncoraError>,
{
    let mut current = output;
    let mut attempt = 0u32;

    loop {
        match validate_output(&current, schema_json) {
            Ok(()) => return Ok(current),
            Err(reason) => {
                attempt += 1;
                if attempt >= max_attempts {
                    return Err(AncoraError::OutputValidation { attempts: attempt, reason });
                }
                current = repair_fn(&current, &reason)?;
            }
        }
    }
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
