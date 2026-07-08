/// A memory item with associated salience metadata.
#[derive(Debug, Clone)]
pub struct SalienceItem {
    pub key: String,
    pub content: String,
    pub importance: u32,
    pub access_count: u32,
    pub age_secs: u64,
}

/// Computes salience score for retention decisions.
pub struct SalienceScorer {
    pub importance_weight: f64,
    pub recency_weight: f64,
    pub frequency_weight: f64,
}

impl SalienceScorer {
    pub fn default_weights() -> Self {
        Self {
            importance_weight: 2.0,
            recency_weight: 1.0,
            frequency_weight: 0.5,
        }
    }

    pub fn score(&self, item: &SalienceItem) -> f64 {
        let recency = 1.0 / (1.0 + item.age_secs as f64 / 3600.0);
        let freq = (item.access_count as f64).ln_1p();
        self.importance_weight * item.importance as f64
            + self.recency_weight * recency
            + self.frequency_weight * freq
    }
}
