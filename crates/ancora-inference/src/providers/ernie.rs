use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

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

/// Return a note about the legacy Baidu OAuth auth flow.
///
/// The new Qianfan endpoint (`qianfan.baidubce.com/v2`) accepts API keys
/// directly. The legacy flow (used by older `aip.baidubce.com` endpoints)
/// requires exchanging a `client_id` and `client_secret` for a temporary
/// access_token via a separate HTTP call. This library uses the modern
/// API-key flow only.
pub fn ernie_oauth_note() -> &'static str {
    "Legacy flow: POST https://aip.baidubce.com/oauth/2.0/token \
     ?grant_type=client_credentials&client_id=<AK>&client_secret=<SK>. \
     Use the Qianfan API key flow instead."
}
