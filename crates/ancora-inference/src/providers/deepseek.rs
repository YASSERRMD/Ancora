use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the DeepSeek provider profile.
///
/// DeepSeek exposes an OpenAI-compatible API at `api.deepseek.com`.
/// Use with `OpenAiClient`. Reads the API key from `DEEPSEEK_API_KEY`.
///
/// Note: the direct endpoint routes through DeepSeek's CN-region
/// infrastructure. Use the self-host profile for non-CN residency.
pub fn build_deepseek_profile() -> ProviderProfile {
    ProviderProfile::new(
        "deepseek",
        "https://api.deepseek.com",
        AuthStrategy::BearerToken { env_var: "DEEPSEEK_API_KEY".to_owned() },
    )
    // DeepSeek V3 (general-purpose chat)
    // Cache-hit pricing: $0.07/M (74% discount vs full $0.27/M input)
    .add_model(
        ModelMeta::new("deepseek-chat", 64_000)
            .with_pricing(0.27, 1.10)
            .with_cached_pricing(0.07)
            .with_tools()
            .with_streaming(),
    )
    // DeepSeek R1 (reasoning model)
    // Cache-hit pricing: $0.14/M (75% discount vs full $0.55/M input)
    .add_model(
        ModelMeta::new("deepseek-reasoner", 64_000)
            .with_pricing(0.55, 2.19)
            .with_cached_pricing(0.14)
            .with_streaming(),
    )
    // DeepSeek Coder (long-context, code-optimized)
    // Cache-hit pricing: $0.035/M
    .add_model(
        ModelMeta::new("deepseek-coder", 128_000)
            .with_pricing(0.14, 0.28)
            .with_cached_pricing(0.035)
            .with_tools()
            .with_streaming(),
    )
    // Aliases
    .add_alias("deepseek-v3", "deepseek-chat")
    .add_alias("deepseek-r1", "deepseek-reasoner")
    .add_alias("v3", "deepseek-chat")
    .add_alias("r1", "deepseek-reasoner")
    .add_alias("coder", "deepseek-coder")
}

/// Build a self-hosted DeepSeek profile for vLLM or other OpenAI-compatible servers.
///
/// Used when data must not leave a specific region. The base URL is read from
/// `DEEPSEEK_BASE_URL` (e.g. `http://localhost:8000`). Auth is disabled by
/// default; set `DEEPSEEK_SELF_HOST_KEY` if the server requires a token.
pub fn build_deepseek_self_host_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "deepseek-self-host",
        base_url,
        AuthStrategy::BearerToken { env_var: "DEEPSEEK_SELF_HOST_KEY".to_owned() },
    )
    .add_model(
        ModelMeta::new("deepseek-chat", 64_000)
            .with_pricing(0.0, 0.0)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("deepseek-reasoner", 64_000)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    .add_alias("deepseek-v3", "deepseek-chat")
    .add_alias("deepseek-r1", "deepseek-reasoner")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deepseek_provider_name_is_deepseek() {
        assert_eq!(build_deepseek_profile().name, "deepseek");
    }

    #[test]
    fn deepseek_base_url_is_correct() {
        assert_eq!(build_deepseek_profile().base_url, "https://api.deepseek.com");
    }

    #[test]
    fn deepseek_v3_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("deepseek-v3"), "deepseek-chat");
    }

    #[test]
    fn deepseek_r1_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("deepseek-r1"), "deepseek-reasoner");
    }

    #[test]
    fn deepseek_short_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("v3"), "deepseek-chat");
        assert_eq!(p.resolve_model_id("r1"), "deepseek-reasoner");
    }

    #[test]
    fn deepseek_chat_has_tools() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        assert!(m.capabilities.tools);
    }

    #[test]
    fn deepseek_reasoner_has_no_tools() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-reasoner").unwrap();
        assert!(!m.capabilities.tools);
    }

    #[test]
    fn deepseek_chat_has_cached_pricing() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        assert!(pricing.cached_per_million.is_some());
    }

    #[test]
    fn deepseek_r1_has_cached_pricing() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-reasoner").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        assert!(pricing.cached_per_million.is_some());
    }

    #[test]
    fn deepseek_cached_price_lower_than_full() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        let cached = pricing.cached_per_million.unwrap();
        assert!(cached < pricing.input_per_million);
    }

    #[test]
    fn deepseek_chat_context_window() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        assert_eq!(m.context_window, 64_000);
    }

    #[test]
    fn deepseek_reasoner_context_window() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-reasoner").unwrap();
        assert_eq!(m.context_window, 64_000);
    }

    #[test]
    fn deepseek_coder_larger_context() {
        let p = build_deepseek_profile();
        let coder = p.model_meta("deepseek-coder").unwrap();
        let chat = p.model_meta("deepseek-chat").unwrap();
        assert!(coder.context_window >= chat.context_window);
    }

    #[test]
    fn deepseek_coder_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("coder"), "deepseek-coder");
    }

    #[test]
    fn deepseek_all_models_stream() {
        let p = build_deepseek_profile();
        for (id, m) in &p.model_catalog {
            assert!(m.capabilities.streaming, "{id} must stream");
        }
    }
}
