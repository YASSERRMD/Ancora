//! Shared metric score type used by all behavior metrics.

#[derive(Debug, Clone)]
pub struct MetricScore {
    pub metric_name: String,
    pub score: f64,
}

impl MetricScore {
    pub fn new(metric_name: impl Into<String>, score: f64) -> Self {
        Self {
            metric_name: metric_name.into(),
            score,
        }
    }
}
