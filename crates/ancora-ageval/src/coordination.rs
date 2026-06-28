//! Coordination success metric: fraction of assigned agents that completed their task.

pub struct CoordinationMetric;

impl CoordinationMetric {
    pub const NAME: &'static str = "coordination_success";

    /// Score = completed / assigned. Returns 1.0 if `assigned` is zero.
    pub fn score(assigned: usize, completed: usize) -> f64 {
        if assigned == 0 {
            return 1.0;
        }
        completed as f64 / assigned as f64
    }
}
