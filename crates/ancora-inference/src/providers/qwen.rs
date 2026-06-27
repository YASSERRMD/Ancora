use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// DashScope international endpoint (Singapore).
pub const QWEN_URL_SINGAPORE: &str =
    "https://dashscope-intl.aliyuncs.com/compatible-mode";

/// DashScope Frankfurt regional endpoint (EU, GDPR-compliant processing).
pub const QWEN_URL_FRANKFURT: &str =
    "https://dashscope-intl-eu.aliyuncs.com/compatible-mode";

/// DashScope Virginia regional endpoint (US East).
pub const QWEN_URL_VIRGINIA: &str =
    "https://dashscope-intl-us.aliyuncs.com/compatible-mode";

/// DashScope China domestic endpoint (routes through CN infrastructure).
pub const QWEN_URL_CHINA: &str =
    "https://dashscope.aliyuncs.com/compatible-mode";

/// Build the Alibaba Qwen (DashScope) provider profile.
///
/// Uses the Singapore international endpoint by default. Auth is read from
/// `DASHSCOPE_API_KEY`. Call `.base_url_for_region(Some("eu"))` on the
/// returned profile to direct requests to a different region.
pub fn build_qwen_profile() -> ProviderProfile {
    ProviderProfile::new(
        "qwen",
        QWEN_URL_SINGAPORE,
        AuthStrategy::BearerToken { env_var: "DASHSCOPE_API_KEY".to_owned() },
    )
    .add_region("sg", QWEN_URL_SINGAPORE)
    .add_region("eu", QWEN_URL_FRANKFURT)
    .add_region("us", QWEN_URL_VIRGINIA)
    .add_region("cn", QWEN_URL_CHINA)
    // Qwen3 235B MoE -- flagship; tools, 128k context
    .add_model(
        ModelMeta::new("qwen3-235b-a22b", 131_072)
            .with_pricing(1.30, 5.20)
            .with_tools()
            .with_streaming(),
    )
    // Qwen3 32B dense -- strong, tools
    .add_model(
        ModelMeta::new("qwen3-32b", 131_072)
            .with_pricing(0.45, 1.80)
            .with_tools()
            .with_streaming(),
    )
    // Qwen3 14B
    .add_model(
        ModelMeta::new("qwen3-14b", 131_072)
            .with_pricing(0.17, 0.68)
            .with_tools()
            .with_streaming(),
    )
    // Qwen3 8B -- lightweight
    .add_model(
        ModelMeta::new("qwen3-8b", 131_072)
            .with_pricing(0.06, 0.24)
            .with_tools()
            .with_streaming(),
    )
    // QwQ 32B -- reasoning/thinking model (no tool calls)
    .add_model(
        ModelMeta::new("qwq-32b", 131_072)
            .with_pricing(0.20, 0.60)
            .with_streaming(),
    )
    // Qwen Max -- highest-quality non-open-weight tier
    .add_model(
        ModelMeta::new("qwen-max", 32_768)
            .with_pricing(1.60, 6.40)
            .with_tools()
            .with_streaming(),
    )
    // Qwen Plus -- balanced capability/cost; cached input at 25% of full rate
    .add_model(
        ModelMeta::new("qwen-plus", 131_072)
            .with_pricing(0.40, 1.20)
            .with_cached_pricing(0.10)
            .with_tools()
            .with_streaming(),
    )
    // Qwen Turbo -- fastest, lowest cost; cached input at 20% of full rate
    .add_model(
        ModelMeta::new("qwen-turbo", 131_072)
            .with_pricing(0.05, 0.10)
            .with_cached_pricing(0.01)
            .with_tools()
            .with_streaming(),
    )
    // Qwen Long -- 1M context window for massive documents
    .add_model(
        ModelMeta::new("qwen-long", 1_000_000)
            .with_pricing(0.05, 0.20)
            .with_streaming(),
    )
    // Vision-language models
    .add_model(
        ModelMeta::new("qwen-vl-max", 32_768)
            .with_pricing(3.00, 9.00)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("qwen-vl-plus", 32_768)
            .with_pricing(0.80, 2.40)
            .with_vision()
            .with_streaming(),
    )
    // Model-id aliases for convenience
    .add_alias("qwen3-max", "qwen3-235b-a22b")
    .add_alias("qwq", "qwq-32b")
    .add_alias("max", "qwen-max")
    .add_alias("plus", "qwen-plus")
    .add_alias("turbo", "qwen-turbo")
    .add_alias("vl-max", "qwen-vl-max")
    .add_alias("vl-plus", "qwen-vl-plus")
    .add_alias("long", "qwen-long")
}

