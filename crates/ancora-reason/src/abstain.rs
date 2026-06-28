//! Abstention policy: mark steps as abstained when evidence confidence is too low.

use crate::confidence::ConfidenceAggregator;
use crate::decompose::{ReasoningStep, StepStatus};

pub struct AbstentionPolicy {
    pub aggregator: ConfidenceAggregator,
}

impl AbstentionPolicy {
    pub fn new(min_confidence: f64) -> Self {
        Self {
            aggregator: ConfidenceAggregator::new(min_confidence),
        }
    }

    /// Returns true if the step was abstained due to low confidence.
    pub fn apply(&self, step: &mut ReasoningStep, scores: &[f64]) -> bool {
        if !self.aggregator.is_confident(scores) {
            step.status = StepStatus::Abstained;
            true
        } else {
            false
        }
    }
}
