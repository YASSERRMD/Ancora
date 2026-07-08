//! Cost analytics for LLM usage tracking.

/// Tracks token usage for a single LLM request.
#[derive(Debug, Clone, Default)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

impl TokenUsage {
    pub fn new(input: u64, output: u64) -> Self {
        Self {
            input_tokens: input,
            output_tokens: output,
        }
    }

    pub fn total(&self) -> u64 {
        self.input_tokens + self.output_tokens
    }
}

/// Per-model pricing in USD per 1000 tokens.
#[derive(Debug, Clone)]
pub struct ModelPricing {
    pub model: String,
    pub input_cost_per_1k: f64,
    pub output_cost_per_1k: f64,
}

impl ModelPricing {
    pub fn new(model: impl Into<String>, input_cost_per_1k: f64, output_cost_per_1k: f64) -> Self {
        Self {
            model: model.into(),
            input_cost_per_1k,
            output_cost_per_1k,
        }
    }

    /// Compute the cost in USD for the given token usage.
    pub fn cost_usd(&self, usage: &TokenUsage) -> f64 {
        let input_cost = (usage.input_tokens as f64 / 1000.0) * self.input_cost_per_1k;
        let output_cost = (usage.output_tokens as f64 / 1000.0) * self.output_cost_per_1k;
        input_cost + output_cost
    }
}

/// Aggregates cost across multiple requests.
#[derive(Debug, Default)]
pub struct CostAggregator {
    pub total_usd: f64,
    pub request_count: u64,
    pub total_tokens: u64,
}

impl CostAggregator {
    pub fn record(&mut self, cost_usd: f64, tokens: u64) {
        self.total_usd += cost_usd;
        self.request_count += 1;
        self.total_tokens += tokens;
    }

    pub fn average_cost_per_request(&self) -> f64 {
        if self.request_count == 0 {
            0.0
        } else {
            self.total_usd / self.request_count as f64
        }
    }
}
