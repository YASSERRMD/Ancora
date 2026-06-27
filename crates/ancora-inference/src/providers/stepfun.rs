use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the StepFun (Step AI) provider profile.
///
/// StepFun exposes an OpenAI-compatible API. Auth is read from `STEPFUN_API_KEY`.
pub fn build_stepfun_profile() -> ProviderProfile {
    ProviderProfile::new(
        "stepfun",
        "https://api.stepfun.com/v1",
        AuthStrategy::BearerToken { env_var: "STEPFUN_API_KEY".to_owned() },
    )
    // Step-1 256k -- very long context, flagship
    .add_model(
        ModelMeta::new("step-1-256k", 262_144)
            .with_pricing(0.45, 0.45)
            .with_streaming(),
    )
    // Step-1 128k
    .add_model(
        ModelMeta::new("step-1-128k", 131_072)
            .with_pricing(0.20, 0.20)
            .with_tools()
            .with_streaming(),
    )
    // Step-1 32k
    .add_model(
        ModelMeta::new("step-1-32k", 32_768)
            .with_pricing(0.07, 0.07)
            .with_tools()
            .with_streaming(),
    )
    // Step-1V -- vision model
    .add_model(
        ModelMeta::new("step-1v-8k", 8_192)
            .with_pricing(0.10, 0.10)
            .with_vision()
            .with_streaming(),
    )
    .add_alias("step-256k", "step-1-256k")
    .add_alias("step-128k", "step-1-128k")
    .add_alias("step-32k", "step-1-32k")
    .add_alias("step-v", "step-1v-8k")
}

/// Normalize a StepFun HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const STEPFUN_FIXTURE: &str = r#"{"id":"chatcmpl-sf-01","choices":[{"message":{"role":"assistant","content":"Hello from StepFun","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":6}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn sf_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_stepfun_profile()))
    }

    #[test]
    fn stepfun_provider_name() {
        assert_eq!(build_stepfun_profile().name, "stepfun");
    }

    #[test]
    fn stepfun_recorded_fixture_completes() {
        let resp = sf_client().parse_response(STEPFUN_FIXTURE, "step-1-128k").unwrap();
        assert_eq!(resp.content, "Hello from StepFun");
        assert_eq!(resp.tokens_in, 10);
        assert_eq!(resp.tokens_out, 6);
    }

    #[test]
    fn stepfun_256k_has_long_context() {
        let p = build_stepfun_profile();
        let meta = p.model_meta("step-1-256k").unwrap();
        assert!(meta.context_window >= 262_000);
    }

    #[test]
    fn stepfun_vision_model_has_vision_flag() {
        let p = build_stepfun_profile();
        let meta = p.model_meta("step-v").unwrap();
        assert!(meta.capabilities.vision);
    }

    #[test]
    fn stepfun_error_429_is_rate_limit() {
        use crate::error::InferenceError;
        let err = normalize_error(429, "rate limited");
        assert!(matches!(err, InferenceError::RateLimit { .. }));
    }
}
