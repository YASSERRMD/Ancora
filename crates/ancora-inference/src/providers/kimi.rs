use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// International endpoint for Kimi (Moonshot AI).
pub const KIMI_URL_INTERNATIONAL: &str = "https://api.moonshot.ai/v1";

/// Domestic CN endpoint for Kimi.
pub const KIMI_URL_DOMESTIC: &str = "https://api.moonshot.cn/v1";

/// Build the Kimi (Moonshot AI) international provider profile.
///
/// Routes to the international endpoint. Auth is read from `MOONSHOT_API_KEY`.
pub fn build_kimi_profile() -> ProviderProfile {
    ProviderProfile::new(
        "kimi",
        KIMI_URL_INTERNATIONAL,
        AuthStrategy::BearerToken { env_var: "MOONSHOT_API_KEY".to_owned() },
    )
    .add_region("intl", KIMI_URL_INTERNATIONAL)
    .add_region("cn", KIMI_URL_DOMESTIC)
    // Kimi K2 -- most capable, long context, tools (flagship)
    .add_model(
        ModelMeta::new("kimi-k2", 131_072)
            .with_pricing(0.60, 2.50)
            .with_tools()
            .with_streaming(),
    )
    // Kimi K2 Turbo -- faster, same context
    .add_model(
        ModelMeta::new("kimi-k2-turbo", 131_072)
            .with_pricing(0.20, 0.80)
            .with_tools()
            .with_streaming(),
    )
    // Classic moonshot models (still in use)
    .add_model(
        ModelMeta::new("moonshot-v1-128k", 131_072)
            .with_pricing(1.00, 1.00)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("moonshot-v1-32k", 32_768)
            .with_pricing(0.24, 0.24)
            .with_streaming(),
    )
    .add_model(
        ModelMeta::new("moonshot-v1-8k", 8_192)
            .with_pricing(0.12, 0.12)
            .with_streaming(),
    )
    // Kimi Moonlight -- very long context, designed for document analysis
    .add_model(
        ModelMeta::new("moonshot-v1-long", 1_000_000)
            .with_pricing(1.50, 1.50)
            .with_streaming(),
    )
    .add_alias("k2", "kimi-k2")
    .add_alias("k2-turbo", "kimi-k2-turbo")
    .add_alias("128k", "moonshot-v1-128k")
    .add_alias("32k", "moonshot-v1-32k")
    .add_alias("8k", "moonshot-v1-8k")
    .add_alias("long", "moonshot-v1-long")
}

/// Build the Kimi domestic (China) provider profile.
///
/// Routes to `api.moonshot.cn`. Use when running workloads inside China or
/// where international routing is unavailable. Same API key.
pub fn build_kimi_domestic_profile() -> ProviderProfile {
    ProviderProfile::new(
        "kimi-cn",
        KIMI_URL_DOMESTIC,
        AuthStrategy::BearerToken { env_var: "MOONSHOT_API_KEY".to_owned() },
    )
}
