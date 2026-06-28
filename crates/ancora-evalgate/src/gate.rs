/// Gate pass-fail decision.
///
/// A `Gate` combines regression detection with significance checking to
/// produce a final `GateDecision` for a set of metrics.

use crate::baseline::Baseline;
use crate::regression::{self, RegressionResult};
use crate::significance::{is_significant, SampleStats};
use crate::threshold::ThresholdRegistry;

/// The verdict for one metric.
#[derive(Debug, Clone)]
pub struct MetricVerdict {
    pub metric: String,
    pub regression: RegressionResult,
    /// Whether the delta was statistically significant.
    pub significant: bool,
    /// Whether this metric causes the gate to fail.
    pub blocks: bool,
}

/// Overall gate decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateDecision {
    Pass,
    Fail,
}

/// Evaluate all metrics in `candidate` against `baseline` and `registry`.
///
/// A metric blocks the gate only when it shows a statistically significant
/// regression beyond the allowed threshold.
pub fn evaluate(
    baseline: &Baseline,
    candidate_metrics: &std::collections::HashMap<String, f64>,
    registry: &ThresholdRegistry,
    sample_n: usize,
    sample_std: f64,
    alpha: f64,
) -> (GateDecision, Vec<MetricVerdict>) {
    let mut verdicts = Vec::new();
    let mut any_blocking = false;

    for (metric, &cand_val) in candidate_metrics {
        let Some(policy) = registry.get(metric) else {
            continue;
        };
        let Some(base_val) = baseline.get(metric) else {
            continue;
        };

        let reg = regression::detect(base_val, cand_val, policy);

        // Build synthetic SampleStats for significance check.
        let base_stats = SampleStats::new(sample_n.max(2), base_val, sample_std)
            .unwrap_or(SampleStats { n: 2, mean: base_val, std_dev: 0.0 });
        let cand_stats = SampleStats::new(sample_n.max(2), cand_val, sample_std)
            .unwrap_or(SampleStats { n: 2, mean: cand_val, std_dev: 0.0 });

        let significant = is_significant(&base_stats, &cand_stats, alpha);
        let blocks = reg.is_blocking() && significant;

        if blocks {
            any_blocking = true;
        }

        verdicts.push(MetricVerdict {
            metric: metric.clone(),
            regression: reg,
            significant,
            blocks,
        });
    }

    let decision = if any_blocking {
        GateDecision::Fail
    } else {
        GateDecision::Pass
    };

    (decision, verdicts)
}
