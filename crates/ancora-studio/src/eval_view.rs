//! Eval results view - shows per-run evaluation scores.

#[derive(Debug, Clone, PartialEq)]
pub enum EvalOutcome {
    Pass,
    Fail,
    Skip,
    Partial { score: f64 },
}

impl EvalOutcome {
    pub fn score(&self) -> f64 {
        match self {
            EvalOutcome::Pass => 1.0,
            EvalOutcome::Fail => 0.0,
            EvalOutcome::Skip => f64::NAN,
            EvalOutcome::Partial { score } => *score,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            EvalOutcome::Pass => "pass",
            EvalOutcome::Fail => "fail",
            EvalOutcome::Skip => "skip",
            EvalOutcome::Partial { .. } => "partial",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalResult {
    pub eval_name: String,
    pub run_id: String,
    pub outcome: EvalOutcome,
    pub reason: Option<String>,
    pub latency_ms: u64,
}

pub struct EvalView {
    results: Vec<EvalResult>,
}

impl EvalView {
    pub fn new(results: Vec<EvalResult>) -> Self {
        Self { results }
    }

    pub fn results(&self) -> &[EvalResult] {
        &self.results
    }

    pub fn pass_rate(&self) -> f64 {
        let scored: Vec<&EvalResult> = self
            .results
            .iter()
            .filter(|r| !matches!(r.outcome, EvalOutcome::Skip))
            .collect();
        if scored.is_empty() {
            return 0.0;
        }
        let passed = scored
            .iter()
            .filter(|r| matches!(r.outcome, EvalOutcome::Pass))
            .count();
        passed as f64 / scored.len() as f64
    }

    pub fn average_score(&self) -> Option<f64> {
        let scores: Vec<f64> = self
            .results
            .iter()
            .map(|r| r.outcome.score())
            .filter(|s| !s.is_nan())
            .collect();
        if scores.is_empty() {
            return None;
        }
        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }

    pub fn results_for_run(&self, run_id: &str) -> Vec<&EvalResult> {
        self.results.iter().filter(|r| r.run_id == run_id).collect()
    }

    pub fn failed_evals(&self) -> Vec<&EvalResult> {
        self.results
            .iter()
            .filter(|r| matches!(r.outcome, EvalOutcome::Fail))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_view() -> EvalView {
        EvalView::new(vec![
            EvalResult {
                eval_name: "accuracy".into(),
                run_id: "r1".into(),
                outcome: EvalOutcome::Pass,
                reason: None,
                latency_ms: 10,
            },
            EvalResult {
                eval_name: "safety".into(),
                run_id: "r1".into(),
                outcome: EvalOutcome::Fail,
                reason: Some("refused".into()),
                latency_ms: 5,
            },
        ])
    }

    #[test]
    fn test_pass_rate() {
        let view = sample_view();
        assert!((view.pass_rate() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_failed_evals() {
        let view = sample_view();
        let failed = view.failed_evals();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].eval_name, "safety");
    }
}
