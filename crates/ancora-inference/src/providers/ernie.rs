use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Baidu ERNIE OpenAI-compatible endpoint.
pub const ERNIE_URL: &str = "https://qianfan.baidubce.com/v2";

/// Build the Baidu ERNIE (Qianfan) provider profile.
///
/// Uses the Qianfan OpenAI-compatible endpoint. Auth is read from
/// `ERNIE_API_KEY`. The older OAuth flow (client_id + client_secret ->
/// access_token) is documented in `ernie_oauth_note` below.
pub fn build_ernie_profile() -> ProviderProfile {
    ProviderProfile::new(
        "ernie",
        ERNIE_URL,
        AuthStrategy::BearerToken {
            env_var: "ERNIE_API_KEY".to_owned(),
        },
    )
    // ERNIE 4.0 -- flagship, tools, 8k context
    .add_model(
        ModelMeta::new("ernie-4.0-8k", 8_192)
            .with_pricing(0.12, 0.12)
            .with_tools()
            .with_streaming(),
    )
    // ERNIE 3.5 -- balanced
    .add_model(
        ModelMeta::new("ernie-3.5-8k", 8_192)
            .with_pricing(0.05, 0.05)
            .with_tools()
            .with_streaming(),
    )
    // ERNIE Speed -- fastest, cheapest
    .add_model(
        ModelMeta::new("ernie-speed-8k", 8_192)
            .with_pricing(0.004, 0.008)
            .with_streaming(),
    )
    // ERNIE Lite
    .add_model(
        ModelMeta::new("ernie-lite-8k", 8_192)
            .with_pricing(0.003, 0.006)
            .with_streaming(),
    )
    .add_alias("ernie4", "ernie-4.0-8k")
    .add_alias("ernie3.5", "ernie-3.5-8k")
    .add_alias("speed", "ernie-speed-8k")
    .add_alias("lite", "ernie-lite-8k")
}

/// Normalize a Baidu ERNIE HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const ERNIE_FIXTURE: &str = r#"{"id":"chatcmpl-ernie-01","choices":[{"message":{"role":"assistant","content":"Hello from ERNIE 4.0","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":7}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn ernie_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_ernie_profile()))
    }

    #[test]
    fn ernie_recorded_fixture_completes() {
        let resp = ernie_client()
            .parse_response(ERNIE_FIXTURE, "ernie-4.0-8k")
            .unwrap();
        assert_eq!(resp.content, "Hello from ERNIE 4.0");
        assert_eq!(resp.tokens_in, 9);
        assert_eq!(resp.tokens_out, 7);
    }

    #[test]
    fn ernie_provider_name() {
        assert_eq!(build_ernie_profile().name, "ernie");
    }

    #[test]
    fn ernie_oauth_note_nonempty() {
        assert!(!ernie_oauth_note().is_empty());
    }

    #[test]
    fn ernie4_has_tools() {
        let p = build_ernie_profile();
        assert!(p.model_meta("ernie-4.0-8k").unwrap().capabilities.tools);
    }
}

/// Return a note about the legacy Baidu OAuth auth flow.
///
/// The new Qianfan endpoint (`qianfan.baidubce.com/v2`) accepts API keys
/// directly. The legacy flow (used by older `aip.baidubce.com` endpoints)
/// requires exchanging a `client_id` and `client_secret` for a temporary
/// access_token via a separate HTTP call. This library uses the modern
/// API-key flow only.
pub fn ernie_oauth_note() -> &'static str {
    "Legacy flow: POST https://aip.baidubce.com/oauth/2.0/token \
     ?grant_type=client_credentials&client_id=<AK>&client_secret=<SK>. \
     Use the Qianfan API key flow instead."
}
