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
