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
    fn cost_summary_correct_for_openai_fixture() {
        use crate::providers::openai::build_openai_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"hi","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":4}}"#;
        let client = OpenAiClient::new(Arc::new(build_openai_profile()));
        let resp = client.parse_response(FIXTURE, "gpt-4o").unwrap();
        let summary = UsageSummary::from_response("openai", "gpt-4o", &resp);
        // 8 * $2.5/M + 4 * $10.0/M = 0.000020 + 0.000040 = 0.000060
        let expected = 8.0 * 2.5 / 1_000_000.0 + 4.0 * 10.0 / 1_000_000.0;
        assert!((summary.cost_usd.unwrap() - expected).abs() < 1e-12);
    }

    #[test]
    fn cost_summary_correct_for_azure_fixture() {
        use crate::providers::azure::build_azure_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"ok","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":5,"completion_tokens":2}}"#;
        let client = OpenAiClient::new(Arc::new(build_azure_profile("r", "dep", "2024-02-01")));
        let resp = client.parse_response(FIXTURE, "dep").unwrap();
        let summary = UsageSummary::from_response("azure-openai", "dep", &resp);
        // Azure deployment has no pricing metadata -> cost is None
        assert!(summary.cost_usd.is_none(), "Azure profile has no pricing so cost should be None");
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

    #[test]
    fn cost_summary_correct_for_stepfun_fixture() {
        use crate::providers::stepfun::build_stepfun_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"step","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":5}}"#;
        let client = OpenAiClient::new(Arc::new(build_stepfun_profile()));
        let response = client.parse_response(FIXTURE, "step-1-32k").unwrap();
        let summary = UsageSummary::from_response("stepfun", "step-1-32k", &response);
        // step-1-32k: $0.07/M in + $0.07/M out
        let expected = 10.0 * 0.07 / 1_000_000.0 + 5.0 * 0.07 / 1_000_000.0;
        assert!((summary.cost_usd.unwrap() - expected).abs() < 1e-12);
    }

    #[test]
    fn cost_summary_correct_for_ernie_fixture() {
        use crate::providers::ernie::build_ernie_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"ernie","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":3}}"#;
        let client = OpenAiClient::new(Arc::new(build_ernie_profile()));
        let response = client.parse_response(FIXTURE, "ernie-speed-8k").unwrap();
        let summary = UsageSummary::from_response("ernie", "ernie-speed-8k", &response);
        // ernie-speed-8k: $0.004/M in + $0.008/M out
        let expected = 9.0 * 0.004 / 1_000_000.0 + 3.0 * 0.008 / 1_000_000.0;
        assert!((summary.cost_usd.unwrap() - expected).abs() < 1e-15);
    }

    #[test]
    fn cost_summary_correct_for_hunyuan_fixture() {
        use crate::providers::hunyuan::build_hunyuan_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"hunyuan","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":11,"completion_tokens":5}}"#;
        let client = OpenAiClient::new(Arc::new(build_hunyuan_profile()));
        let response = client.parse_response(FIXTURE, "hunyuan-standard").unwrap();
        let summary = UsageSummary::from_response("hunyuan", "hunyuan-standard", &response);
        // hunyuan-standard: $0.05/M in + $0.05/M out
        let expected = 11.0 * 0.05 / 1_000_000.0 + 5.0 * 0.05 / 1_000_000.0;
        assert!((summary.cost_usd.unwrap() - expected).abs() < 1e-12);
    }

    #[test]
    fn cost_summary_correct_for_doubao_fixture() {
        use crate::providers::doubao::build_doubao_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"doubao","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":12,"completion_tokens":4}}"#;
        let client = OpenAiClient::new(Arc::new(build_doubao_profile()));
        let response = client.parse_response(FIXTURE, "doubao-1.5-lite-32k").unwrap();
        let summary = UsageSummary::from_response("doubao", "doubao-1.5-lite-32k", &response);
        // doubao-lite: $0.01/M in + $0.03/M out
        let expected = 12.0 * 0.01 / 1_000_000.0 + 4.0 * 0.03 / 1_000_000.0;
        assert!((summary.cost_usd.unwrap() - expected).abs() < 1e-15);
    }

    #[test]
    fn cost_summary_correct_for_hunyuan_lite_free() {
        use crate::providers::hunyuan::build_hunyuan_profile;
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"free","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":20,"completion_tokens":10}}"#;
        let client = OpenAiClient::new(Arc::new(build_hunyuan_profile()));
        let response = client.parse_response(FIXTURE, "hunyuan-lite").unwrap();
        let summary = UsageSummary::from_response("hunyuan", "hunyuan-lite", &response);
        // hunyuan-lite: $0.0/M both -- should be exactly zero
        assert_eq!(summary.cost_usd, Some(0.0));
    }
}
