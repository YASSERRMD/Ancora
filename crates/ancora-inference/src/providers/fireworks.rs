use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Fireworks AI provider profile.
///
/// Fireworks AI provides an OpenAI-compatible inference endpoint optimized
/// for open-source models. Use with `OpenAiClient`.
/// Reads the API key from `FIREWORKS_API_KEY` at call time.
pub fn build_fireworks_profile() -> ProviderProfile {
    ProviderProfile::new(
        "fireworks",
        "https://api.fireworks.ai/inference",
        AuthStrategy::BearerToken { env_var: "FIREWORKS_API_KEY".to_owned() },
    )
    // Llama 3.1 family
    .add_model(
        ModelMeta::new("accounts/fireworks/models/llama-v3p1-70b-instruct", 131_072)
            .with_pricing(0.90, 0.90)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("accounts/fireworks/models/llama-v3p1-8b-instruct", 131_072)
            .with_pricing(0.20, 0.20)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("accounts/fireworks/models/llama-v3p1-405b-instruct", 131_072)
            .with_pricing(3.00, 3.00)
            .with_tools()
            .with_streaming(),
    )
    // Mixtral
    .add_model(
        ModelMeta::new("accounts/fireworks/models/mixtral-8x22b-instruct", 65_536)
            .with_pricing(1.20, 1.20)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("accounts/fireworks/models/mixtral-8x7b-instruct", 32_768)
            .with_pricing(0.50, 0.50)
            .with_streaming(),
    )
    // Qwen
    .add_model(
        ModelMeta::new("accounts/fireworks/models/qwen2p5-72b-instruct", 32_768)
            .with_pricing(0.90, 0.90)
            .with_tools()
            .with_streaming(),
    )
    // Aliases
    .add_alias("llama3.1-70b", "accounts/fireworks/models/llama-v3p1-70b-instruct")
    .add_alias("llama3.1-8b", "accounts/fireworks/models/llama-v3p1-8b-instruct")
    .add_alias("llama3.1-405b", "accounts/fireworks/models/llama-v3p1-405b-instruct")
    .add_alias("mixtral-8x22b", "accounts/fireworks/models/mixtral-8x22b-instruct")
    .add_alias("mixtral", "accounts/fireworks/models/mixtral-8x7b-instruct")
    .add_alias("qwen2.5-72b", "accounts/fireworks/models/qwen2p5-72b-instruct")
}

#[cfg(test)]
const FIREWORKS_FIXTURE: &str = r#"{"id":"chatcmpl-fw-01","choices":[{"message":{"role":"assistant","content":"Hello from Fireworks","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":4}}"#;

#[cfg(test)]
const FIREWORKS_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" Fireworks"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fireworks_provider_name_is_fireworks() {
        assert_eq!(build_fireworks_profile().name, "fireworks");
    }

    #[test]
    fn fireworks_base_url_is_correct() {
        assert_eq!(
            build_fireworks_profile().base_url,
            "https://api.fireworks.ai/inference"
        );
    }

    #[test]
    fn fireworks_llama_70b_alias_resolves() {
        let p = build_fireworks_profile();
        assert_eq!(
            p.resolve_model_id("llama3.1-70b"),
            "accounts/fireworks/models/llama-v3p1-70b-instruct"
        );
    }

    #[test]
    fn fireworks_mixtral_alias_resolves() {
        let p = build_fireworks_profile();
        assert_eq!(
            p.resolve_model_id("mixtral"),
            "accounts/fireworks/models/mixtral-8x7b-instruct"
        );
    }

    #[test]
    fn fireworks_llama_70b_has_tools() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-70b-instruct").unwrap();
        assert!(m.capabilities.tools);
    }

    #[test]
    fn fireworks_llama_8b_has_no_tools() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-8b-instruct").unwrap();
        assert!(!m.capabilities.tools);
    }

    #[test]
    fn fireworks_all_models_stream() {
        let p = build_fireworks_profile();
        let ids = [
            "accounts/fireworks/models/llama-v3p1-70b-instruct",
            "accounts/fireworks/models/llama-v3p1-8b-instruct",
            "accounts/fireworks/models/llama-v3p1-405b-instruct",
            "accounts/fireworks/models/mixtral-8x22b-instruct",
            "accounts/fireworks/models/mixtral-8x7b-instruct",
            "accounts/fireworks/models/qwen2p5-72b-instruct",
        ];
        for id in ids {
            let m = p.model_meta(id).unwrap_or_else(|| panic!("{id} missing"));
            assert!(m.capabilities.streaming, "{id} must stream");
        }
    }

    #[test]
    fn fireworks_llama_large_context_window() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/llama-v3p1-405b-instruct").unwrap();
        assert_eq!(m.context_window, 131_072);
    }

    #[test]
    fn fireworks_qwen_has_tools() {
        let p = build_fireworks_profile();
        let m = p.model_meta("accounts/fireworks/models/qwen2p5-72b-instruct").unwrap();
        assert!(m.capabilities.tools);
    }

    #[test]
    fn fireworks_405b_alias_resolves() {
        let p = build_fireworks_profile();
        assert_eq!(
            p.resolve_model_id("llama3.1-405b"),
            "accounts/fireworks/models/llama-v3p1-405b-instruct"
        );
    }
}
