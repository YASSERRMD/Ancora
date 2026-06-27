use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Google Gemini provider profile.
///
/// Reads the API key from `GOOGLE_API_KEY` at call time and passes it as
/// the `key` query parameter on every request. The URL template is
/// `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}`.
/// The `GeminiClient` adapter constructs the model-specific path at call time.
pub fn build_gemini_profile() -> ProviderProfile {
    ProviderProfile::new(
        "gemini",
        "https://generativelanguage.googleapis.com",
        AuthStrategy::QueryParam {
            param: "key".to_owned(),
            env_var: "GOOGLE_API_KEY".to_owned(),
        },
    )
    .add_model(
        ModelMeta::new("gemini-2.0-flash", 1_000_000)
            .with_pricing(0.10, 0.40)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("gemini-2.5-pro", 2_000_000)
            .with_pricing(1.25, 10.0)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("gemini-1.5-flash", 1_000_000)
            .with_pricing(0.075, 0.30)
            .with_tools()
            .with_vision()
            .with_streaming(),
    )
    .add_alias("gemini-flash", "gemini-2.0-flash")
    .add_alias("gemini-pro", "gemini-2.5-pro")
}
