use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Default local LiteLLM proxy URL.
pub const LITELLM_DEFAULT_URL: &str = "http://localhost:4000";

/// Build a LiteLLM proxy profile pointing at a custom base URL.
///
/// LiteLLM is an open-source proxy that exposes an OpenAI-compatible API and
/// routes to 100+ providers. Auth is controlled by the LiteLLM master key;
/// leave `LITELLM_API_KEY` unset for unauthenticated local deployments (use
/// `build_litellm_noauth_profile` instead).
pub fn build_litellm_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "litellm",
        base_url,
        AuthStrategy::BearerToken { env_var: "LITELLM_API_KEY".to_owned() },
    )
    // Pre-registered virtual model IDs for common routing targets.
    .add_model(ModelMeta::new("openai/gpt-4o", 128_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("openai/gpt-4o-mini", 128_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("anthropic/claude-3-5-haiku", 200_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("anthropic/claude-3-7-sonnet", 200_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("gemini/gemini-2.0-flash", 1_048_576).with_tools().with_vision().with_streaming())
    .add_alias("gpt-4o", "openai/gpt-4o")
    .add_alias("gpt-4o-mini", "openai/gpt-4o-mini")
    .add_alias("haiku", "anthropic/claude-3-5-haiku")
    .add_alias("sonnet", "anthropic/claude-3-7-sonnet")
    .add_alias("gemini-flash", "gemini/gemini-2.0-flash")
}

/// Build a LiteLLM profile with no authentication (local dev / trusted network).
pub fn build_litellm_noauth_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "litellm-local",
        base_url,
        AuthStrategy::None,
    )
    .add_model(ModelMeta::new("openai/gpt-4o", 128_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("anthropic/claude-3-5-haiku", 200_000).with_tools().with_vision().with_streaming())
    .add_alias("gpt-4o", "openai/gpt-4o")
    .add_alias("haiku", "anthropic/claude-3-5-haiku")
}

/// Return true if the model ID uses LiteLLM's `provider/model` routing format.
pub fn is_routed_model_id(model_id: &str) -> bool {
    model_id.contains('/')
}

/// Normalize a LiteLLM HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const LITELLM_FIXTURE: &str = r#"{"id":"chatcmpl-ll-01","choices":[{"message":{"role":"assistant","content":"Hello from LiteLLM","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":6}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn llm_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(
            build_litellm_profile(LITELLM_DEFAULT_URL),
        ))
    }

    #[test]
    fn litellm_provider_name() {
        assert_eq!(build_litellm_profile("http://localhost:4000").name, "litellm");
    }

    #[test]
    fn litellm_recorded_fixture_completes() {
        let resp = llm_client().parse_response(LITELLM_FIXTURE, "openai/gpt-4o").unwrap();
        assert_eq!(resp.content, "Hello from LiteLLM");
        assert_eq!(resp.tokens_in, 10);
        assert_eq!(resp.tokens_out, 6);
    }

    #[test]
    fn litellm_noauth_profile_name() {
        assert_eq!(build_litellm_noauth_profile("http://localhost:4000").name, "litellm-local");
    }

    #[test]
    fn litellm_is_routed_model_id() {
        assert!(is_routed_model_id("openai/gpt-4o"));
        assert!(!is_routed_model_id("gpt-4o"));
    }
}
