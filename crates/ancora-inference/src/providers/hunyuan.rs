use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Tencent Hunyuan provider profile.
///
/// Hunyuan exposes an OpenAI-compatible API. Auth is read from `HUNYUAN_API_KEY`.
pub fn build_hunyuan_profile() -> ProviderProfile {
    ProviderProfile::new(
        "hunyuan",
        "https://api.hunyuan.cloud.tencent.com/v1",
        AuthStrategy::BearerToken { env_var: "HUNYUAN_API_KEY".to_owned() },
    )
    // Hunyuan Turbo -- fastest, large context
    .add_model(
        ModelMeta::new("hunyuan-turbo", 131_072)
            .with_pricing(0.15, 0.50)
            .with_tools()
            .with_streaming(),
    )
    // Hunyuan Pro -- high capability
    .add_model(
        ModelMeta::new("hunyuan-pro", 32_768)
            .with_pricing(0.45, 1.50)
            .with_tools()
            .with_streaming(),
    )
    // Hunyuan Standard -- balanced
    .add_model(
        ModelMeta::new("hunyuan-standard", 32_768)
            .with_pricing(0.05, 0.05)
            .with_streaming(),
    )
    // Hunyuan Vision -- multimodal
    .add_model(
        ModelMeta::new("hunyuan-vision", 8_192)
            .with_pricing(0.18, 0.18)
            .with_vision()
            .with_streaming(),
    )
    // Hunyuan Lite -- free tier
    .add_model(
        ModelMeta::new("hunyuan-lite", 256_000)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    .add_alias("turbo", "hunyuan-turbo")
    .add_alias("pro", "hunyuan-pro")
    .add_alias("standard", "hunyuan-standard")
    .add_alias("vision", "hunyuan-vision")
    .add_alias("lite", "hunyuan-lite")
}

/// Normalize a Tencent Hunyuan HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const HUNYUAN_FIXTURE: &str = r#"{"id":"chatcmpl-hy-01","choices":[{"message":{"role":"assistant","content":"Hello from Hunyuan","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":11,"completion_tokens":5}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn hy_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_hunyuan_profile()))
    }

    #[test]
    fn hunyuan_provider_name() {
        assert_eq!(build_hunyuan_profile().name, "hunyuan");
    }

    #[test]
    fn hunyuan_recorded_fixture_completes() {
        let resp = hy_client().parse_response(HUNYUAN_FIXTURE, "hunyuan-turbo").unwrap();
        assert_eq!(resp.content, "Hello from Hunyuan");
        assert_eq!(resp.tokens_in, 11);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn hunyuan_turbo_has_tools() {
        let p = build_hunyuan_profile();
        assert!(p.model_meta("hunyuan-turbo").unwrap().capabilities.tools);
    }

    #[test]
    fn hunyuan_vision_has_vision_flag() {
        let p = build_hunyuan_profile();
        assert!(p.model_meta("vision").unwrap().capabilities.vision);
    }
}
