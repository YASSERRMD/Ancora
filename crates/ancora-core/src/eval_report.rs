use crate::eval_runner::CaseResult;

/// A snapshot of pass rates for a set of eval cases.
#[derive(Debug, Clone)]
pub struct EvalBaseline {
    pub case_pass_rates: Vec<(String, f64)>,
}

impl EvalBaseline {
    pub fn from_results(results: &[CaseResult]) -> Self {
        Self {
            case_pass_rates: results
                .iter()
                .map(|r| (r.case_id.clone(), r.pass_rate()))
                .collect(),
        }
    }
}

/// Comparison of a new eval run against a baseline.
#[derive(Debug, Clone)]
pub struct RegressionEntry {
    pub case_id: String,
    pub baseline_rate: f64,
    pub current_rate: f64,
    pub delta: f64,
    pub regressed: bool,
}

/// Report comparing current results to a baseline.
#[derive(Debug, Clone)]
pub struct RegressionReport {
    pub entries: Vec<RegressionEntry>,
    pub total_regressions: usize,
}

impl RegressionReport {
    pub fn build(baseline: &EvalBaseline, current: &[CaseResult]) -> Self {
        let baseline_map: std::collections::HashMap<&str, f64> = baseline
            .case_pass_rates
            .iter()
            .map(|(id, rate)| (id.as_str(), *rate))
            .collect();
        let mut entries = Vec::new();
        for r in current {
            let baseline_rate = baseline_map.get(r.case_id.as_str()).copied().unwrap_or(0.0);
            let current_rate = r.pass_rate();
            let delta = current_rate - baseline_rate;
            entries.push(RegressionEntry {
                case_id: r.case_id.clone(),
                baseline_rate,
                current_rate,
                delta,
                regressed: delta < -0.05,
            });
        }
        let total_regressions = entries.iter().filter(|e| e.regressed).count();
        RegressionReport { entries, total_regressions }
    }

    pub fn is_clean(&self) -> bool {
        self.total_regressions == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval_runner::CaseResult;

    fn make_result(id: &str, n: usize, pass_count: usize) -> CaseResult {
        CaseResult { case_id: id.into(), rollouts: vec![], pass_count, n }
    }

    #[test]
    fn regression_report_is_clean_when_no_regressions() {
        let baseline = EvalBaseline { case_pass_rates: vec![("c1".into(), 0.8)] };
        let current = vec![make_result("c1", 5, 5)];
        let report = RegressionReport::build(&baseline, &current);
        assert!(report.is_clean());
    }

    #[test]
    fn regression_report_detects_regression() {
        let baseline = EvalBaseline { case_pass_rates: vec![("c1".into(), 1.0)] };
        let current = vec![make_result("c1", 5, 0)];
        let report = RegressionReport::build(&baseline, &current);
        assert!(!report.is_clean());
        assert_eq!(report.total_regressions, 1);
    }

    #[test]
    fn regression_report_delta_is_correct() {
        let baseline = EvalBaseline { case_pass_rates: vec![("c1".into(), 0.5)] };
        let current = vec![make_result("c1", 4, 4)];
        let report = RegressionReport::build(&baseline, &current);
        assert!((report.entries[0].delta - 0.5).abs() < 1e-9);
    }

    #[test]
    fn baseline_from_results_captures_pass_rates() {
        let results = vec![make_result("c1", 4, 2), make_result("c2", 4, 4)];
        let baseline = EvalBaseline::from_results(&results);
        assert_eq!(baseline.case_pass_rates.len(), 2);
    }
}
