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
}
