//! Step decomposition for small models.
//!
//! Large tasks that a capable model can handle in one shot often overwhelm
//! small models.  This module provides a task decomposition layer that:
//! 1. Breaks a complex task into a sequence of simpler sub-tasks.
//! 2. Executes each sub-task with the SLM individually.
//! 3. Aggregates the results into a final answer.
//!
//! The decomposition plan is expressed as a list of [`Step`]s.  Each step has
//! an explicit expected-output description that the verifier (see
//! `verifier.rs`) checks before proceeding to the next step.

use serde::{Deserialize, Serialize};

/// A single step in a decomposed task plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// Short identifier for the step (e.g. `"extract_entities"`).
    pub id: String,
    /// Human-readable description that goes into the SLM prompt.
    pub description: String,
    /// How the step's output should be formatted.
    pub output_format: OutputFormat,
    /// Whether this step is optional (if it fails, execution continues).
    pub optional: bool,
}

/// Expected output format for a decomposed step.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    /// Free-form text.
    Text,
    /// A JSON object (must parse as `serde_json::Value::Object`).
    JsonObject,
    /// A JSON array.
    JsonArray,
    /// A single yes/no answer.
    YesNo,
}

/// The result of executing a single decomposed step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Which step this result belongs to.
    pub step_id: String,
    /// Raw model output.
    pub raw_output: String,
    /// Whether the output passed format validation.
    pub valid: bool,
    /// Optional error message when `valid == false`.
    pub error: Option<String>,
}

/// A complete decomposition plan for a task.
#[derive(Debug, Clone)]
pub struct DecompositionPlan {
    /// Task description fed as system context.
    pub task_description: String,
    /// Ordered list of steps to execute.
    pub steps: Vec<Step>,
}

impl DecompositionPlan {
    /// Create a new plan.
    pub fn new(task_description: impl Into<String>, steps: Vec<Step>) -> Self {
        Self {
            task_description: task_description.into(),
            steps,
        }
    }

    /// Build the prompt for a specific step, incorporating prior step results
    /// as context.
    pub fn build_step_prompt(&self, step: &Step, prior: &[StepResult]) -> String {
        let mut prompt = format!(
            "Task: {}\n\nCurrent step ({}): {}\n",
            self.task_description, step.id, step.description
        );

        if !prior.is_empty() {
            prompt.push_str("\nPrevious results:\n");
            for r in prior {
                prompt.push_str(&format!("[{}]: {}\n", r.step_id, r.raw_output.trim()));
            }
        }

        let fmt_hint = match step.output_format {
            OutputFormat::Text => "Respond with plain text.",
            OutputFormat::JsonObject => "Respond with a JSON object only.",
            OutputFormat::JsonArray => "Respond with a JSON array only.",
            OutputFormat::YesNo => "Respond with exactly 'yes' or 'no'.",
        };
        prompt.push_str(&format!("\n{}", fmt_hint));
        prompt
    }
}

/// Validate a step's raw output against the expected [`OutputFormat`].
pub fn validate_step_output(raw: &str, format: &OutputFormat) -> Result<(), String> {
    let trimmed = raw.trim();
    match format {
        OutputFormat::Text => {
            if trimmed.is_empty() {
                Err("empty output".into())
            } else {
                Ok(())
            }
        }
        OutputFormat::JsonObject => {
            let v: serde_json::Value =
                serde_json::from_str(trimmed).map_err(|e| format!("not valid JSON: {}", e))?;
            if v.is_object() {
                Ok(())
            } else {
                Err(format!("expected JSON object, got {:?}", v))
            }
        }
        OutputFormat::JsonArray => {
            let v: serde_json::Value =
                serde_json::from_str(trimmed).map_err(|e| format!("not valid JSON: {}", e))?;
            if v.is_array() {
                Ok(())
            } else {
                Err(format!("expected JSON array, got {:?}", v))
            }
        }
        OutputFormat::YesNo => {
            let lower = trimmed.to_lowercase();
            if lower == "yes" || lower == "no" {
                Ok(())
            } else {
                Err(format!("expected 'yes' or 'no', got {:?}", trimmed))
            }
        }
    }
}

/// Execute a decomposition plan using a caller-provided model function.
///
/// Stops at the first failing non-optional step and returns all results so
/// far.  If all steps complete (or only optional steps fail), returns all
/// [`StepResult`]s.
pub fn execute_plan<F>(plan: &DecompositionPlan, model_fn: F) -> Vec<StepResult>
where
    F: Fn(&str) -> String,
{
    let mut results: Vec<StepResult> = Vec::new();

    for step in &plan.steps {
        let prompt = plan.build_step_prompt(step, &results);
        let raw = model_fn(&prompt);
        let valid_res = validate_step_output(&raw, &step.output_format);
        let valid = valid_res.is_ok();
        let error = valid_res.err();

        let result = StepResult {
            step_id: step.id.clone(),
            raw_output: raw,
            valid,
            error,
        };

        let should_stop = !valid && !step.optional;
        results.push(result);
        if should_stop {
            break;
        }
    }

    results
}
