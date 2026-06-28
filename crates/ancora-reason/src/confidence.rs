//! Confidence aggregation: compute mean confidence from per-step scores.

pub struct ConfidenceAggregator {
    pub min_confidence: f64,
}

impl ConfidenceAggregator {
    pub fn new(min_confidence: f64) -> Self {
        Self { min_confidence }
    }

    pub fn aggregate(&self, scores: &[f64]) -> f64 {
        if scores.is_empty() {
            return 0.0;
        }
        scores.iter().sum::<f64>() / scores.len() as f64
    }

    pub fn is_confident(&self, scores: &[f64]) -> bool {
        self.aggregate(scores) >= self.min_confidence
    }
}
