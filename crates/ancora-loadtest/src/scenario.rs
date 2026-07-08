use crate::soak::{SoakHarness, SoakSummary};
use crate::workload::WorkloadSpec;

pub struct ScenarioResult {
    pub name: String,
    pub passed: bool,
    pub summary: SoakSummary,
}

pub struct Scenario {
    pub spec: WorkloadSpec,
    pub max_error_rate: f64,
    pub max_p99_ms: u64,
}

impl Scenario {
    pub fn new(spec: WorkloadSpec, max_error_rate: f64, max_p99_ms: u64) -> Self {
        Self {
            spec,
            max_error_rate,
            max_p99_ms,
        }
    }

    /// Run the scenario using the provided tick function.
    /// `tick(i)` returns `(latency_ms, is_error)` for request i.
    pub fn run<F>(&self, mut tick: F) -> ScenarioResult
    where
        F: FnMut(u64) -> (u64, bool),
    {
        let mut harness = SoakHarness::new(&self.spec.name, self.spec.duration_secs, 0);
        let total = self.spec.total_expected_requests();
        for i in 0..total {
            let (lat, err) = tick(i);
            let ts = if self.spec.target_rps > 0.0 {
                (i as f64 / self.spec.target_rps) as u64
            } else {
                i
            };
            harness.record(lat, err, ts);
        }
        harness.finish(self.spec.duration_secs);
        let summary = harness.summary();
        let passed = summary.passes_slo(self.max_error_rate, self.max_p99_ms);
        ScenarioResult {
            name: self.spec.name.clone(),
            passed,
            summary,
        }
    }
}
