/// Threshold policy per metric.
///
/// A `ThresholdPolicy` defines how much a metric may regress before a gate
/// blocks the PR, expressed either as an absolute delta or a relative
/// fraction of the baseline value.

/// Direction of a metric - whether higher or lower is better.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricDirection {
    /// A higher value is better (e.g. accuracy, recall).
    HigherIsBetter,
    /// A lower value is better (e.g. latency, cost).
    LowerIsBetter,
}

/// How the allowed regression is expressed.
#[derive(Debug, Clone, Copy)]
pub enum ThresholdKind {
    /// Absolute change in the metric's native unit (e.g. 0.02 for 2 pp accuracy).
    Absolute(f64),
    /// Relative fraction of the baseline value (e.g. 0.05 means 5 %).
    Relative(f64),
}

/// Policy associating a metric with its allowed regression.
#[derive(Debug, Clone)]
pub struct ThresholdPolicy {
    pub metric: String,
    pub direction: MetricDirection,
    pub allowed_regression: ThresholdKind,
}

impl ThresholdPolicy {
    /// Construct a new policy.
    pub fn new(
        metric: impl Into<String>,
        direction: MetricDirection,
        allowed_regression: ThresholdKind,
    ) -> Self {
        Self {
            metric: metric.into(),
            direction,
            allowed_regression,
        }
    }

    /// Return the maximum tolerable regression given a `baseline` value.
    /// The returned value is always non-negative.
    pub fn max_regression(&self, baseline: f64) -> f64 {
        match self.allowed_regression {
            ThresholdKind::Absolute(d) => d.abs(),
            ThresholdKind::Relative(r) => (r * baseline).abs(),
        }
    }

    /// Return `true` when `candidate` is within the allowed regression of
    /// `baseline` for this metric's direction.
    pub fn within_threshold(&self, baseline: f64, candidate: f64) -> bool {
        let max = self.max_regression(baseline);
        match self.direction {
            MetricDirection::HigherIsBetter => {
                // regression = baseline - candidate (positive means worse)
                (baseline - candidate) <= max
            }
            MetricDirection::LowerIsBetter => {
                // regression = candidate - baseline (positive means worse)
                (candidate - baseline) <= max
            }
        }
    }
}

/// Registry of per-metric threshold policies.
#[derive(Debug, Default)]
pub struct ThresholdRegistry {
    policies: Vec<ThresholdPolicy>,
}

impl ThresholdRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, policy: ThresholdPolicy) {
        self.policies.push(policy);
    }

    /// Look up the policy for the given metric name, if any.
    pub fn get(&self, metric: &str) -> Option<&ThresholdPolicy> {
        self.policies.iter().find(|p| p.metric == metric)
    }
}
