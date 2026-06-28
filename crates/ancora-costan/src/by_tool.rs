/// Cost breakdown by tool invocation.

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ToolCostBreakdown {
    costs: HashMap<String, f64>,
    invocations: HashMap<String, u64>,
}

impl ToolCostBreakdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, tool: &str, cost_usd: f64) {
        *self.costs.entry(tool.to_string()).or_insert(0.0) += cost_usd;
        *self.invocations.entry(tool.to_string()).or_insert(0) += 1;
    }

    pub fn cost_for(&self, tool: &str) -> f64 {
        self.costs.get(tool).copied().unwrap_or(0.0)
    }

    pub fn invocations_for(&self, tool: &str) -> u64 {
        self.invocations.get(tool).copied().unwrap_or(0)
    }

    pub fn total_cost(&self) -> f64 {
        self.costs.values().sum()
    }

    pub fn cost_per_invocation(&self, tool: &str) -> Option<f64> {
        let inv = self.invocations_for(tool);
        if inv == 0 {
            return None;
        }
        Some(self.cost_for(tool) / inv as f64)
    }

    pub fn top_tools_by_cost(&self) -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> =
            self.costs.iter().map(|(k, v)| (k.clone(), *v)).collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    pub fn tools(&self) -> Vec<String> {
        let mut v: Vec<String> = self.costs.keys().cloned().collect();
        v.sort();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_cost_tracking() {
        let mut b = ToolCostBreakdown::new();
        b.record("search", 0.10);
        b.record("search", 0.05);
        b.record("calculator", 0.02);
        assert!((b.cost_for("search") - 0.15).abs() < 1e-9);
        assert_eq!(b.invocations_for("search"), 2);
    }
}
