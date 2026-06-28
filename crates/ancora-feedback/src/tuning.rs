use crate::aggregation::FeedbackMetrics;

/// A tuning signal derived from aggregated feedback.
#[derive(Debug, Clone)]
pub struct TuningSignal {
    /// The guardrail or policy this signal targets.
    pub target: String,
    /// Suggested threshold adjustment (-1.0 to +1.0).
    pub adjustment: f64,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Configuration for the tuning input generator.
#[derive(Debug, Clone)]
pub struct TuningConfig {
    /// Approval rate below which a signal is generated to tighten guardrails.
    pub low_approval_threshold: f64,
    /// Approval rate above which a signal is generated to relax guardrails.
    pub high_approval_threshold: f64,
    /// Magnitude of the suggested adjustment.
    pub adjustment_step: f64,
}

impl Default for TuningConfig {
    fn default() -> Self {
        Self {
            low_approval_threshold: 0.5,
            high_approval_threshold: 0.9,
            adjustment_step: 0.05,
        }
    }
}

/// Derive tuning signals from feedback metrics for a named guardrail.
pub fn derive_tuning_signal(
    target: impl Into<String>,
    metrics: &FeedbackMetrics,
    config: &TuningConfig,
) -> Option<TuningSignal> {
    let rate = metrics.approval_rate?;
    let target = target.into();

    if rate < config.low_approval_threshold {
        Some(TuningSignal {
            target: target.clone(),
            adjustment: -config.adjustment_step,
            rationale: format!(
                "Approval rate {:.1}% is below threshold {:.1}%; tightening guardrail",
                rate * 100.0,
                config.low_approval_threshold * 100.0
            ),
        })
    } else if rate > config.high_approval_threshold {
        Some(TuningSignal {
            target: target.clone(),
            adjustment: config.adjustment_step,
            rationale: format!(
                "Approval rate {:.1}% exceeds threshold {:.1}%; relaxing guardrail",
                rate * 100.0,
                config.high_approval_threshold * 100.0
            ),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::FeedbackMetrics;

    fn make_metrics(up: usize, total: usize) -> FeedbackMetrics {
        FeedbackMetrics {
            total,
            thumbs_up: up,
            thumbs_down: total - up,
            with_comment: 0,
            approval_rate: if total == 0 { None } else { Some(up as f64 / total as f64) },
        }
    }

    #[test]
    fn low_approval_produces_negative_adjustment() {
        let cfg = TuningConfig::default();
        let metrics = make_metrics(2, 10); // 20% approval
        let signal = derive_tuning_signal("toxicity-guard", &metrics, &cfg).unwrap();
        assert!(signal.adjustment < 0.0);
    }

    #[test]
    fn high_approval_produces_positive_adjustment() {
        let cfg = TuningConfig::default();
        let metrics = make_metrics(10, 10); // 100% approval
        let signal = derive_tuning_signal("safety-guard", &metrics, &cfg).unwrap();
        assert!(signal.adjustment > 0.0);
    }

    #[test]
    fn mid_range_approval_produces_no_signal() {
        let cfg = TuningConfig::default();
        let metrics = make_metrics(7, 10); // 70% approval
        let signal = derive_tuning_signal("guard", &metrics, &cfg);
        assert!(signal.is_none());
    }
}
