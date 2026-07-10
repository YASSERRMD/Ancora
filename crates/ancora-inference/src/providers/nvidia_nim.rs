use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the NVIDIA NIM (hosted) provider profile.
///
/// NIM exposes 100+ models through one OpenAI-compatible endpoint at
/// `https://integrate.api.nvidia.com/v1`. Auth is a bearer token read from
/// `NVIDIA_API_KEY` (an `nvapi-...` key). The base URL already carries the
/// `/v1` segment NVIDIA documents, so the completions path is overridden to
/// `/chat/completions` rather than the OpenAI-default `/v1/chat/completions`
/// (which would otherwise double up to `/v1/v1/chat/completions`).
///
/// Pricing is intentionally omitted from the model catalog: NVIDIA does not
/// publish stable per-token pricing for the hosted NIM endpoint (preview
/// access is free; production access is contract-based), so `cost_usd` on
/// responses from these models is `None` rather than a fabricated number.
pub fn build_nvidia_nim_profile() -> ProviderProfile {
    ProviderProfile::new(
        "nvidia_nim",
        "https://integrate.api.nvidia.com/v1",
        AuthStrategy::BearerToken {
            env_var: "NVIDIA_API_KEY".to_owned(),
        },
    )
    .with_chat_path("/chat/completions")
    .add_model(
        ModelMeta::new("meta/llama-3.1-8b-instruct", 128_000)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("meta/llama-3.1-70b-instruct", 128_000)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("meta/llama-3.1-405b-instruct", 128_000)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("mistralai/mixtral-8x7b-instruct-v0.1", 32_768)
            .with_tools()
            .with_streaming(),
    )
    .add_model(ModelMeta::new("nvidia/nemotron-4-340b-instruct", 4_096).with_streaming())
    .add_alias("llama-3.1-8b", "meta/llama-3.1-8b-instruct")
    .add_alias("llama-3.1-70b", "meta/llama-3.1-70b-instruct")
    .add_alias("llama-3.1-405b", "meta/llama-3.1-405b-instruct")
    .add_alias("mixtral-8x7b", "mistralai/mixtral-8x7b-instruct-v0.1")
    .add_alias("nemotron-4-340b", "nvidia/nemotron-4-340b-instruct")
}

/// Build a self-hosted NIM container profile (identical API, base URL only
/// change, e.g. `http://localhost:8000/v1`).
///
/// Self-hosted NIM containers typically run without an API key by default,
/// so auth defaults to `None`. Callers that front their container with an
/// API key can override `profile.auth` directly (its field is public).
pub fn build_nvidia_nim_self_host_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new("nvidia_nim-self-host", base_url, AuthStrategy::None)
        .with_chat_path("/chat/completions")
}

/// Delegate SSE line parsing to the shared OpenAI-compatible parser.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Normalize a NIM HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const NIM_FIXTURE: &str = r#"{"id":"chatcmpl-nim-01","choices":[{"message":{"role":"assistant","content":"Hello from NIM","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":4}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn nim_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_nvidia_nim_profile()))
    }

    #[test]
    fn nvidia_nim_provider_name() {
        assert_eq!(build_nvidia_nim_profile().name, "nvidia_nim");
    }

    #[test]
    fn nvidia_nim_base_url_carries_v1() {
        assert_eq!(
            build_nvidia_nim_profile().base_url,
            "https://integrate.api.nvidia.com/v1"
        );
    }

    #[test]
    fn nvidia_nim_completions_url_does_not_double_v1() {
        let p = build_nvidia_nim_profile();
        assert_eq!(
            p.completions_url(None),
            "https://integrate.api.nvidia.com/v1/chat/completions"
        );
    }

    #[test]
    fn nvidia_nim_auth_reads_nvidia_api_key_env_var() {
        let p = build_nvidia_nim_profile();
        match p.auth {
            AuthStrategy::BearerToken { ref env_var } => assert_eq!(env_var, "NVIDIA_API_KEY"),
            _ => panic!("expected BearerToken auth"),
        }
    }

    #[test]
    fn nvidia_nim_recorded_fixture_completes() {
        let resp = nim_client()
            .parse_response(NIM_FIXTURE, "meta/llama-3.1-8b-instruct")
            .unwrap();
        assert_eq!(resp.content, "Hello from NIM");
        assert_eq!(resp.tokens_in, 9);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn nvidia_nim_pricing_is_deliberately_absent() {
        let p = build_nvidia_nim_profile();
        let meta = p.model_meta("llama-3.1-8b").unwrap();
        assert!(meta.pricing.is_none());
    }

    #[test]
    fn nvidia_nim_alias_resolves_to_canonical_model_id() {
        let p = build_nvidia_nim_profile();
        assert_eq!(
            p.resolve_model_id("llama-3.1-70b"),
            "meta/llama-3.1-70b-instruct"
        );
    }

    #[test]
    fn nvidia_nim_405b_fits_large_context() {
        let p = build_nvidia_nim_profile();
        let meta = p.model_meta("llama-3.1-405b").unwrap();
        assert!(meta.fits_context(100_000));
    }

    #[test]
    fn nvidia_nim_tools_flag_set_on_llama_models() {
        let p = build_nvidia_nim_profile();
        assert!(p.model_meta("llama-3.1-8b").unwrap().capabilities.tools);
    }

    #[test]
    fn nvidia_nim_self_host_profile_name() {
        let p = build_nvidia_nim_self_host_profile("http://localhost:8000/v1");
        assert_eq!(p.name, "nvidia_nim-self-host");
    }

    #[test]
    fn nvidia_nim_self_host_defaults_to_no_auth() {
        let p = build_nvidia_nim_self_host_profile("http://localhost:8000/v1");
        assert!(matches!(p.auth, AuthStrategy::None));
    }

    #[test]
    fn nvidia_nim_self_host_url_switch_is_base_url_only() {
        let hosted = build_nvidia_nim_profile();
        let self_hosted = build_nvidia_nim_self_host_profile("http://localhost:8000/v1");
        assert_eq!(
            hosted.chat_completions_path,
            self_hosted.chat_completions_path
        );
        assert_eq!(
            self_hosted.completions_url(None),
            "http://localhost:8000/v1/chat/completions"
        );
    }

    #[test]
    fn nvidia_nim_error_429_is_rate_limit() {
        use crate::error::InferenceError;
        let err = normalize_error(429, "rate limited");
        assert!(matches!(err, InferenceError::RateLimit { .. }));
    }

    #[test]
    fn nvidia_nim_error_401_is_auth_rejected() {
        use crate::error::InferenceError;
        let err = normalize_error(401, "bad key");
        assert!(matches!(err, InferenceError::AuthRejected(_)));
    }

    #[test]
    fn nvidia_nim_parse_sse_done_signals_stream_end() {
        let result = parse_stream_line("data: [DONE]");
        assert!(result.map(|e| e.finished).unwrap_or(false));
    }

    #[test]
    fn nvidia_nim_parse_sse_token_returns_event() {
        let line = r#"data: {"choices":[{"delta":{"content":"hi"},"finish_reason":null}]}"#;
        let event = parse_stream_line(line);
        assert!(event.is_some());
    }
}
