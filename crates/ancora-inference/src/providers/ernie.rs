use crate::provider::{AuthStrategy, ProviderProfile};

/// Baidu ERNIE OpenAI-compatible endpoint.
pub const ERNIE_URL: &str = "https://qianfan.baidubce.com/v2";

/// Build the Baidu ERNIE (Qianfan) provider profile.
///
/// Uses the Qianfan OpenAI-compatible endpoint. Auth is read from
/// `ERNIE_API_KEY`. The older OAuth flow (client_id + client_secret ->
/// access_token) is documented in `ernie_oauth_note` below.
pub fn build_ernie_profile() -> ProviderProfile {
    ProviderProfile::new(
        "ernie",
        ERNIE_URL,
        AuthStrategy::BearerToken { env_var: "ERNIE_API_KEY".to_owned() },
    )
}
