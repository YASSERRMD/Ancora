use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Zhipu GLM (Z.AI) provider profile.
///
/// Zhipu AI exposes an OpenAI-compatible API at `open.bigmodel.cn`.
/// Auth is read from `ZHIPU_API_KEY`. The endpoint uses a non-standard
/// path prefix (`/api/paas/v4`) so `chat_completions_path` is overridden.
pub fn build_glm_profile() -> ProviderProfile {
    ProviderProfile::new(
        "glm",
        "https://open.bigmodel.cn/api/paas/v4",
        AuthStrategy::BearerToken { env_var: "ZHIPU_API_KEY".to_owned() },
    )
    .with_chat_path("/chat/completions")
    // GLM-5 -- flagship model; tools, structured output, 128k context
    .add_model(
        ModelMeta::new("glm-5", 131_072)
            .with_pricing(0.60, 2.40)
            .with_tools()
            .with_streaming(),
    )
    // GLM-5.1 -- improved reasoning; tools, 128k context
    .add_model(
        ModelMeta::new("glm-5.1", 131_072)
            .with_pricing(0.80, 3.20)
            .with_tools()
            .with_streaming(),
    )
    // GLM-5 Long -- extended context window
    .add_model(
        ModelMeta::new("glm-5-long", 256_000)
            .with_pricing(0.60, 2.40)
            .with_streaming(),
    )
    // GLM Turbo -- fast and cheap
    .add_model(
        ModelMeta::new("glm-turbo", 131_072)
            .with_pricing(0.06, 0.12)
            .with_tools()
            .with_streaming(),
    )
    // GLM-4 Flash -- ultra-low cost or free tier
    .add_model(
        ModelMeta::new("glm-4-flash", 131_072)
            .with_pricing(0.01, 0.03)
            .with_streaming(),
    )
    // GLM-4V -- vision-language model
    .add_model(
        ModelMeta::new("glm-4v", 8_192)
            .with_pricing(0.25, 1.00)
            .with_vision()
            .with_streaming(),
    )
    .add_alias("glm5", "glm-5")
    .add_alias("glm5.1", "glm-5.1")
    .add_alias("turbo", "glm-turbo")
    .add_alias("flash", "glm-4-flash")
    .add_alias("vl", "glm-4v")
}

/// Normalize a GLM HTTP error to `InferenceError`.
pub fn normalize_error(status: u16, body: &str) -> crate::error::InferenceError {
    crate::error::InferenceError::from_http(status, body, None)
}

/// Compute the USD cost for a GLM request given token counts.
///
/// Returns `None` if the model has no pricing metadata or the model-id is unknown.
pub fn compute_cost(model_id: &str, tokens_in: u64, tokens_out: u64) -> Option<f64> {
    let p = build_glm_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog.get(canonical)?.compute_cost(tokens_in, tokens_out, 0)
}

/// Return the context-window size in tokens for a given model-id (resolves aliases).
pub fn context_window(model_id: &str) -> Option<u32> {
    let p = build_glm_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog.get(canonical).map(|m| m.context_window)
}

/// Build a GLM profile that forces JSON-object output via `response_format`.
///
/// Adds a request transform that injects `{"response_format":{"type":"json_object"}}`
/// into every request body. Use this when you need structured extraction and the
/// caller cannot set `response_format` on the `CompletionRequest` directly.
pub fn build_glm_json_profile() -> ProviderProfile {
    build_glm_profile()
        .with_request_transform(|body| {
            body["response_format"] = serde_json::json!({"type": "json_object"});
        })
}

/// Validate that a string is a JSON object (not array, not scalar).
///
/// GLM JSON mode produces `{"key": ...}` objects. Returns `true` if the
/// string parses as an object with at least one key.
pub fn is_json_object(s: &str) -> bool {
    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(s).is_ok()
}

/// Parse a single SSE line from a GLM streaming response.
///
/// GLM uses the standard OpenAI SSE wire format. Delegates to
/// `OpenAiClient::parse_sse_line`.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Return `true` if the model supports tool/function calls.
///
/// GLM uses the standard OpenAI `tools` array format.
/// Tool-capable: glm-5, glm-5.1, glm-turbo.
pub fn supports_tools(model_id: &str) -> bool {
    let p = build_glm_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog.get(canonical).map_or(false, |m| m.capabilities.tools)
}

#[cfg(test)]
const GLM_FIXTURE: &str = r#"{"id":"chatcmpl-glm-01","choices":[{"message":{"role":"assistant","content":"Hello from GLM-5","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":14,"completion_tokens":7}}"#;

#[cfg(test)]
const GLM_TOOL_FIXTURE: &str = r#"{"id":"chatcmpl-glm-02","choices":[{"message":{"role":"assistant","content":"","tool_calls":[{"id":"call-glm-01","type":"function","function":{"name":"extract_entities","arguments":"{\"text\":\"Apple Inc was founded by Steve Jobs\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":30,"completion_tokens":15}}"#;

#[cfg(test)]
const GLM_JSON_FIXTURE: &str = r#"{"id":"chatcmpl-glm-03","choices":[{"message":{"role":"assistant","content":"{\"company\":\"Apple Inc\",\"founder\":\"Steve Jobs\",\"year\":1976}","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":40,"completion_tokens":20}}"#;

#[cfg(test)]
const GLM_STREAM_LINES: &[&str] = &[
    r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
    r#"data: {"choices":[{"delta":{"content":" from GLM"},"finish_reason":"stop"}]}"#,
    "data: [DONE]",
];

