use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Cohere provider profile.
///
/// Cohere's Chat API uses a distinct wire format (message/chat_history/preamble).
/// Use with `CohereClient` from `adapters::cohere`.
/// Reads the API key from `CO_API_KEY` at call time.
pub fn build_cohere_profile() -> ProviderProfile {
    ProviderProfile::new(
        "cohere",
        "https://api.cohere.ai",
        AuthStrategy::BearerToken {
            env_var: "CO_API_KEY".to_owned(),
        },
    )
    .with_chat_path("/v1/chat")
    .add_model(
        ModelMeta::new("command-r-plus", 128_000)
            .with_pricing(2.50, 10.0)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("command-r", 128_000)
            .with_pricing(0.15, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("command", 4_096)
            .with_pricing(1.0, 2.0)
            .with_streaming(),
    )
    .add_alias("command-r-plus-latest", "command-r-plus")
    .add_alias("command-r-latest", "command-r")
}
