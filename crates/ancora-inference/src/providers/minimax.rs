use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// MiniMax international API endpoint.
pub const MINIMAX_URL: &str = "https://api.minimaxi.com/v1";

/// Build the MiniMax provider profile.
///
/// Uses the international endpoint at `api.minimaxi.com`. Auth is read
/// from `MINIMAX_API_KEY`.
pub fn build_minimax_profile() -> ProviderProfile {
    ProviderProfile::new(
        "minimax",
        MINIMAX_URL,
        AuthStrategy::BearerToken { env_var: "MINIMAX_API_KEY".to_owned() },
    )
}
