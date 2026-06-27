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
    // Qwen Plus -- balanced capability/cost
    .add_model(
        ModelMeta::new("qwen-plus", 131_072)
            .with_pricing(0.40, 1.20)
            .with_tools()
            .with_streaming(),
    )
    // Qwen Turbo -- fastest, lowest cost
    .add_model(
        ModelMeta::new("qwen-turbo", 131_072)
            .with_pricing(0.05, 0.10)
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