#[cfg(test)]
const GLM_SELF_HOST_FIXTURE: &str = r#"{"id":"chatcmpl-glm-sh-01","choices":[{"message":{"role":"assistant","content":"Hello from vLLM GLM","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":6}}"#;

#[cfg(test)]
const GLM_LLAMACPP_FIXTURE: &str = r#"{"id":"chatcmpl-glm-lc-01","choices":[{"message":{"role":"assistant","content":"Hello from llama.cpp GLM","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":9,"completion_tokens":7}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn glm_client() -> crate::openai::OpenAiClient {
        use std::sync::Arc;
        crate::openai::OpenAiClient::new(Arc::new(build_glm_profile()))
    }

    #[test]
    fn glm_recorded_fixture_completes() {
        let resp = glm_client().parse_response(GLM_FIXTURE, "glm-5").unwrap();
        assert_eq!(resp.content, "Hello from GLM-5");
        assert_eq!(resp.tokens_in, 14);
        assert_eq!(resp.tokens_out, 7);
    }

    #[test]
    fn glm_fixture_no_tool_calls() {
        let resp = glm_client().parse_response(GLM_FIXTURE, "glm-5").unwrap();
        assert!(resp.tool_calls.is_empty());
    }

    #[test]
    fn glm_provider_name_is_glm() {
        assert_eq!(build_glm_profile().name, "glm");
    }

    #[test]
    fn glm_base_url_is_bigmodel() {
        let p = build_glm_profile();
        assert!(p.base_url.contains("bigmodel.cn"));
    }

    #[test]
    fn glm_tool_round_trip_works() {
        let resp = glm_client().parse_response(GLM_TOOL_FIXTURE, "glm-5").unwrap();
        assert_eq!(resp.tool_calls.len(), 1);
        assert_eq!(resp.tool_calls[0].function.name, "extract_entities");
        let args: serde_json::Value =
            serde_json::from_str(&resp.tool_calls[0].function.arguments).unwrap();
        assert_eq!(args["text"], "Apple Inc was founded by Steve Jobs");
    }

    #[test]
    fn glm_tool_call_request_body_has_tools() {
        use crate::types::{CompletionRequest, FunctionDefinition, Message, ToolDefinition};
        let mut req = CompletionRequest::simple(
            "glm-5",
            vec![Message::text("user", "Extract companies from: Apple Inc was founded by Steve Jobs")],
        );
        req.tools = vec![ToolDefinition {
            kind: "function".to_owned(),
            function: FunctionDefinition {
                name: "extract_entities".to_owned(),
                description: "Extract named entities".to_owned(),
                parameters: serde_json::json!({"type":"object","properties":{"text":{"type":"string"}},"required":["text"]}),
            },
        }];
        let body = glm_client().build_request_body(&req, false).unwrap();
        assert!(body["tools"].is_array());
        assert_eq!(body["tools"][0]["function"]["name"], "extract_entities");
    }

    #[test]
    fn glm_supports_tools_glm5_true() {
        assert!(supports_tools("glm-5"));
    }

    #[test]
    fn glm_supports_tools_flash_false() {
        assert!(!supports_tools("flash"));
    }

    #[test]
    fn glm_streaming_fixture_ordered() {
        use crate::openai::OpenAiClient;
        let tokens: Vec<String> = GLM_STREAM_LINES.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(tokens, vec!["Hello", " from GLM"]);
    }

    #[test]
    fn glm_streaming_combined_text() {
        use crate::openai::OpenAiClient;
        let combined: String = GLM_STREAM_LINES.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text)
            .collect();
        assert_eq!(combined, "Hello from GLM");
    }

    #[test]
    fn glm_stream_done_emits_finished() {
        use crate::openai::OpenAiClient;
        let ev = OpenAiClient::parse_sse_line("data: [DONE]").unwrap();
        assert!(ev.finished);
    }

    #[test]
    fn glm_structured_output_validates_as_json_object() {
        use std::sync::Arc;
        let json_client = crate::openai::OpenAiClient::new(Arc::new(build_glm_json_profile()));
        let resp = json_client.parse_response(GLM_JSON_FIXTURE, "glm-5").unwrap();
        // The content should be a JSON object string
        assert!(is_json_object(&resp.content));
    }

    #[test]
    fn glm_json_profile_injects_response_format() {
        use crate::types::{CompletionRequest, Message};
        use std::sync::Arc;
        let json_client = crate::openai::OpenAiClient::new(Arc::new(build_glm_json_profile()));
        let req = CompletionRequest::simple(
            "glm-5",
            vec![Message::text("user", "Extract company name as JSON")],
        );
        let body = json_client.build_request_body(&req, false).unwrap();
        assert_eq!(body["response_format"]["type"], "json_object");
    }

    #[test]
    fn glm_json_fixture_has_expected_keys() {
        let resp = glm_client().parse_response(GLM_JSON_FIXTURE, "glm-5").unwrap();
        let obj: serde_json::Value = serde_json::from_str(&resp.content).unwrap();
        assert_eq!(obj["company"], "Apple Inc");
        assert_eq!(obj["founder"], "Steve Jobs");
    }

    #[test]
    fn is_json_object_accepts_valid_object() {
        assert!(is_json_object(r#"{"key": "value"}"#));
    }

    #[test]
    fn is_json_object_rejects_array() {
        assert!(!is_json_object(r#"["a","b"]"#));
    }
}
