/// Cost breakdown by LLM provider (e.g., Anthropic, OpenAI, Mistral).

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ProviderCostBreakdown {
    costs: HashMap<String, f64>,
    requests: HashMap<String, u64>,
}

impl ProviderCostBreakdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, provider: &str, cost_usd: f64) {
        *self.costs.entry(provider.to_string()).or_insert(0.0) += cost_usd;
        *self.requests.entry(provider.to_string()).or_insert(0) += 1;
    }

    pub fn cost_for(&self, provider: &str) -> f64 {
        self.costs.get(provider).copied().unwrap_or(0.0)
    }

    pub fn requests_for(&self, provider: &str) -> u64 {
        self.requests.get(provider).copied().unwrap_or(0)
    }

    pub fn total_cost(&self) -> f64 {
        self.costs.values().sum()
    }

    /// Average cost per request for a provider.
    pub fn avg_cost_per_request(&self, provider: &str) -> Option<f64> {
        let reqs = self.requests_for(provider);
        if reqs == 0 {
            return None;
        }
        Some(self.cost_for(provider) / reqs as f64)
    }

    pub fn providers(&self) -> Vec<String> {
        let mut v: Vec<String> = self.costs.keys().cloned().collect();
        v.sort();
        v
    }

    pub fn top_providers(&self) -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> =
            self.costs.iter().map(|(k, v)| (k.clone(), *v)).collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avg_cost_per_request_correct() {
        let mut b = ProviderCostBreakdown::new();
        b.record("anthropic", 2.0);
        b.record("anthropic", 4.0);
        let avg = b.avg_cost_per_request("anthropic").unwrap();
        assert!((avg - 3.0).abs() < 1e-9);
    }
}
