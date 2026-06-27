use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// Build the DeepSeek provider profile.
///
/// DeepSeek exposes an OpenAI-compatible API at `api.deepseek.com`.
/// Use with `OpenAiClient`. Reads the API key from `DEEPSEEK_API_KEY`.
///
/// Note: the direct endpoint routes through DeepSeek's CN-region
/// infrastructure. Use the self-host profile for non-CN residency.
pub fn build_deepseek_profile() -> ProviderProfile {
    ProviderProfile::new(
        "deepseek",
        "https://api.deepseek.com",
        AuthStrategy::BearerToken { env_var: "DEEPSEEK_API_KEY".to_owned() },
    )
}
