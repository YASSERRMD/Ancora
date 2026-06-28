//! Memory retention metric: fraction of important items retained after consolidation.

pub struct MemoryMetric;

impl MemoryMetric {
    pub const NAME: &'static str = "memory_retention";

    /// Score = retained / total. Returns 1.0 if `total` is zero.
    pub fn score(retained: usize, total: usize) -> f64 {
        if total == 0 {
            return 1.0;
        }
        retained as f64 / total as f64
    }
}
