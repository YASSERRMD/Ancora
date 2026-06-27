use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Anthropic provider profile.
///
/// Reads the API key from `ANTHROPIC_API_KEY` at call time.
/// The `anthropic-version` header is pinned to `2023-06-01` via `extra_headers`.
pub fn build_anthropic_profile() -> ProviderProfile {
    ProviderProfile::new(
        "anthropic",
        "https://api.anthropic.com",
        AuthStrategy::HeaderKey {
            header: "x-api-key".to_owned(),
            env_var: "ANTHROPIC_API_KEY".to_owned(),
        },
    )
    .with_chat_path("/v1/messages")
    .with_extra_header("anthropic-version", "2023-06-01")
    .add_model(
        ModelMeta::new("claude-opus-4-8", 200_000)
            .with_pricing(15.0, 75.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("claude-sonnet-4-6", 200_000)
            .with_pricing(3.0, 15.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("claude-haiku-4-5", 200_000)
            .with_pricing(0.80, 4.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_alias("claude-3-5-sonnet", "claude-sonnet-4-6")
    .add_alias("claude-3-5-haiku", "claude-haiku-4-5")
    .add_alias("claude-opus", "claude-opus-4-8")
}
