use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the OpenAI provider profile with the current model catalog and pricing.
///
/// Reads the API key from `OPENAI_API_KEY` at call time.
pub fn build_openai_profile() -> ProviderProfile {
    ProviderProfile::new(
        "openai",
        "https://api.openai.com",
        AuthStrategy::BearerToken { env_var: "OPENAI_API_KEY".to_owned() },
    )
    // GPT-4o family
    .add_model(
        ModelMeta::new("gpt-4o", 128_000)
            .with_pricing(2.5, 10.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("gpt-4o-mini", 128_000)
            .with_pricing(0.15, 0.60)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    // o-series (reasoning)
    .add_model(
        ModelMeta::new("o1", 200_000)
            .with_pricing(15.0, 60.0)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("o3-mini", 200_000)
            .with_pricing(1.1, 4.4)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("o4-mini", 200_000)
            .with_pricing(1.1, 4.4)
            .with_tools()
            .with_streaming(),
    )
    // Aliases
    .add_alias("gpt-4o-latest", "gpt-4o")
    .add_alias("o3", "o3-mini")
}

#[cfg(test)]
const FIXTURE: &str = r#"{"id":"chatcmpl-openai","choices":[{"message":{"role":"assistant","content":"Hello from OpenAI","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":4}}"#;

#[cfg(test)]
const FIXTURE_STREAM_TOKENS: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" world"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::OpenAiClient;
    use std::sync::Arc;

    fn client() -> OpenAiClient {
        OpenAiClient::new(Arc::new(build_openai_profile()))
    }

    #[test]
    fn openai_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE, "gpt-4o").unwrap();
        assert_eq!(resp.content, "Hello from OpenAI");
        assert_eq!(resp.tokens_in, 8);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn openai_cost_from_fixture() {
        let resp = client().parse_response(FIXTURE, "gpt-4o").unwrap();
        // 8 in * $2.5/M + 4 out * $10/M
        let expected = 8.0 * 2.5 / 1_000_000.0 + 4.0 * 10.0 / 1_000_000.0;
        let cost = resp.cost_usd.unwrap();
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn openai_streaming_fixture_yields_ordered_tokens() {
        let mut tokens = Vec::new();
        for line in FIXTURE_STREAM_TOKENS {
            if let Some(ev) = OpenAiClient::parse_sse_line(line) {
                tokens.push(ev);
            }
        }
        assert_eq!(tokens[0].text, "Hello");
        assert!(!tokens[0].finished);
        assert_eq!(tokens[1].text, " world");
        assert!(tokens[1].finished);
        assert!(tokens[2].finished && tokens[2].text.is_empty());
    }

    #[test]
    fn openai_catalog_has_gpt4o() {
        let p = build_openai_profile();
        let meta = p.model_meta("gpt-4o").unwrap();
        assert_eq!(meta.context_window, 128_000);
        assert!(meta.capabilities.tools);
        assert!(meta.capabilities.vision);
    }

    #[test]
    fn openai_alias_gpt4o_latest_resolves() {
        let p = build_openai_profile();
        assert_eq!(p.resolve_model_id("gpt-4o-latest"), "gpt-4o");
    }

    #[test]
    fn openai_mini_model_catalog() {
        let p = build_openai_profile();
        let meta = p.model_meta("gpt-4o-mini").unwrap();
        assert_eq!(meta.context_window, 128_000);
        // mini is cheaper
        let pricing = meta.pricing.as_ref().unwrap();
        assert!(pricing.input_per_million < 1.0);
    }

    #[test]
    fn openai_o1_catalog_no_vision() {
        let p = build_openai_profile();
        let meta = p.model_meta("o1").unwrap();
        assert!(!meta.capabilities.vision);
    }
}
