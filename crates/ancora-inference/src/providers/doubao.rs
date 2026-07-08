use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the ByteDance Doubao (Ark) provider profile.
///
/// Doubao is served via the Volcano Engine ARK platform with an OpenAI-compatible
/// API. Auth is read from `DOUBAO_API_KEY` (the ARK API key).
pub fn build_doubao_profile() -> ProviderProfile {
    ProviderProfile::new(
        "doubao",
        "https://ark.cn-beijing.volces.com/api/v3",
        AuthStrategy::BearerToken {
            env_var: "DOUBAO_API_KEY".to_owned(),
        },
    )
    // Doubao 1.5 Pro 32k -- production-grade, tools
    .add_model(
        ModelMeta::new("doubao-1.5-pro-32k", 32_768)
            .with_pricing(0.04, 0.08)
            .with_tools()
            .with_streaming(),
    )
    // Doubao 1.5 Pro 256k -- very long context
    .add_model(
        ModelMeta::new("doubao-1.5-pro-256k", 262_144)
            .with_pricing(0.11, 0.22)
            .with_tools()
            .with_streaming(),
    )
    // Doubao 1.5 Lite -- cheap and fast
    .add_model(
        ModelMeta::new("doubao-1.5-lite-32k", 32_768)
            .with_pricing(0.01, 0.03)
            .with_streaming(),
    )
    // Doubao Vision -- multimodal
    .add_model(
        ModelMeta::new("doubao-1.5-vision-32k", 32_768)
            .with_pricing(0.08, 0.08)
            .with_vision()
            .with_streaming(),
    )
    // Doubao 1.5 Thinking -- chain-of-thought reasoning
    .add_model(
        ModelMeta::new("doubao-1.5-thinking-32k", 32_768)
            .with_pricing(0.06, 0.12)
            .with_streaming(),
    )
    // Doubao Character -- long-context roleplay / persona
    .add_model(
        ModelMeta::new("doubao-character-128k", 131_072)
            .with_pricing(0.05, 0.10)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("pro-32k", "doubao-1.5-pro-32k")
    .add_alias("pro-256k", "doubao-1.5-pro-256k")
    .add_alias("lite", "doubao-1.5-lite-32k")
    .add_alias("vision", "doubao-1.5-vision-32k")
    .add_alias("thinking", "doubao-1.5-thinking-32k")
    .add_alias("character", "doubao-character-128k")
}

/// Build a self-hosted Doubao-compatible profile (e.g., vLLM-served Doubao weights).
pub fn build_doubao_self_host_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "doubao-self-host",
        base_url,
        AuthStrategy::BearerToken {
            env_var: "DOUBAO_SELF_HOST_API_KEY".to_owned(),
        },
    )
}

/// Return true if the model supports vision input.
pub fn supports_vision(model_id: &str) -> bool {
    matches!(model_id, "doubao-1.5-vision-32k" | "vision")
}

/// Delegate SSE line parsing to the shared OpenAI-compatible parser.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Normalize a Doubao HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const DOUBAO_FIXTURE: &str = r#"{"id":"chatcmpl-db-01","choices":[{"message":{"role":"assistant","content":"Hello from Doubao","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":12,"completion_tokens":5}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn doubao_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_doubao_profile()))
    }

    #[test]
    fn doubao_provider_name() {
        assert_eq!(build_doubao_profile().name, "doubao");
    }

    #[test]
    fn doubao_recorded_fixture_completes() {
        let resp = doubao_client()
            .parse_response(DOUBAO_FIXTURE, "doubao-1.5-pro-32k")
            .unwrap();
        assert_eq!(resp.content, "Hello from Doubao");
        assert_eq!(resp.tokens_in, 12);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn doubao_pro_256k_fits_large_context() {
        let p = build_doubao_profile();
        let meta = p.model_meta("pro-256k").unwrap();
        assert!(meta.fits_context(200_000));
    }

    #[test]
    fn doubao_vision_has_vision_flag() {
        let p = build_doubao_profile();
        assert!(p.model_meta("vision").unwrap().capabilities.vision);
    }

    #[test]
    fn doubao_thinking_has_no_tools_flag() {
        let p = build_doubao_profile();
        assert!(!p.model_meta("thinking").unwrap().capabilities.tools);
    }

    #[test]
    fn doubao_character_fits_128k_context() {
        let p = build_doubao_profile();
        let meta = p.model_meta("character").unwrap();
        assert!(meta.fits_context(100_000));
    }

    #[test]
    fn doubao_self_host_profile_name() {
        let p = build_doubao_self_host_profile("http://localhost:8000");
        assert_eq!(p.name, "doubao-self-host");
    }

    #[test]
    fn doubao_error_429_is_rate_limit() {
        use crate::error::InferenceError;
        let err = normalize_error(429, "rate limited");
        assert!(matches!(err, InferenceError::RateLimit { .. }));
    }

    #[test]
    fn doubao_supports_vision_helper() {
        assert!(supports_vision("doubao-1.5-vision-32k"));
        assert!(supports_vision("vision"));
        assert!(!supports_vision("doubao-1.5-pro-32k"));
    }

    #[test]
    fn doubao_parse_sse_done_signals_stream_end() {
        let result = parse_stream_line("data: [DONE]");
        assert!(result.map(|e| e.finished).unwrap_or(false));
    }

    #[test]
    fn doubao_parse_sse_token_returns_event() {
        let line = r#"data: {"choices":[{"delta":{"content":"hi"},"finish_reason":null}]}"#;
        let event = parse_stream_line(line);
        assert!(event.is_some());
    }
}
