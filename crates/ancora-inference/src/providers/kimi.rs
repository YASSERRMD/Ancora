use crate::provider::{AuthStrategy, ProviderProfile};

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