/// Parse a single SSE line from a DashScope streaming response.
///
/// DashScope uses the standard OpenAI SSE format:
/// `data: {"choices":[{"delta":{"content":"..."},"finish_reason":null}]}`
/// Delegates to `OpenAiClient::parse_sse_line` which handles `[DONE]`.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Build a self-hosted Qwen profile for vLLM or other OpenAI-compatible servers.
///
/// Qwen open-weight models (qwen3-32b, qwq-32b) can be served locally via vLLM.
/// This gives full data residency control. Auth is optional; set `QWEN_SELF_HOST_KEY`
/// if the server requires a bearer token.
pub fn build_qwen_self_host_profile(base_url: impl Into<String>) -> ProviderProfile {
    ProviderProfile::new(
        "qwen-self-host",
        base_url,
        AuthStrategy::BearerToken { env_var: "QWEN_SELF_HOST_KEY".to_owned() },
    )
    // Qwen3 32B is the largest open-weight dense model
    .add_model(
        ModelMeta::new("qwen3-32b", 131_072)
            .with_pricing(0.0, 0.0)
            .with_tools()
            .with_streaming(),
    )
    // QwQ 32B reasoning model (MIT license)
    .add_model(
        ModelMeta::new("qwq-32b", 131_072)
            .with_pricing(0.0, 0.0)
            .with_streaming(),
    )
    // Qwen3 14B for lower-VRAM deployments
    .add_model(
        ModelMeta::new("qwen3-14b", 131_072)
            .with_pricing(0.0, 0.0)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("qwq", "qwq-32b")
    .add_alias("qwen3", "qwen3-32b")
}

/// Return `true` if the model supports tool/function calls.
///
/// Qwen uses the standard OpenAI `tools` array in the request body.
/// Tool-capable models: qwen3-235b-a22b, qwen3-32b, qwen3-14b, qwen3-8b,
/// qwen-max, qwen-plus, qwen-turbo.
pub fn supports_tools(model_id: &str) -> bool {
    let p = build_qwen_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog.get(canonical).map_or(false, |m| m.capabilities.tools)
}

#[cfg(test)]
const QWEN_FIXTURE: &str = r#"{"id":"chatcmpl-qwen-01","choices":[{"message":{"role":"assistant","content":"Hello from Qwen","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":12,"completion_tokens":6}}"#;

#[cfg(test)]
const QWEN_TOOL_FIXTURE: &str = r#"{"id":"chatcmpl-qwen-02","choices":[{"message":{"role":"assistant","content":"","tool_calls":[{"id":"call-qw-01","type":"function","function":{"name":"translate","arguments":"{\"text\":\"Hello\",\"target_lang\":\"zh\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":25,"completion_tokens":12}}"#;

#[cfg(test)]
const QWEN_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Ni"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":"hao"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
const QWEN_SELF_HOST_FIXTURE: &str = r#"{"id":"chatcmpl-sh-qwen-01","choices":[{"message":{"role":"assistant","content":"Hello from self-hosted Qwen","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":8,"completion_tokens":7}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn qwen_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_qwen_profile()))
    }

    #[test]
    fn qwen_recorded_fixture_completes() {
        let resp = qwen_client().parse_response(QWEN_FIXTURE, "qwen-plus").unwrap();
        assert_eq!(resp.content, "Hello from Qwen");
        assert_eq!(resp.tokens_in, 12);
        assert_eq!(resp.tokens_out, 6);
    }

    #[test]
    fn qwen_fixture_no_tool_calls() {
        let resp = qwen_client().parse_response(QWEN_FIXTURE, "qwen-plus").unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn qwen_provider_name_is_qwen() {
        assert_eq!(build_qwen_profile().name, "qwen");
    }

    #[test]
    fn qwen_default_base_url_is_singapore() {
        let p = build_qwen_profile();
        assert!(p.base_url.contains("dashscope-intl.aliyuncs.com"));
    }

    #[test]
    fn qwen_tool_round_trip_works() {
        let resp = qwen_client().parse_response(QWEN_TOOL_FIXTURE, "qwen-plus").unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        assert_eq!(resp.tool_calls[0].function.name, "translate");
        let args: serde_json::Value =
            serde_json::from_str(&resp.tool_calls[0].function.arguments).unwrap();
        assert_eq!(args["target_lang"], "zh");
    }

    #[test]
    fn qwen_tool_call_request_body_has_tools() {
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        let mut req = CompletionRequest::simple(
            "qwen-plus",
            vec![Message::text("user", "Translate: Hello")],
        );
        req.tools = vec![ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "translate".to_owned(),
                description: "Translate text".to_owned(),
                parameters: serde_json::json!({"type":"object","properties":{"text":{"type":"string"},"target_lang":{"type":"string"}},"required":["text","target_lang"]}),
            },
        }];
        let body = qwen_client().build_request_body(&req, false).unwrap();
        assert!(body["tools"].is_array());
        assert_eq!(body["tools"][0]["function"]["name"], "translate");
    }

    #[test]
    fn qwen_tool_fixture_token_counts() {
        let resp = qwen_client().parse_response(QWEN_TOOL_FIXTURE, "qwen-plus").unwrap();
        assert_eq!(resp.tokens_in, 25);
        assert_eq!(resp.tokens_out, 12);
    }

    #[test]
    fn qwen_streaming_fixture_ordered() {
        use crate::openai::OpenAiClient;
        let tokens: Vec<String> = QWEN_STREAM_LINES.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(tokens, vec!["Ni", "hao"]);
    }

    #[test]
    fn qwen_streaming_combined_text() {
        use crate::openai::OpenAiClient;
        let combined: String = QWEN_STREAM_LINES.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text)
            .collect();
        assert_eq!(combined, "Nihao");
    }

    #[test]
    fn qwen_stream_done_emits_finished() {
        use crate::openai::OpenAiClient;
        let ev = OpenAiClient::parse_sse_line("data: [DONE]").unwrap();
        assert!(ev.finished);
    }

    #[test]
    fn qwen_sg_region_resolves_singapore_url() {
        let p = build_qwen_profile();
        assert!(p.base_url_for_region(Some("sg")).contains("dashscope-intl.aliyuncs.com"));
        assert!(!p.base_url_for_region(Some("sg")).contains("-eu"));
        assert!(!p.base_url_for_region(Some("sg")).contains("-us"));
    }

    #[test]
    fn qwen_eu_region_resolves_frankfurt_url() {
        let p = build_qwen_profile();
        assert!(p.base_url_for_region(Some("eu")).contains("-eu"));
    }

    #[test]
    fn qwen_us_region_resolves_virginia_url() {
        let p = build_qwen_profile();
        assert!(p.base_url_for_region(Some("us")).contains("-us"));
    }

    #[test]
    fn qwen_cn_region_resolves_china_url() {
        let p = build_qwen_profile();
        let cn_url = p.base_url_for_region(Some("cn"));
        assert!(cn_url.contains("dashscope.aliyuncs.com"));
        assert!(!cn_url.contains("-intl"));
    }

    #[test]
    fn qwen_unknown_region_falls_back_to_default() {
        let p = build_qwen_profile();
        // Unknown region should fall back to the default (Singapore)
        assert_eq!(p.base_url_for_region(Some("au")), p.base_url_for_region(None));
    }

    #[test]
    fn qwen_self_host_fixture_completes_offline() {
        use std::sync::Arc;
        let client = crate::openai::OpenAiClient::new(Arc::new(
            build_qwen_self_host_profile("http://localhost:8000"),
        ));
        let resp = client.parse_response(QWEN_SELF_HOST_FIXTURE, "qwen3-32b").unwrap();
        assert_eq!(resp.content, "Hello from self-hosted Qwen");
        assert_eq!(resp.tokens_in, 8);
        assert_eq!(resp.tokens_out, 7);
    }

    #[test]
    fn qwen_self_host_has_zero_cost() {
        use std::sync::Arc;
        let client = crate::openai::OpenAiClient::new(Arc::new(
            build_qwen_self_host_profile("http://localhost:8000"),
        ));
        let resp = client.parse_response(QWEN_SELF_HOST_FIXTURE, "qwen3-32b").unwrap();
        let cost = resp.cost_usd.unwrap_or(0.0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn qwen_self_host_base_url_is_custom() {
        let p = build_qwen_self_host_profile("http://gpu-box:8000");
        assert_eq!(p.base_url, "http://gpu-box:8000");
        assert_eq!(p.name, "qwen-self-host");
    }

    #[test]
    fn qwen_self_host_alias_resolves() {
        let p = build_qwen_self_host_profile("http://localhost:8000");
        assert_eq!(p.resolve_model_id("qwq"), "qwq-32b");
        assert_eq!(p.resolve_model_id("qwen3"), "qwen3-32b");
    }
}
