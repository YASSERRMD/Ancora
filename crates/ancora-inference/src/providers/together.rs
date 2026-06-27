use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Together AI provider profile.
///
/// Together AI's inference API is OpenAI-compatible. Use with `OpenAiClient`.
/// Reads the API key from `TOGETHER_API_KEY` at call time.
pub fn build_together_profile() -> ProviderProfile {
    ProviderProfile::new(
        "together",
        "https://api.together.xyz",
        AuthStrategy::BearerToken { env_var: "TOGETHER_API_KEY".to_owned() },
    )
}
