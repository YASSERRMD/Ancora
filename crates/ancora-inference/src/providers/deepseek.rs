use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the DeepSeek provider profile.
///
/// DeepSeek exposes an OpenAI-compatible API at `api.deepseek.com`.
/// Use with `OpenAiClient`. Reads the API key from `DEEPSEEK_API_KEY`.
///
/// Note: the direct endpoint routes through DeepSeek's CN-region
/// infrastructure. Use the self-host profile for non-CN residency.
pub fn build_deepseek_profile() -> ProviderProfile {
    ProviderProfile::new(
        "deepseek",
        "https://api.deepseek.com",
        AuthStrategy::BearerToken {
            env_var: "DEEPSEEK_API_KEY".to_owned(),
        },
    )
    // DeepSeek V3 (general-purpose chat)
    // Cache-hit pricing: $0.07/M (74% discount vs full $0.27/M input)
    .add_model(
        ModelMeta::new("deepseek-chat", 64_000)
            .with_pricing(0.27, 1.10)
            .with_cached_pricing(0.07)
            .with_tools()
            .with_streaming(),
    )
    // DeepSeek R1 (reasoning model)
    // Cache-hit pricing: $0.14/M (75% discount vs full $0.55/M input)
    .add_model(
        ModelMeta::new("deepseek-reasoner", 64_000)
            .with_pricing(0.55, 2.19)
            .with_cached_pricing(0.14)
            .with_streaming(),
    )
    // DeepSeek Coder (long-context, code-optimized)
    // Cache-hit pricing: $0.035/M
    .add_model(
        ModelMeta::new("deepseek-coder", 128_000)
            .with_pricing(0.14, 0.28)
            .with_cached_pricing(0.035)
            .with_tools()
            .with_streaming(),
    )
    // Aliases
    .add_alias("deepseek-v3", "deepseek-chat")
    .add_alias("deepseek-r1", "deepseek-reasoner")
    .add_alias("v3", "deepseek-chat")
    .add_alias("r1", "deepseek-reasoner")
    .add_alias("coder", "deepseek-coder")
}

/// Build a self-hosted DeepSeek profile for vLLM or other OpenAI-compatible servers.
///
/// Used when data must not leave a specific region. The base URL is read from
/// `DEEPSEEK_BASE_URL` (e.g. `http://localhost:8000`). Auth is disabled by
/// default; set `DEEPSEEK_SELF_HOST_KEY` if the server requires a token.
pub fn build_deepseek_self_host_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "deepseek-self-host",
        base_url,
        AuthStrategy::BearerToken {
            env_var: "DEEPSEEK_SELF_HOST_KEY".to_owned(),
        },
    )
    .add_model(
        ModelMeta::new("deepseek-chat", 64_000)
            .with_pricing(0.0, 0.0)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("deepseek-reasoner", 64_000)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    .add_alias("deepseek-v3", "deepseek-chat")
    .add_alias("deepseek-r1", "deepseek-reasoner")
}

/// Normalize DeepSeek HTTP error codes to `InferenceError`.
///
/// DeepSeek returns standard HTTP status codes. The OpenAI client's
/// `from_http` handler is sufficient for most cases; this function
/// documents the mapping and provides an entry point for additional
/// DeepSeek-specific error handling (e.g. the CN-region rate-limit body).
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

#[cfg(test)]
const DS_SELF_HOST_FIXTURE: &str = r#"{"id":"chatcmpl-sh-01","choices":[{"message":{"role":"assistant","content":"Hello from vLLM","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":5,"completion_tokens":4}}"#;

#[cfg(test)]
const DS_FIXTURE: &str = r#"{"id":"chatcmpl-ds-01","choices":[{"message":{"role":"assistant","content":"Hello from DeepSeek","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":5,"prompt_cache_hit_tokens":4,"prompt_cache_miss_tokens":6}}"#;

#[cfg(test)]
const DS_TOOL_FIXTURE: &str = r#"{"id":"chatcmpl-ds-02","choices":[{"message":{"role":"assistant","content":"","tool_calls":[{"id":"call-ds-01","type":"function","function":{"name":"get_weather","arguments":"{\"location\":\"Beijing\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":20,"completion_tokens":10,"prompt_cache_hit_tokens":0,"prompt_cache_miss_tokens":20}}"#;

#[cfg(test)]
const DS_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" DeepSeek"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

