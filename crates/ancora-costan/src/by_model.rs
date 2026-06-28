/// Cost breakdown by model.

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ModelCostBreakdown {
    /// map from model id to accumulated cost in USD.
    costs: HashMap<String, f64>,
    /// map from model id to total tokens.
    tokens: HashMap<String, u64>,
}

impl ModelCostBreakdown {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record cost for a specific model.
    pub fn record(&mut self, model: &str, cost_usd: f64, tokens: u64) {
        *self.costs.entry(model.to_string()).or_insert(0.0) += cost_usd;
        *self.tokens.entry(model.to_string()).or_insert(0) += tokens;
    }

    /// Get cost for a model. Returns 0 if not recorded.
    pub fn cost_for(&self, model: &str) -> f64 {
        self.costs.get(model).copied().unwrap_or(0.0)
    }

    /// Get tokens for a model.
    pub fn tokens_for(&self, model: &str) -> u64 {
        self.tokens.get(model).copied().unwrap_or(0)
    }

    /// Total cost across all models.
    pub fn total_cost(&self) -> f64 {
        self.costs.values().sum()
    }

    /// Return all models sorted by descending cost.
    pub fn top_models(&self) -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> =
            self.costs.iter().map(|(k, v)| (k.clone(), *v)).collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    /// Cost fraction for a model (0.0 if total is zero).
    pub fn fraction_for(&self, model: &str) -> f64 {
        let total = self.total_cost();
        if total == 0.0 {
            return 0.0;
        }
        self.cost_for(model) / total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_models_ordered() {
        let mut b = ModelCostBreakdown::new();
        b.record("model-a", 1.0, 1000);
        b.record("model-b", 5.0, 5000);
        let top = b.top_models();
        assert_eq!(top[0].0, "model-b");
    }
}
