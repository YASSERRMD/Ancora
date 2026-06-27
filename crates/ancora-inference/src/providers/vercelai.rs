use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Vercel AI Gateway base URL.
pub const VERCEL_AI_GATEWAY_URL: &str = "https://gateway.ai.vercel.app/v1";

/// Build a Vercel AI Gateway provider profile.
///
/// The Vercel AI Gateway is a managed routing layer that dispatches to
/// multiple AI providers (OpenAI, Anthropic, Mistral, etc.) using a unified
/// endpoint. Model IDs follow the `provider/model` format (e.g.
/// `"openai/gpt-4o"`, `"anthropic/claude-3-5-haiku-20241022"`). Auth is read
/// from `VERCEL_AI_TOKEN`.
pub fn build_vercelai_profile() -> ProviderProfile {
    ProviderProfile::new(
        "vercelai",
        VERCEL_AI_GATEWAY_URL,
        AuthStrategy::BearerToken { env_var: "VERCEL_AI_TOKEN".to_owned() },
    )
    .add_model(ModelMeta::new("openai/gpt-4o", 128_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("openai/gpt-4o-mini", 128_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("anthropic/claude-3-5-haiku-20241022", 200_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("anthropic/claude-3-7-sonnet-20250219", 200_000).with_tools().with_vision().with_streaming())
    .add_model(ModelMeta::new("mistral/mistral-large-latest", 131_072).with_tools().with_streaming())
    .add_alias("gpt-4o", "openai/gpt-4o")
    .add_alias("gpt-4o-mini", "openai/gpt-4o-mini")
    .add_alias("haiku", "anthropic/claude-3-5-haiku-20241022")
    .add_alias("sonnet", "anthropic/claude-3-7-sonnet-20250219")
    .add_alias("mistral-large", "mistral/mistral-large-latest")
}

/// Build a Vercel AI Gateway profile pointing at a custom endpoint.
///
/// Use this when self-hosting a Vercel-compatible gateway or testing against
/// a local mock.
pub fn build_vercelai_custom_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "vercelai",
        base_url,
        AuthStrategy::BearerToken { env_var: "VERCEL_AI_TOKEN".to_owned() },
    )
    .add_model(ModelMeta::new("openai/gpt-4o", 128_000).with_tools().with_vision().with_streaming())
    .add_alias("gpt-4o", "openai/gpt-4o")
}

/// Return true if the model ID uses the gateway routing format (`provider/model`).
pub fn is_gateway_model_id(model_id: &str) -> bool {
    model_id.contains('/')
}

/// Extract the upstream provider name from a `provider/model` ID.
///
/// Returns `None` for bare model IDs that do not include a provider prefix.
pub fn extract_provider(model_id: &str) -> Option<&str> {
    model_id.split_once('/').map(|(provider, _)| provider)
}

/// Delegate SSE line parsing to the shared OpenAI-compatible parser.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Normalize a Vercel AI Gateway HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const VERCELAI_FIXTURE: &str = r#"{"id":"chatcmpl-vai-01","choices":[{"message":{"role":"assistant","content":"Hello from Vercel AI","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":5}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn vai_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_vercelai_profile()))
    }

    #[test]
    fn vercelai_provider_name() {
        assert_eq!(build_vercelai_profile().name, "vercelai");
    }

    #[test]
    fn vercelai_recorded_fixture_completes() {
        let resp = vai_client().parse_response(VERCELAI_FIXTURE, "openai/gpt-4o").unwrap();
        assert_eq!(resp.content, "Hello from Vercel AI");
        assert_eq!(resp.tokens_in, 9);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn vercelai_base_url() {
        let p = build_vercelai_profile();
        assert_eq!(p.base_url, VERCEL_AI_GATEWAY_URL);
    }

    #[test]
    fn vercelai_haiku_alias_resolves() {
        let p = build_vercelai_profile();
        assert_eq!(
            p.resolve_model_id("haiku"),
            Some("anthropic/claude-3-5-haiku-20241022".to_owned())
        );
    }

    #[test]
    fn vercelai_is_gateway_model_id() {
        assert!(is_gateway_model_id("openai/gpt-4o"));
        assert!(!is_gateway_model_id("gpt-4o"));
    }

    #[test]
    fn vercelai_error_429_is_rate_limit() {
        use crate::error::InferenceError;
        let err = normalize_error(429, "rate limited");
        assert!(matches!(err, InferenceError::RateLimit { .. }));
    }
}
