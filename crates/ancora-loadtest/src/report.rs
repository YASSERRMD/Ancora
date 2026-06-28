use serde::Serialize;
use crate::soak::SoakSummary;

#[derive(Debug, Serialize)]
pub struct LoadTestReport {
    pub scenarios: Vec<ScenarioReport>,
    pub all_passed: bool,
}

#[derive(Debug, Serialize)]
pub struct ScenarioReport {
    pub name: String,
    pub passed: bool,
    pub total_requests: u64,
    pub total_errors: u64,
    pub error_rate_pct: f64,
    pub peak_rps: f64,
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
}

impl ScenarioReport {
    pub fn from_summary(name: &str, passed: bool, s: &SoakSummary) -> Self {
        Self {
            name: name.to_string(),
            passed,
            total_requests: s.total_requests,
            total_errors: s.total_errors,
            error_rate_pct: s.error_rate * 100.0,
            peak_rps: s.peak_rps,
            p50_ms: s.p50_ms,
            p95_ms: s.p95_ms,
            p99_ms: s.p99_ms,
        }
    }
}

impl LoadTestReport {
    pub fn new(reports: Vec<ScenarioReport>) -> Self {
        let all_passed = reports.iter().all(|r| r.passed);
        Self { scenarios: reports, all_passed }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn failed_count(&self) -> usize {
        self.scenarios.iter().filter(|r| !r.passed).count()
    }
}
