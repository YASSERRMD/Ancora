use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// International endpoint for Kimi (Moonshot AI).
pub const KIMI_URL_INTERNATIONAL: &str = "https://api.moonshot.ai/v1";

/// Domestic CN endpoint for Kimi.
pub const KIMI_URL_DOMESTIC: &str = "https://api.moonshot.cn/v1";

/// Build the Kimi (Moonshot AI) international provider profile.
///
/// Routes to the international endpoint. Auth is read from `MOONSHOT_API_KEY`.
pub fn build_kimi_profile() -> ProviderProfile {
    ProviderProfile::new(
        "kimi",
        KIMI_URL_INTERNATIONAL,
        AuthStrategy::BearerToken {
            env_var: "MOONSHOT_API_KEY".to_owned(),
        },
    )
    .add_region("intl", KIMI_URL_INTERNATIONAL)
    .add_region("cn", KIMI_URL_DOMESTIC)
    // Kimi K2 -- most capable, long context, tools (flagship)
    .add_model(
        ModelMeta::new("kimi-k2", 131_072)
            .with_pricing(0.60, 2.50)
            .with_tools()
            .with_streaming(),
    )
    // Kimi K2 Turbo -- faster, same context
    .add_model(
        ModelMeta::new("kimi-k2-turbo", 131_072)
            .with_pricing(0.20, 0.80)
            .with_tools()
            .with_streaming(),
    )
    // Classic moonshot models (still in use)
    .add_model(
        ModelMeta::new("moonshot-v1-128k", 131_072)
            .with_pricing(1.00, 1.00)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("moonshot-v1-32k", 32_768)
            .with_pricing(0.24, 0.24)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("moonshot-v1-8k", 8_192)
            .with_pricing(0.12, 0.12)
            .with_streaming(),
    )
    // Kimi Moonlight -- very long context, designed for document analysis
    .add_model(
        ModelMeta::new("moonshot-v1-long", 1_000_000)
            .with_pricing(1.50, 1.50)
            .with_streaming(),
    )
    .add_alias("k2", "kimi-k2")
    .add_alias("k2-turbo", "kimi-k2-turbo")
    .add_alias("128k", "moonshot-v1-128k")
    .add_alias("32k", "moonshot-v1-32k")
    .add_alias("8k", "moonshot-v1-8k")
    .add_alias("long", "moonshot-v1-long")
}

/// Parse a single SSE line from a Kimi streaming response.
///
/// Kimi uses the standard OpenAI SSE format.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Build a profile that proxies Kimi through a local gateway (e.g. LiteLLM).
///
/// Neither Kimi K2 nor Moonshot models are open-weight. Use this profile
/// when an on-premises gateway forwards requests to Moonshot's API on behalf
/// of a private network. Auth is read from `KIMI_GATEWAY_KEY`.
pub fn build_kimi_gateway_profile(gateway_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "kimi-gateway",
        gateway_url,
        AuthStrategy::BearerToken {
            env_var: "KIMI_GATEWAY_KEY".to_owned(),
        },
    )
    .add_model(
        ModelMeta::new("kimi-k2", 131_072)
            .with_pricing(0.0, 0.0)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("k2", "kimi-k2")
}

/// Return `true` if the model supports tool/function calls.
pub fn supports_tools(model_id: &str) -> bool {
    let p = build_kimi_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog
        .get(canonical)
        .map_or(false, |m| m.capabilities.tools)
}

/// Build the Kimi domestic (China) provider profile.
///
/// Routes to `api.moonshot.cn`. Use when running workloads inside China or
/// where international routing is unavailable. Same API key.
pub fn build_kimi_domestic_profile() -> ProviderProfile {
    ProviderProfile::new(
        "kimi-cn",
        KIMI_URL_DOMESTIC,
        AuthStrategy::BearerToken {
            env_var: "MOONSHOT_API_KEY".to_owned(),
        },
    )
    .add_model(
        ModelMeta::new("kimi-k2", 131_072)
            .with_pricing(0.60, 2.50)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("moonshot-v1-128k", 131_072)
            .with_pricing(1.00, 1.00)
            .with_streaming(),
    )
}

