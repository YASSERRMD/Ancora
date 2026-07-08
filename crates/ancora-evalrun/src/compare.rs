/// Compare two eval runs and highlight deltas.
use crate::aggregate::AggregateMetrics;
use crate::executor::RunId;

/// Delta between two eval runs.
#[derive(Debug, Clone)]
pub struct RunDelta {
    pub run_a: RunId,
    pub run_b: RunId,
    pub pass_rate_delta: f64,
    pub pass_rate_a: f64,
    pub pass_rate_b: f64,
    pub latency_delta_ms: f64,
    pub cost_delta_tokens: i64,
    pub significant: bool,
}

/// Threshold below which a change in pass rate is not considered significant.
pub const SIGNIFICANCE_THRESHOLD: f64 = 0.01;

/// Compare two runs given their metrics.
pub fn compare_runs(
    id_a: RunId,
    metrics_a: &AggregateMetrics,
    id_b: RunId,
    metrics_b: &AggregateMetrics,
) -> RunDelta {
    let pass_rate_delta = metrics_b.pass_rate - metrics_a.pass_rate;
    let latency_delta_ms = metrics_b.mean_latency_ms - metrics_a.mean_latency_ms;
    let cost_delta_tokens = metrics_b.total_cost_tokens as i64 - metrics_a.total_cost_tokens as i64;
    let significant = pass_rate_delta.abs() >= SIGNIFICANCE_THRESHOLD;

    RunDelta {
        run_a: id_a,
        run_b: id_b,
        pass_rate_delta,
        pass_rate_a: metrics_a.pass_rate,
        pass_rate_b: metrics_b.pass_rate,
        latency_delta_ms,
        cost_delta_tokens,
        significant,
    }
}

/// Verdict for a comparison.
#[derive(Debug, Clone, PartialEq)]
pub enum CompareVerdict {
    Improved,
    Regressed,
    Neutral,
}

impl RunDelta {
    pub fn verdict(&self) -> CompareVerdict {
        if !self.significant {
            return CompareVerdict::Neutral;
        }
        if self.pass_rate_delta > 0.0 {
            CompareVerdict::Improved
        } else {
            CompareVerdict::Regressed
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} vs {}: pass_rate {:.3} -> {:.3} (delta={:+.3}) latency {:+.1}ms cost {:+}tk verdict={:?}",
            self.run_a.0,
            self.run_b.0,
            self.pass_rate_a,
            self.pass_rate_b,
            self.pass_rate_delta,
            self.latency_delta_ms,
            self.cost_delta_tokens,
            self.verdict(),
        )
    }
}

/// Compare confidence intervals: do they overlap?
pub fn ci_overlap(a_lower: f64, a_upper: f64, b_lower: f64, b_upper: f64) -> bool {
    !(b_lower > a_upper || a_lower > b_upper)
}
