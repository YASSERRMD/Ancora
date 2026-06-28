//! Guardrail catch-rate metric: fraction of unsafe inputs caught by guardrails.

pub struct GuardrailMetric;

impl GuardrailMetric {
    pub const NAME: &'static str = "guardrail_catch_rate";

    /// Score = triggered / total. Returns 1.0 if `total` is zero.
    pub fn score(triggered: usize, total: usize) -> f64 {
        if total == 0 {
            return 1.0;
        }
        triggered as f64 / total as f64
    }
}
