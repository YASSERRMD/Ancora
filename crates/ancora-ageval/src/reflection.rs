//! Reflection improvement metric: quantifies whether self-critique improved output.

pub struct ReflectionMetric;

impl ReflectionMetric {
    pub const NAME: &'static str = "reflection_improvement";

    /// Returns 1.0 if `after` improved over `before` (longer and changed), 0.5 if
    /// changed but not longer, 0.0 if unchanged.
    pub fn score(before: &str, after: &str) -> f64 {
        if before == after {
            return 0.0;
        }
        if after.len() > before.len() {
            1.0
        } else {
            0.5
        }
    }
}
