use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// MiniMax international API endpoint.
pub const MINIMAX_URL: &str = "https://api.minimaxi.com/v1";

/// Build the MiniMax provider profile.
///
/// Uses the international endpoint at `api.minimaxi.com`. Auth is read
/// from `MINIMAX_API_KEY`.
pub fn build_minimax_profile() -> ProviderProfile {
    ProviderProfile::new(
        "minimax",
        MINIMAX_URL,
        AuthStrategy::BearerToken { env_var: "MINIMAX_API_KEY".to_owned() },
    )
    // MiniMax-Text-01 -- 1M context window, flagship text model
    .add_model(
        ModelMeta::new("MiniMax-Text-01", 1_000_000)
            .with_pricing(0.20, 1.10)
            .with_tools()
            .with_streaming(),
    )
    // MiniMax-VL-01 -- vision-language, 1M context
    .add_model(
        ModelMeta::new("MiniMax-VL-01", 1_000_000)
            .with_pricing(0.80, 4.50)
            .with_vision()
            .with_streaming(),
    )
    // MiniMax M2 -- latest reasoning model
    .add_model(
        ModelMeta::new("MiniMax-M2", 131_072)
            .with_pricing(0.15, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("text-01", "MiniMax-Text-01")
    .add_alias("vl-01", "MiniMax-VL-01")
    .add_alias("m2", "MiniMax-M2")
}

/// Return `true` if the model supports tool/function calls.
pub fn supports_tools(model_id: &str) -> bool {
    let p = build_minimax_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog.get(canonical).map_or(false, |m| m.capabilities.tools)
}

/// Parse a single SSE line from a MiniMax streaming response.
///
/// MiniMax uses the standard OpenAI SSE format.
pub fn parse_stream_line(line: &str) -> Option<crate::types::TokenEvent> {
    crate::openai::OpenAiClient::parse_sse_line(line)
}

/// Return `true` if the model supports vision (image) input.
pub fn supports_vision(model_id: &str) -> bool {
    let p = build_minimax_profile();
    let canonical = p.resolve_model_id(model_id);
    p.model_catalog.get(canonical).map_or(false, |m| m.capabilities.vision)
}
