use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Anthropic provider profile.
///
/// Reads the API key from `ANTHROPIC_API_KEY` at call time.
/// The `anthropic-version` header is pinned to `2023-06-01` via `extra_headers`.
pub fn build_anthropic_profile() -> ProviderProfile {
    ProviderProfile::new(
        "anthropic",
        "https://api.anthropic.com",
        AuthStrategy::HeaderKey {
            header: "x-api-key".to_owned(),
            env_var: "ANTHROPIC_API_KEY".to_owned(),
        },
    )
    .with_chat_path("/v1/messages")
    .with_extra_header("anthropic-version", "2023-06-01")
    .add_model(
        ModelMeta::new("claude-opus-4-8", 200_000)
            .with_pricing(15.0, 75.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("claude-sonnet-4-6", 200_000)
            .with_pricing(3.0, 15.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("claude-haiku-4-5", 200_000)
            .with_pricing(0.80, 4.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_alias("claude-3-5-sonnet", "claude-sonnet-4-6")
    .add_alias("claude-3-5-haiku", "claude-haiku-4-5")
    .add_alias("claude-opus", "claude-opus-4-8")
}

#[cfg(test)]
const FIXTURE_CHAT: &str = r#"{"id":"msg_01","type":"message","role":"assistant","content":[{"type":"text","text":"Hello from Anthropic"}],"model":"claude-sonnet-4-6","stop_reason":"end_turn","usage":{"input_tokens":12,"output_tokens":4}}"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::anthropic::AnthropicClient;
    use std::sync::Arc;

    fn client() -> AnthropicClient {
        AnthropicClient::new(Arc::new(build_anthropic_profile()))
    }

    #[test]
    fn anthropic_recorded_fixture_completes() {
        let resp = client().parse_response(FIXTURE_CHAT, "claude-sonnet-4-6").unwrap();
        assert_eq!(resp.content, "Hello from Anthropic");
        assert_eq!(resp.tokens_in, 12);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn anthropic_cost_computed_from_profile_pricing() {
        let resp = client().parse_response(FIXTURE_CHAT, "claude-sonnet-4-6").unwrap();
        // 12 * $3.0/M + 4 * $15.0/M
        let expected = 12.0 * 3.0 / 1_000_000.0 + 4.0 * 15.0 / 1_000_000.0;
        let cost = resp.cost_usd.expect("cost must be Some for priced model");
        assert!((cost - expected).abs() < 1e-12);
    }

    #[test]
    fn anthropic_profile_base_url_correct() {
        let p = build_anthropic_profile();
        assert_eq!(p.base_url, "https://api.anthropic.com");
    }

    #[test]
    fn anthropic_profile_version_header_present() {
        let p = build_anthropic_profile();
        assert_eq!(
            p.extra_headers.get("anthropic-version").map(|s| s.as_str()),
            Some("2023-06-01")
        );
    }

    #[test]
    fn anthropic_profile_has_opus_model() {
        let p = build_anthropic_profile();
        let meta = p.model_meta("claude-opus-4-8").unwrap();
        assert_eq!(meta.context_window, 200_000);
        assert!(meta.capabilities.tools);
        assert!(meta.capabilities.vision);
    }

    #[test]
    fn anthropic_alias_claude_opus_resolves() {
        let p = build_anthropic_profile();
        assert_eq!(p.resolve_model_id("claude-opus"), "claude-opus-4-8");
    }
}
