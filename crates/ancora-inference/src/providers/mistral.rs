use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Mistral AI provider profile.
///
/// Mistral's API is OpenAI-compatible; use it with `OpenAiClient`.
/// Reads the API key from `MISTRAL_API_KEY` at call time.
pub fn build_mistral_profile() -> ProviderProfile {
    ProviderProfile::new(
        "mistral",
        "https://api.mistral.ai",
        AuthStrategy::BearerToken { env_var: "MISTRAL_API_KEY".to_owned() },
    )
    .add_model(
        ModelMeta::new("mistral-large-latest", 128_000)
            .with_pricing(2.0, 6.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("mistral-small-latest", 32_000)
            .with_pricing(0.20, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("open-mistral-7b", 32_000)
            .with_pricing(0.25, 0.25)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("codestral-latest", 32_000)
            .with_pricing(0.20, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("mistral-large", "mistral-large-latest")
    .add_alias("mistral-small", "mistral-small-latest")
    .add_alias("codestral", "codestral-latest")
}
