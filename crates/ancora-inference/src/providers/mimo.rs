use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build a Xiaomi MiMo profile for a custom-hosted endpoint (vLLM / Ollama / etc.).
///
/// MiMo is Xiaomi's open-source reasoning model family. There is no official
/// cloud endpoint; users deploy the weights themselves. Pass the base URL of
/// your vLLM or Ollama server and, optionally, set an API key via the env var
/// `MIMO_API_KEY` (leave it unset for no-auth local deployments by using
/// `build_mimo_noauth_profile`).
pub fn build_mimo_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "mimo",
        base_url,
        AuthStrategy::BearerToken { env_var: "MIMO_API_KEY".to_owned() },
    )
    // MiMo-7B-RL -- RL-tuned reasoning variant
    .add_model(
        ModelMeta::new("mimo-7b-rl", 32_768)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    // MiMo-7B -- base instruction-tuned model
    .add_model(
        ModelMeta::new("mimo-7b", 32_768)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    // MiMo-7B-RL-FC -- function-calling capable variant (served with fc lora adapter)
    .add_model(
        ModelMeta::new("mimo-7b-rl-fc", 32_768)
            .with_pricing(0.0, 0.0)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("rl", "mimo-7b-rl")
    .add_alias("base", "mimo-7b")
    .add_alias("fc", "mimo-7b-rl-fc")
}

/// Return true if the model id supports function calling in this profile.
pub fn supports_tools(model_id: &str) -> bool {
    matches!(model_id, "mimo-7b-rl-fc" | "fc")
}

/// Delegate SSE line parsing to the shared OpenAI-compatible parser.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Build a MiMo profile for a local no-auth endpoint (e.g., plain vLLM without token auth).
pub fn build_mimo_noauth_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "mimo-local",
        base_url,
        AuthStrategy::None,
    )
    .add_model(
        ModelMeta::new("mimo-7b-rl", 32_768)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("mimo-7b", 32_768)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    .add_alias("rl", "mimo-7b-rl")
    .add_alias("base", "mimo-7b")
}

/// Normalize a MiMo HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const MIMO_FIXTURE: &str = r#"{"id":"chatcmpl-mm-01","choices":[{"message":{"role":"assistant","content":"Hello from MiMo","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":5}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn mimo_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(
            build_mimo_profile("http://localhost:8000/v1"),
        ))
    }

    #[test]
    fn mimo_provider_name() {
        assert_eq!(build_mimo_profile("http://localhost:8000/v1").name, "mimo");
    }

    #[test]
    fn mimo_recorded_fixture_completes() {
        let resp = mimo_client().parse_response(MIMO_FIXTURE, "mimo-7b-rl").unwrap();
        assert_eq!(resp.content, "Hello from MiMo");
        assert_eq!(resp.tokens_in, 8);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn mimo_noauth_profile_name() {
        let p = build_mimo_noauth_profile("http://localhost:8000/v1");
        assert_eq!(p.name, "mimo-local");
    }

    #[test]
    fn mimo_fc_variant_has_tools() {
        let p = build_mimo_profile("http://localhost:8000/v1");
        assert!(p.model_meta("fc").unwrap().capabilities.tools);
    }

    #[test]
    fn mimo_rl_base_has_no_tools() {
        let p = build_mimo_profile("http://localhost:8000/v1");
        assert!(!p.model_meta("rl").unwrap().capabilities.tools);
    }

    #[test]
    fn mimo_supports_tools_helper() {
        assert!(supports_tools("mimo-7b-rl-fc"));
        assert!(!supports_tools("mimo-7b-rl"));
    }

    #[test]
    fn mimo_error_429_is_rate_limit() {
        use crate::error::InferenceError;
        let err = normalize_error(429, "rate limited");
        assert!(matches!(err, InferenceError::RateLimit { .. }));
    }
}
