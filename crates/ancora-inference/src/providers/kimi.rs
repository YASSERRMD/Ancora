use crate::provider::{AuthStrategy, ProviderProfile};

/// International endpoint for Kimi (Moonshot AI).
pub const KIMI_URL_INTERNATIONAL: &str = "https://api.moonshot.ai/v1";

/// Build the Kimi (Moonshot AI) international provider profile.
///
/// Routes to the international endpoint. Auth is read from `MOONSHOT_API_KEY`.
pub fn build_kimi_profile() -> ProviderProfile {
    ProviderProfile::new(
        "kimi",
        KIMI_URL_INTERNATIONAL,
        AuthStrategy::BearerToken { env_var: "MOONSHOT_API_KEY".to_owned() },
    )
}