#[cfg(test)]
const KIMI_FIXTURE: &str = r#"{"id":"chatcmpl-kimi-01","choices":[{"message":{"role":"assistant","content":"Hello from Kimi K2","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":11,"completion_tokens":8}}"#;

#[cfg(test)]
const KIMI_TOOL_FIXTURE: &str = r#"{"id":"chatcmpl-kimi-02","choices":[{"message":{"role":"assistant","content":"","tool_calls":[{"id":"call-km-01","type":"function","function":{"name":"search","arguments":"{\"query\":\"LLM benchmarks 2025\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":22,"completion_tokens":11}}"#;

#[cfg(test)]
const KIMI_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Kimi"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" K2"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn kimi_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_kimi_profile()))
    }

    #[test]
    fn kimi_recorded_fixture_completes() {
        let resp = kimi_client()
            .parse_response(KIMI_FIXTURE, "kimi-k2")
            .unwrap();
        assert_eq!(resp.content, "Hello from Kimi K2");
        assert_eq!(resp.tokens_in, 11);
        assert_eq!(resp.tokens_out, 8);
    }

    #[test]
    fn kimi_provider_name_is_kimi() {
        assert_eq!(build_kimi_profile().name, "kimi");
    }

    #[test]
    fn kimi_default_url_is_international() {
        let p = build_kimi_profile();
        assert!(p.base_url.contains("moonshot.ai"));
    }

    #[test]
    fn kimi_cn_region_resolves_domestic() {
        let p = build_kimi_profile();
        assert!(p.base_url_for_region(Some("cn")).contains("moonshot.cn"));
    }

    #[test]
    fn kimi_tool_round_trip_works() {
        let resp = kimi_client()
            .parse_response(KIMI_TOOL_FIXTURE, "kimi-k2")
            .unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        assert_eq!(resp.tool_calls[0].function.name, "search");
        let args: serde_json::Value =
            serde_json::from_str(&resp.tool_calls[0].function.arguments).unwrap();
        assert_eq!(args["query"], "LLM benchmarks 2025");
    }

    #[test]
    fn kimi_k2_has_tools() {
        assert!(supports_tools("kimi-k2"));
    }

    #[test]
    fn kimi_128k_has_no_tools() {
        assert!(!supports_tools("moonshot-v1-128k"));
    }

    #[test]
    fn kimi_long_context_assembly_correct() {
        use crate::types::{CompletionRequest, Message};
        let many: Vec<Message> = (0..200)
            .map(|i| {
                Message::text(
                    if i % 2 == 0 { "user" } else { "assistant" },
                    &format!("msg {i}"),
                )
            })
            .collect();
        let req = CompletionRequest::simple("moonshot-v1-long", many.clone());
        let body = kimi_client().build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], "moonshot-v1-long");
        assert_eq!(body["messages"].as_array().unwrap().len(), 200);
    }

    #[test]
    fn kimi_long_model_fits_large_context() {
        let p = build_kimi_profile();
        let meta = p.model_meta("moonshot-v1-long").unwrap();
        assert!(meta.fits_context(500_000));
    }

    #[test]
    fn kimi_streaming_fixture_ordered() {
        use crate::openai::OpenAiClient;
        let texts: Vec<String> = KIMI_STREAM_LINES
            .iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(texts, vec!["Kimi", " K2"]);
    }

    #[test]
    fn kimi_gateway_fixture_completes_offline() {
        use std::sync::Arc;
        let client = crate::openai::OpenAiClient::new(Arc::new(build_kimi_gateway_profile(
            "http://localhost:4000",
        )));
        let resp = client.parse_response(KIMI_FIXTURE, "kimi-k2").unwrap();
        assert_eq!(resp.content, "Hello from Kimi K2");
    }

    #[test]
    fn kimi_gateway_profile_name() {
        let p = build_kimi_gateway_profile("http://localhost:4000");
        assert_eq!(p.name, "kimi-gateway");
    }
}
