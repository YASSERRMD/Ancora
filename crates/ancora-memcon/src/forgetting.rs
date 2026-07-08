use crate::salience::{SalienceItem, SalienceScorer};

/// Policy for removing low-salience old items.
pub struct ForgettingPolicy {
    pub min_salience: f64,
    pub max_age_secs: u64,
}

impl ForgettingPolicy {
    pub fn new(min_salience: f64, max_age_secs: u64) -> Self {
        Self {
            min_salience,
            max_age_secs,
        }
    }

    pub fn should_forget(&self, item: &SalienceItem, scorer: &SalienceScorer) -> bool {
        if item.age_secs > self.max_age_secs {
            return true;
        }
        scorer.score(item) < self.min_salience
    }

    pub fn prune(&self, items: Vec<SalienceItem>, scorer: &SalienceScorer) -> Vec<SalienceItem> {
        items
            .into_iter()
            .filter(|i| !self.should_forget(i, scorer))
            .collect()
    }
}
