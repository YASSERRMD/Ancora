use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the Groq provider profile.
///
/// Groq exposes an OpenAI-compatible endpoint. Use with `OpenAiClient`.
/// Reads the API key from `GROQ_API_KEY` at call time.
pub fn build_groq_profile() -> ProviderProfile {
    ProviderProfile::new(
        "groq",
        "https://api.groq.com/openai",
        AuthStrategy::BearerToken { env_var: "GROQ_API_KEY".to_owned() },
    )
}
