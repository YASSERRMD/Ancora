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
                    return Err(AncoraError::OutputValidation {
                        attempts: attempt,
                        reason,
                    });
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

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA: &str = r#"{"type":"object"}"#;

    #[test]
    fn valid_output_passes_invalid_triggers_repair() {
        let mut repair_calls = 0u32;

        let result = validate_with_repair("not json".to_string(), SCHEMA, 3, |_output, _reason| {
            repair_calls += 1;
            Ok(r#"{"fixed": true}"#.to_string())
        })
        .unwrap();

        assert_eq!(result, r#"{"fixed": true}"#);
        assert_eq!(repair_calls, 1, "repair must be called exactly once");
    }

    #[test]
    fn repair_attempts_are_bounded() {
        let mut repair_calls = 0u32;
        let max_attempts = 3u32;

        let err = validate_with_repair(
            "not json".to_string(),
            SCHEMA,
            max_attempts,
            |_output, _reason| {
                repair_calls += 1;
                Ok("still not json".to_string())
            },
        )
        .unwrap_err();

        assert!(
            matches!(err, AncoraError::OutputValidation { attempts, .. } if attempts == max_attempts),
            "expected OutputValidation with attempts = {max_attempts}, got {err:?}",
        );
        assert_eq!(
            repair_calls,
            max_attempts - 1,
            "repair is called max_attempts - 1 times before the budget is exhausted"
        );
    }
}
