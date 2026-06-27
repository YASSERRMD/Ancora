use crate::types::CompletionResponse;

/// Aggregated token and cost summary across one or more responses.
#[derive(Debug, Default, Clone)]
pub struct UsageSummary {
    pub provider: String,
    pub model_id: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub cost_usd: Option<f64>,
}

impl UsageSummary {
    pub fn from_response(provider: impl Into<String>, model_id: impl Into<String>, resp: &CompletionResponse) -> Self {
        Self {
            provider: provider.into(),
            model_id: model_id.into(),
            tokens_in: resp.tokens_in,
            tokens_out: resp.tokens_out,
            cost_usd: resp.cost_usd,
        }
    }

    /// Combine usage from multiple responses (same provider and model).
    pub fn merge(mut self, other: &UsageSummary) -> Self {
        self.tokens_in += other.tokens_in;
        self.tokens_out += other.tokens_out;
        self.cost_usd = match (self.cost_usd, other.cost_usd) {
            (Some(a), Some(b)) => Some(a + b),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resp(tokens_in: u64, tokens_out: u64, cost: Option<f64>) -> CompletionResponse {
        CompletionResponse {
            content: String::new(),
            tokens_in,
            tokens_out,
            cost_usd: cost,
            tool_calls: vec![],
        }
    }

    #[test]
    fn usage_summary_from_response() {
        let r = resp(10, 4, Some(0.001));
        let s = UsageSummary::from_response("openai", "gpt-4o", &r);
        assert_eq!(s.tokens_in, 10);
        assert_eq!(s.tokens_out, 4);
        assert!((s.cost_usd.unwrap() - 0.001).abs() < 1e-9);
    }

    #[test]
    fn usage_summary_merge_adds_tokens_and_cost() {
        let r1 = resp(10, 4, Some(0.001));
        let r2 = resp(20, 8, Some(0.002));
        let s1 = UsageSummary::from_response("openai", "gpt-4o", &r1);
        let s2 = UsageSummary::from_response("openai", "gpt-4o", &r2);
        let merged = s1.merge(&s2);
        assert_eq!(merged.tokens_in, 30);
        assert_eq!(merged.tokens_out, 12);
        assert!((merged.cost_usd.unwrap() - 0.003).abs() < 1e-9);
    }

    #[test]
    fn usage_summary_merge_handles_none_cost() {
        let r1 = resp(10, 4, None);
        let r2 = resp(20, 8, Some(0.002));
        let s1 = UsageSummary::from_response("openai", "gpt-4o", &r1);
        let s2 = UsageSummary::from_response("openai", "gpt-4o", &r2);
        let merged = s1.merge(&s2);
        assert_eq!(merged.cost_usd, Some(0.002));
    }
}
