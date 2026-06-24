/// Input and output token counts for a single model call.
#[derive(Debug, Clone, Copy, Default)]
pub struct TokenUsage {
    pub tokens_in: u64,
    pub tokens_out: u64,
}

/// Accumulated token and cost information for a single node.
#[derive(Debug, Clone)]
pub struct NodeCost {
    pub node_id: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub cost_usd: f64,
}

/// Aggregated cost information for an entire run.
#[derive(Debug, Clone)]
pub struct CostSummary {
    pub nodes: Vec<NodeCost>,
    pub total_tokens_in: u64,
    pub total_tokens_out: u64,
    pub total_cost_usd: f64,
}

/// Accumulates token usage records and computes cost summaries.
pub struct CostTracker {
    records: Vec<(String, TokenUsage)>,
    usd_per_input_token: f64,
    usd_per_output_token: f64,
}

impl CostTracker {
    pub fn new(usd_per_input_token: f64, usd_per_output_token: f64) -> Self {
        Self {
            records: Vec::new(),
            usd_per_input_token,
            usd_per_output_token,
        }
    }

    /// Record token usage for one activity or model call on `node_id`.
    pub fn record(&mut self, node_id: impl Into<String>, usage: TokenUsage) {
        self.records.push((node_id.into(), usage));
    }

    /// Aggregate per-node and run-wide totals into a `CostSummary`.
    pub fn summary(&self) -> CostSummary {
        let mut node_map: std::collections::HashMap<&str, (u64, u64)> =
            std::collections::HashMap::new();

        for (node_id, usage) in &self.records {
            let entry = node_map.entry(node_id.as_str()).or_default();
            entry.0 += usage.tokens_in;
            entry.1 += usage.tokens_out;
        }

        let mut nodes: Vec<NodeCost> = node_map
            .into_iter()
            .map(|(node_id, (tokens_in, tokens_out))| {
                let cost_usd = tokens_in as f64 * self.usd_per_input_token
                    + tokens_out as f64 * self.usd_per_output_token;
                NodeCost { node_id: node_id.to_string(), tokens_in, tokens_out, cost_usd }
            })
            .collect();
        nodes.sort_by(|a, b| a.node_id.cmp(&b.node_id));

        let total_tokens_in = nodes.iter().map(|n| n.tokens_in).sum();
        let total_tokens_out = nodes.iter().map(|n| n.tokens_out).sum();
        let total_cost_usd = nodes.iter().map(|n| n.cost_usd).sum();

        CostSummary { nodes, total_tokens_in, total_tokens_out, total_cost_usd }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_aggregation_matches_recorded_usage() {
        let mut tracker = CostTracker::new(0.001, 0.002);
        tracker.record("node-a", TokenUsage { tokens_in: 100, tokens_out: 50 });
        tracker.record("node-b", TokenUsage { tokens_in: 200, tokens_out: 80 });
        tracker.record("node-a", TokenUsage { tokens_in: 10, tokens_out: 5 });

        let summary = tracker.summary();

        assert_eq!(summary.total_tokens_in, 310);
        assert_eq!(summary.total_tokens_out, 135);

        let node_a = summary.nodes.iter().find(|n| n.node_id == "node-a").unwrap();
        assert_eq!(node_a.tokens_in, 110);
        assert_eq!(node_a.tokens_out, 55);

        let expected_a = 110.0 * 0.001 + 55.0 * 0.002;
        assert!((node_a.cost_usd - expected_a).abs() < 1e-9);

        let expected_total = summary.total_tokens_in as f64 * 0.001
            + summary.total_tokens_out as f64 * 0.002;
        assert!((summary.total_cost_usd - expected_total).abs() < 1e-9);
    }
}
