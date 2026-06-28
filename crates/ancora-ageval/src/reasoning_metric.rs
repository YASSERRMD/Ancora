//! Reasoning correctness metric: fraction of reasoning steps verified.

pub struct ReasoningMetric;

impl ReasoningMetric {
    pub const NAME: &'static str = "reasoning_correctness";

    /// Score = verified / total. Returns 1.0 if `total` is zero.
    pub fn score(verified: usize, total: usize) -> f64 {
        if total == 0 {
            return 1.0;
        }
        verified as f64 / total as f64
    }
}
