use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the ByteDance Doubao (Ark) provider profile.
///
/// Doubao is served via the Volcano Engine ARK platform with an OpenAI-compatible
/// API. Auth is read from `DOUBAO_API_KEY` (the ARK API key).
pub fn build_doubao_profile() -> ProviderProfile {
    ProviderProfile::new(
        "doubao",
        "https://ark.cn-beijing.volces.com/api/v3",
        AuthStrategy::BearerToken { env_var: "DOUBAO_API_KEY".to_owned() },
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
    .add_alias("pro-32k", "doubao-1.5-pro-32k")
    .add_alias("pro-256k", "doubao-1.5-pro-256k")
    .add_alias("lite", "doubao-1.5-lite-32k")
    .add_alias("vision", "doubao-1.5-vision-32k")
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
        let resp = doubao_client().parse_response(DOUBAO_FIXTURE, "doubao-1.5-pro-32k").unwrap();
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
}
