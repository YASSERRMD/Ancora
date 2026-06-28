//! Planning quality metric: fraction of expected plan steps present in actual output.

pub struct PlanningMetric;

impl PlanningMetric {
    pub const NAME: &'static str = "planning_quality";

    /// Score = matched / expected. Returns 1.0 if expected is empty.
    pub fn score(expected_steps: &[String], actual_steps: &[String]) -> f64 {
        if expected_steps.is_empty() {
            return 1.0;
        }
        let matched = actual_steps
            .iter()
            .filter(|s| expected_steps.contains(s))
            .count();
        matched as f64 / expected_steps.len() as f64
    }
}
