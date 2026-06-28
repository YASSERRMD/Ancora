//! Eval report generation: aggregated results with regression tracking.

use crate::metric::MetricScore;

pub struct EvalReport {
    pub name: String,
    pub tick: u64,
    pub scores: Vec<MetricScore>,
    pub regressions: Vec<String>,
}

impl EvalReport {
    pub fn new(name: impl Into<String>, tick: u64) -> Self {
        Self {
            name: name.into(),
            tick,
            scores: Vec::new(),
            regressions: Vec::new(),
        }
    }

    pub fn add_score(&mut self, score: MetricScore) {
        self.scores.push(score);
    }

    pub fn add_regression(&mut self, metric_name: impl Into<String>) {
        self.regressions.push(metric_name.into());
    }

    pub fn mean_score(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        self.scores.iter().map(|s| s.score).sum::<f64>() / self.scores.len() as f64
    }

    pub fn has_regressions(&self) -> bool {
        !self.regressions.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "EvalReport[{}] tick={} metrics={} mean={:.3} regressions={}",
            self.name,
            self.tick,
            self.scores.len(),
            self.mean_score(),
            self.regressions.len()
        )
    }
}
