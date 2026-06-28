/// Evaluation data redaction policy.
///
/// Eval runs may capture prompts, completions, and scores. This module
/// ensures that raw text payloads are never exported to telemetry sinks
/// without explicit opt-in.

use crate::pii_scrub::scrub_pii;

/// An evaluation sample as it comes off the eval runner.
#[derive(Debug, Clone)]
pub struct EvalSample {
    pub id: String,
    pub prompt: Option<String>,
    pub completion: Option<String>,
    pub score: f64,
    pub metadata: Vec<(String, String)>,
}

/// Configuration for the eval redaction policy.
#[derive(Debug, Clone)]
pub struct EvalPolicy {
    /// If false, prompt and completion are always dropped.
    pub allow_prompt_export: bool,
    /// If false, completion text is always dropped.
    pub allow_completion_export: bool,
    /// Whether to scrub PII from metadata values.
    pub scrub_metadata: bool,
}

impl Default for EvalPolicy {
    fn default() -> Self {
        EvalPolicy {
            allow_prompt_export: false,
            allow_completion_export: false,
            scrub_metadata: true,
        }
    }
}

/// A redacted representation of an eval sample safe for telemetry export.
#[derive(Debug, Clone)]
pub struct RedactedEvalSample {
    pub id: String,
    pub prompt: Option<String>,
    pub completion: Option<String>,
    pub score: f64,
    pub metadata: Vec<(String, String)>,
}

impl EvalPolicy {
    /// Apply the policy to a raw eval sample.
    pub fn apply(&self, sample: EvalSample) -> RedactedEvalSample {
        let prompt = if self.allow_prompt_export {
            sample.prompt.map(|p| scrub_pii(&p))
        } else {
            None
        };

        let completion = if self.allow_completion_export {
            sample.completion.map(|c| scrub_pii(&c))
        } else {
            None
        };

        let metadata = if self.scrub_metadata {
            sample
                .metadata
                .into_iter()
                .map(|(k, v)| (k, scrub_pii(&v)))
                .collect()
        } else {
            sample.metadata
        };

        RedactedEvalSample {
            id: sample.id,
            prompt,
            completion,
            score: sample.score,
            metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_dropped_by_default() {
        let policy = EvalPolicy::default();
        let sample = EvalSample {
            id: "e1".to_string(),
            prompt: Some("Tell me about Alice".to_string()),
            completion: Some("Alice is...".to_string()),
            score: 0.9,
            metadata: vec![],
        };
        let redacted = policy.apply(sample);
        assert!(redacted.prompt.is_none());
        assert!(redacted.completion.is_none());
    }

    #[test]
    fn score_always_exported() {
        let policy = EvalPolicy::default();
        let sample = EvalSample {
            id: "e2".to_string(),
            prompt: None,
            completion: None,
            score: 0.75,
            metadata: vec![],
        };
        let redacted = policy.apply(sample);
        assert!((redacted.score - 0.75).abs() < f64::EPSILON);
    }
}