// DeepSeek R1 returns a reasoning_content field alongside content.
// The OpenAI client extracts `choices[].message.content`; reasoning_content
// is an additional field that should not break parsing.
#[cfg(test)]
const DS_REASONING_FIXTURE: &str = r#"{"id":"chatcmpl-ds-r1","choices":[{"message":{"role":"assistant","content":"The answer is 42","reasoning_content":"Let me think step by step..."},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":6}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deepseek_provider_name_is_deepseek() {
        assert_eq!(build_deepseek_profile().name, "deepseek");
    }

    #[test]
    fn deepseek_base_url_is_correct() {
        assert_eq!(
            build_deepseek_profile().base_url,
            "https://api.deepseek.com"
        );
    }

    #[test]
    fn deepseek_v3_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("deepseek-v3"), "deepseek-chat");
    }

    #[test]
    fn deepseek_r1_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("deepseek-r1"), "deepseek-reasoner");
    }

    #[test]
    fn deepseek_short_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("v3"), "deepseek-chat");
        assert_eq!(p.resolve_model_id("r1"), "deepseek-reasoner");
    }

    #[test]
    fn deepseek_chat_has_tools() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        assert!(m.capabilities.tools);
    }

    #[test]
    fn deepseek_reasoner_has_no_tools() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-reasoner").unwrap();
        assert!(!m.capabilities.tools);
    }

    #[test]
    fn deepseek_chat_has_cached_pricing() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        assert!(pricing.cached_per_million.is_some());
    }

    #[test]
    fn deepseek_r1_has_cached_pricing() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-reasoner").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        assert!(pricing.cached_per_million.is_some());
    }

    #[test]
    fn deepseek_cached_price_lower_than_full() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        let cached = pricing.cached_per_million.unwrap();
        assert!(cached < pricing.input_per_million);
    }

    #[test]
    fn deepseek_chat_context_window() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-chat").unwrap();
        assert_eq!(m.context_window, 64_000);
    }

    #[test]
    fn deepseek_reasoner_context_window() {
        let p = build_deepseek_profile();
        let m = p.model_meta("deepseek-reasoner").unwrap();
        assert_eq!(m.context_window, 64_000);
    }

    #[test]
    fn deepseek_coder_larger_context() {
        let p = build_deepseek_profile();
        let coder = p.model_meta("deepseek-coder").unwrap();
        let chat = p.model_meta("deepseek-chat").unwrap();
        assert!(coder.context_window >= chat.context_window);
    }

    #[test]
    fn deepseek_coder_alias_resolves() {
        let p = build_deepseek_profile();
        assert_eq!(p.resolve_model_id("coder"), "deepseek-coder");
    }

    #[test]
    fn deepseek_all_models_stream() {
        let p = build_deepseek_profile();
        for (id, m) in &p.model_catalog {
            assert!(m.capabilities.streaming, "{id} must stream");
        }
    }

    fn ds_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_deepseek_profile()))
    }

    #[test]
    fn deepseek_tool_call_mapping_request_has_tools() {
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        let mut req = CompletionRequest::simple(
            "deepseek-chat",
            vec![Message::text("user", "What is the weather?")],
        );
        req.tools = vec![ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "get_weather".to_owned(),
                description: "Get weather".to_owned(),
                parameters: serde_json::json!({"type":"object","properties":{"location":{"type":"string"}},"required":["location"]}),
            },
        }];
        let body = ds_client().build_request_body(&req, false).unwrap();
        assert!(body["tools"].is_array());
        assert_eq!(body["tools"][0]["function"]["name"], "get_weather");
    }

    #[test]
    fn deepseek_streaming_parser_uses_openai_sse() {
        use crate::openai::OpenAiClient;
        let texts: Vec<String> = DS_STREAM_LINES
            .iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(texts, vec!["Hello", " DeepSeek"]);
    }

    #[test]
    fn deepseek_reasoning_content_does_not_break_parsing() {
        // DeepSeek R1 adds a `reasoning_content` field to the message object.
        // The OpenAI client ignores unknown fields (serde default behavior),
        // so the standard `content` field is still extracted correctly.
        let resp = ds_client()
            .parse_response(DS_REASONING_FIXTURE, "deepseek-reasoner")
            .unwrap();
        assert_eq!(resp.content, "The answer is 42");
    }

    #[test]
    fn deepseek_recorded_fixture_completes() {
        let resp = ds_client()
            .parse_response(DS_FIXTURE, "deepseek-chat")
            .unwrap();
        assert_eq!(resp.content, "Hello from DeepSeek");
        assert_eq!(resp.tokens_in, 10);
        assert_eq!(resp.tokens_out, 5);
    }

    #[test]
    fn deepseek_fixture_no_tool_calls() {
        let resp = ds_client()
            .parse_response(DS_FIXTURE, "deepseek-chat")
            .unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn deepseek_fixture_content_non_empty() {
        let resp = ds_client()
            .parse_response(DS_FIXTURE, "deepseek-chat")
            .unwrap();
        assert!(!resp.content.is_empty());
    }

    #[test]
    fn deepseek_tool_round_trip_works() {
        let resp = ds_client()
            .parse_response(DS_TOOL_FIXTURE, "deepseek-chat")
            .unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        assert_eq!(resp.tool_calls[0].function.name, "get_weather");
        let args: serde_json::Value =
            serde_json::from_str(&resp.tool_calls[0].function.arguments).unwrap();
        assert_eq!(args["location"], "Beijing");
    }

    #[test]
    fn deepseek_streaming_fixture_ordered() {
        use crate::openai::OpenAiClient;
        let texts: Vec<String> = DS_STREAM_LINES
            .iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(texts, vec!["Hello", " DeepSeek"]);
    }

    #[test]
    fn deepseek_long_context_request_assembles_correctly() {
        use crate::types::{CompletionRequest, Message};
        // Build a request with many messages that would fill a large context
        let many_messages: Vec<Message> = (0..100)
            .map(|i| {
                if i % 2 == 0 {
                    Message::text("user", format!("Turn {i}: what is {i}+{i}?"))
                } else {
                    Message::text("assistant", format!("Turn {i}: the answer is {}", i + i))
                }
            })
            .collect();
        let req = CompletionRequest::simple("deepseek-coder", many_messages.clone());
        let body = ds_client().build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], "deepseek-coder");
        assert_eq!(body["messages"].as_array().unwrap().len(), 100);
    }

    #[test]
    fn deepseek_coder_model_fits_large_context() {
        let p = build_deepseek_profile();
        let meta = p.model_meta("deepseek-coder").unwrap();
        assert!(meta.fits_context(100_000));
    }

    #[test]
    fn deepseek_streaming_combined_text() {
        use crate::openai::OpenAiClient;
        let combined: String = DS_STREAM_LINES
            .iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text)
            .collect();
        assert_eq!(combined, "Hello DeepSeek");
    }

    #[test]
    fn deepseek_tool_fixture_token_counts() {
        let resp = ds_client()
            .parse_response(DS_TOOL_FIXTURE, "deepseek-chat")
            .unwrap();
        assert_eq!(resp.tokens_in, 20);
        assert_eq!(resp.tokens_out, 10);
    }

    #[test]
    fn deepseek_cache_hit_cost_vs_full_precise() {
        let p = build_deepseek_profile();
        let meta = p.model_meta("deepseek-chat").unwrap();
        let pricing = meta.pricing.as_ref().unwrap();
        let cached = pricing.cached_per_million.unwrap();
        // Cache-hit tier should be at least 50% cheaper
        assert!(cached <= pricing.input_per_million * 0.5);
    }

    #[test]
    fn deepseek_error_429_is_rate_limited() {
        use crate::error::InferenceError;
        let err = normalize_error(429, r#"{"error":{"message":"Rate limit exceeded"}}"#);
        assert!(matches!(err, InferenceError::RateLimit { .. }));
    }

    #[test]
    fn deepseek_error_401_is_auth_rejected() {
        use crate::error::InferenceError;
        let err = normalize_error(401, r#"{"error":{"message":"Invalid API key"}}"#);
        assert!(matches!(err, InferenceError::AuthRejected(_)));
    }

    #[test]
    fn deepseek_rate_limit_backoff_with_retry_after() {
        use crate::error::InferenceError;
        let err = normalize_error(429, "rate limited");
        // Without a Retry-After header there is no duration hint
        if let InferenceError::RateLimit { retry_after } = err {
            assert!(retry_after.is_none(), "no header means no hint");
        } else {
            panic!("expected RateLimit");
        }
    }

    #[test]
    fn deepseek_self_host_profile_base_url() {
        let p = build_deepseek_self_host_profile("http://localhost:8000");
        assert_eq!(p.base_url, "http://localhost:8000");
        assert_eq!(p.name, "deepseek-self-host");
    }

    #[test]
    fn deepseek_self_host_has_zero_cost() {
        let p = build_deepseek_self_host_profile("http://localhost:8000");
        let m = p.model_meta("deepseek-chat").unwrap();
        let pricing = m.pricing.as_ref().unwrap();
        assert_eq!(pricing.input_per_million, 0.0);
        assert_eq!(pricing.output_per_million, 0.0);
    }

    #[test]
    fn deepseek_self_host_fixture_completes_offline() {
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        let client = OpenAiClient::new(Arc::new(build_deepseek_self_host_profile(
            "http://localhost:8000",
        )));
        let resp = client
            .parse_response(DS_SELF_HOST_FIXTURE, "deepseek-chat")
            .unwrap();
        assert_eq!(resp.content, "Hello from vLLM");
        assert_eq!(resp.tokens_in, 5);
        assert_eq!(resp.tokens_out, 4);
    }

    #[test]
    fn deepseek_self_host_fixture_has_zero_cost() {
        use crate::openai::OpenAiClient;
        use std::sync::Arc;
        let client = OpenAiClient::new(Arc::new(build_deepseek_self_host_profile(
            "http://localhost:8000",
        )));
        let resp = client
            .parse_response(DS_SELF_HOST_FIXTURE, "deepseek-chat")
            .unwrap();
        // Zero pricing configured -- cost should be 0 or None
        let cost = resp.cost_usd.unwrap_or(0.0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn deepseek_self_host_alias_works() {
        let p = build_deepseek_self_host_profile("http://localhost:8000");
        assert_eq!(p.resolve_model_id("deepseek-v3"), "deepseek-chat");
        assert_eq!(p.resolve_model_id("deepseek-r1"), "deepseek-reasoner");
    }

    #[test]
    fn deepseek_rate_limit_with_retry_after_header() {
        use crate::error::InferenceError;
        // When DeepSeek provides Retry-After, the duration is parsed
        let err = InferenceError::from_http(429, "rate limited", Some("30"));
        if let InferenceError::RateLimit { retry_after } = err {
            assert_eq!(retry_after.unwrap().as_secs(), 30);
        } else {
            panic!("expected RateLimit");
        }
    }

    #[test]
    fn deepseek_error_500_is_http_error() {
        use crate::error::InferenceError;
        let err = normalize_error(500, "internal server error");
        assert!(matches!(err, InferenceError::Http { status: 500, .. }));
    }

    #[test]
    fn deepseek_r1_cache_discount_at_least_50_pct() {
        let p = build_deepseek_profile();
        let meta = p.model_meta("deepseek-reasoner").unwrap();
        let pricing = meta.pricing.as_ref().unwrap();
        let cached = pricing.cached_per_million.unwrap();
        assert!(cached <= pricing.input_per_million * 0.5);
    }

    #[test]
    fn deepseek_cache_hit_tokens_present_in_fixture() {
        // The fixture has prompt_cache_hit_tokens=4 and prompt_cache_miss_tokens=6.
        // Standard OpenAI parse uses prompt_tokens (10) for tokens_in.
        // Cache-hit accounting is handled by the provider profile's cached_per_million
        // tier; the `compute_cost` method receives cached_in as the third argument.
        let raw: serde_json::Value = serde_json::from_str(DS_FIXTURE).unwrap();
        let cached = raw["usage"]["prompt_cache_hit_tokens"]
            .as_u64()
            .unwrap_or(0);
        assert_eq!(cached, 4);
    }

    #[test]
    fn deepseek_cache_hit_cost_lower_than_full() {
        let p = build_deepseek_profile();
        let meta = p.model_meta("deepseek-chat").unwrap();
        let full_cost = meta.compute_cost(10, 5, 0).unwrap();
        // 4 cached tokens billed at $0.07/M instead of $0.27/M
        let cached_cost = meta.compute_cost(6, 5, 4).unwrap();
        assert!(cached_cost < full_cost);
    }

    #[test]
    fn deepseek_reasoning_fixture_tokens_correct() {
        let resp = ds_client()
            .parse_response(DS_REASONING_FIXTURE, "deepseek-reasoner")
            .unwrap();
        assert_eq!(resp.tokens_in, 8);
        assert_eq!(resp.tokens_out, 6);
    }

    #[test]
    fn deepseek_stream_done_emits_finished() {
        use crate::openai::OpenAiClient;
        let ev = OpenAiClient::parse_sse_line("data: [DONE]").unwrap();
        assert!(ev.finished);
    }

    #[test]
    fn deepseek_tool_call_model_resolved_in_body() {
        use crate::types::{CompletionRequest, Message};
        let req = CompletionRequest::simple("deepseek-v3", vec![Message::text("user", "Hi")]);
        let body = ds_client().build_request_body(&req, false).unwrap();
        assert_eq!(body["model"], "deepseek-chat");
    }
}
