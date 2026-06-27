use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Portkey AI gateway base URL.
pub const PORTKEY_URL: &str = "https://api.portkey.ai/v1";

/// Build a Portkey gateway profile using the Portkey API key header.
///
/// Portkey is an enterprise AI gateway that supports routing, fallbacks,
/// caching, and observability across 250+ providers. Auth uses the
/// `x-portkey-api-key` header (read from `PORTKEY_API_KEY`). For virtual-key
/// routing, use `build_portkey_virtual_key_profile`.
pub fn build_portkey_profile() -> ProviderProfile {
    ProviderProfile::new(
        "portkey",
        PORTKEY_URL,
        AuthStrategy::HeaderKey {
            env_var: "PORTKEY_API_KEY".to_owned(),
            header: "x-portkey-api-key".to_owned(),
        },
    )
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

/// Build a Portkey profile that targets a specific virtual key.
///
/// Virtual keys let you pre-configure a provider and model in the Portkey
/// dashboard. The virtual key is sent in `x-portkey-virtual-key` and routes
/// directly to the configured upstream without needing the raw API key at
/// call time.
pub fn build_portkey_virtual_key_profile(virtual_key_env: impl Into<String>) -> ProviderProfile {
    let vk_env: String = virtual_key_env.into();
    ProviderProfile::new(
        "portkey",
        PORTKEY_URL,
        AuthStrategy::HeaderKey {
            env_var: "PORTKEY_API_KEY".to_owned(),
            header: "x-portkey-api-key".to_owned(),
        },
    )
    .with_extra_header("x-portkey-virtual-key", format!("${{{}}}", vk_env))
    .add_model(ModelMeta::new("default", 200_000).with_tools().with_vision().with_streaming())
}

/// Normalize a Portkey HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const PORTKEY_FIXTURE: &str = r#"{"id":"chatcmpl-pk-01","choices":[{"message":{"role":"assistant","content":"Hello from Portkey","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":11,"completion_tokens":4}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn pk_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_portkey_profile()))
    }

    #[test]
    fn portkey_provider_name() {
        assert_eq!(build_portkey_profile().name, "portkey");
    }

    #[test]
    fn portkey_recorded_fixture_completes() {
        let resp = pk_client().parse_response(PORTKEY_FIXTURE, "openai/gpt-4o").unwrap();
        assert_eq!(resp.content, "Hello from Portkey");
        assert_eq!(resp.tokens_in, 11);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn portkey_uses_header_key_auth() {
        let p = build_portkey_profile();
        assert!(matches!(
            &p.auth,
            AuthStrategy::HeaderKey { header, .. } if header == "x-portkey-api-key"
        ));
    }

    #[test]
    fn portkey_base_url() {
        assert_eq!(build_portkey_profile().base_url, PORTKEY_URL);
    }
}
