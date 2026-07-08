/// Guardrails halt an experiment when a variant is causing harm.
///
/// A guardrail monitors a safety metric (e.g. error rate, latency) and
/// triggers if the metric for any treatment variant exceeds a threshold
/// relative to the control.
use crate::outcome::OutcomeStore;

/// Direction of the guardrail check.
#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailDirection {
    /// Trigger if treatment metric is greater than threshold multiple of control.
    TreatmentAbove,
    /// Trigger if treatment metric is below a minimum absolute floor.
    AbsoluteFloor { floor: f64 },
}

/// A single guardrail rule.
#[derive(Debug, Clone)]
pub struct Guardrail {
    pub name: String,
    pub experiment_id: String,
    pub safety_metric: String,
    pub control_variant: String,
    pub direction: GuardrailDirection,
    /// Multiplier above control mean that triggers the guardrail (for TreatmentAbove).
    pub threshold_multiplier: f64,
}

/// Result of evaluating a guardrail.
#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailStatus {
    /// Guardrail not triggered; experiment may continue.
    Clear,
    /// Guardrail triggered for the named variant with the observed value.
    Triggered {
        variant: String,
        observed: f64,
        limit: f64,
    },
    /// Not enough data to evaluate.
    Insufficient,
}

impl Guardrail {
    pub fn new(
        name: impl Into<String>,
        experiment_id: impl Into<String>,
        safety_metric: impl Into<String>,
        control_variant: impl Into<String>,
        direction: GuardrailDirection,
        threshold_multiplier: f64,
    ) -> Self {
        Guardrail {
            name: name.into(),
            experiment_id: experiment_id.into(),
            safety_metric: safety_metric.into(),
            control_variant: control_variant.into(),
            direction,
            threshold_multiplier,
        }
    }

    /// Evaluate the guardrail against live outcome data.
    ///
    /// Returns `Triggered` for the first offending treatment variant found.
    pub fn evaluate(&self, store: &OutcomeStore, treatment_variants: &[&str]) -> GuardrailStatus {
        let ctrl_stats = store.stats_for_variant(&self.experiment_id, &self.control_variant);

        match &self.direction {
            GuardrailDirection::TreatmentAbove => {
                let ctrl = match ctrl_stats {
                    Some(s) if s.n >= 1 => s,
                    _ => return GuardrailStatus::Insufficient,
                };
                let limit = ctrl.mean * self.threshold_multiplier;
                for &trt in treatment_variants {
                    if let Some(trt_stats) = store.stats_for_variant(&self.experiment_id, trt) {
                        if trt_stats.n >= 1 && trt_stats.mean > limit {
                            return GuardrailStatus::Triggered {
                                variant: trt.to_string(),
                                observed: trt_stats.mean,
                                limit,
                            };
                        }
                    }
                }
                GuardrailStatus::Clear
            }
            GuardrailDirection::AbsoluteFloor { floor } => {
                for &trt in treatment_variants {
                    if let Some(trt_stats) = store.stats_for_variant(&self.experiment_id, trt) {
                        if trt_stats.n >= 1 && trt_stats.mean < *floor {
                            return GuardrailStatus::Triggered {
                                variant: trt.to_string(),
                                observed: trt_stats.mean,
                                limit: *floor,
                            };
                        }
                    }
                }
                GuardrailStatus::Clear
            }
        }
    }
}

/// Evaluate all guardrails and return any that triggered.
pub fn evaluate_all(
    guardrails: &[Guardrail],
    store: &OutcomeStore,
    treatment_variants: &[&str],
) -> Vec<(String, GuardrailStatus)> {
    guardrails
        .iter()
        .filter_map(|g| {
            let status = g.evaluate(store, treatment_variants);
            if status != GuardrailStatus::Clear {
                Some((g.name.clone(), status))
            } else {
                None
            }
        })
        .collect()
}
