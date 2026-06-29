//! Constrained decoding integration for small models.
//!
//! When a model does not natively support JSON mode, we simulate constrained
//! decoding by:
//! 1. Wrapping the raw model output in extraction logic.
//! 2. Validating the result against a minimal schema.
//! 3. Re-prompting with stronger constraints on failure (up to `max_retries`).
//!
//! In production, this module would integrate with llama.cpp's grammar
//! constraints or Outlines. Here we provide the orchestration skeleton and
//! pure-Rust validation so tests run offline without any model process.

use serde_json::Value;

/// Configuration for constrained decoding.
#[derive(Debug, Clone)]
pub struct ConstrainedConfig {
    /// Maximum number of retry attempts when output fails validation.
    pub max_retries: usize,
    /// If true, wrap the prompt with an explicit JSON fence instruction.
    pub add_json_fence_instruction: bool,
}

impl Default for ConstrainedConfig {
    fn default() -> Self {
        Self { max_retries: 3, add_json_fence_instruction: true }
    }
}

/// Errors from the constrained-decoding pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstrainedError {
    /// Exceeded retry budget without producing valid output.
    RetryBudgetExceeded { attempts: usize },
    /// The output does not parse as JSON.
    NotJson(String),
    /// The JSON structure violates the required shape.
    SchemaViolation(String),
}

impl std::fmt::Display for ConstrainedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RetryBudgetExceeded { attempts } => {
                write!(f, "retry budget exceeded after {} attempts", attempts)
            }
            Self::NotJson(e) => write!(f, "output is not valid JSON: {}", e),
            Self::SchemaViolation(e) => write!(f, "JSON schema violation: {}", e),
        }
    }
}

/// Wrap a user prompt with a JSON-fence instruction that small models respond
/// to better than just "output JSON".
pub fn add_json_fence_instruction(prompt: &str) -> String {
    format!(
        "{}\n\nYour response MUST be a single JSON object enclosed in triple backticks:\n```json\n{{...}}\n```\nDo not include any explanation outside the code block.",
        prompt
    )
}

/// Extract JSON from model output that may be wrapped in a markdown code block.
///
/// Accepts:
/// - Bare JSON objects / arrays.
/// - ` ```json ... ``` ` fenced blocks.
/// - ` ``` ... ``` ` fenced blocks (no language tag).
pub fn extract_json(raw: &str) -> Option<String> {
    let trimmed = raw.trim();

    // Try fenced block extraction first.
    for fence in &["```json", "```"] {
        if let Some(start) = trimmed.find(fence) {
            let after_open = start + fence.len();
            // Find the closing ```.
            if let Some(rel_end) = trimmed[after_open..].find("```") {
                let content = trimmed[after_open..after_open + rel_end].trim();
                if !content.is_empty() {
                    return Some(content.to_string());
                }
            }
        }
    }

    // Fall back to finding a bare JSON object or array.
    let obj_start = trimmed.find('{');
    let arr_start = trimmed.find('[');

    match (obj_start, arr_start) {
        (Some(o), Some(a)) => {
            let start = o.min(a);
            Some(trimmed[start..].to_string())
        }
        (Some(o), None) => Some(trimmed[o..].to_string()),
        (None, Some(a)) => Some(trimmed[a..].to_string()),
        (None, None) => None,
    }
}

/// Attempt to parse and validate the JSON extracted from model output.
///
/// Returns the parsed [`Value`] if valid, or a [`ConstrainedError`] if not.
pub fn validate_json_output(raw: &str) -> Result<Value, ConstrainedError> {
    let extracted =
        extract_json(raw).ok_or_else(|| ConstrainedError::NotJson("no JSON found".into()))?;

    serde_json::from_str::<Value>(&extracted)
        .map_err(|e| ConstrainedError::NotJson(e.to_string()))
}

/// A deterministic mock "model" function type used for testing / replay.
/// In production this would invoke an actual inference endpoint.
pub type ModelFn = Box<dyn Fn(&str) -> String>;

/// Run the constrained-decoding pipeline with a caller-provided model function.
///
/// The pipeline:
/// 1. Optionally prepend a JSON-fence instruction.
/// 2. Call `model_fn(prompt)` to get raw output.
/// 3. Validate; if valid return the parsed JSON.
/// 4. On failure, retry with a stronger re-prompt, up to `config.max_retries`.
pub fn run_constrained<F>(
    prompt: &str,
    config: &ConstrainedConfig,
    model_fn: F,
) -> Result<Value, ConstrainedError>
where
    F: Fn(&str) -> String,
{
    let base_prompt = if config.add_json_fence_instruction {
        add_json_fence_instruction(prompt)
    } else {
        prompt.to_string()
    };

    for attempt in 0..=config.max_retries {
        let current_prompt = if attempt == 0 {
            base_prompt.clone()
        } else {
            format!(
                "{}\n\n[Attempt {}] Your previous response was not valid JSON. Please respond with ONLY a JSON object, no prose.",
                base_prompt, attempt + 1
            )
        };

        let raw = model_fn(&current_prompt);
        match validate_json_output(&raw) {
            Ok(v) => return Ok(v),
            Err(_) if attempt < config.max_retries => continue,
            Err(e) => return Err(e),
        }
    }

    Err(ConstrainedError::RetryBudgetExceeded { attempts: config.max_retries + 1 })
}
