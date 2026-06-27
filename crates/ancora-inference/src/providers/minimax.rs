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
    // MiniMax-Text-01 -- 1M context window, flagship text model
    .add_model(
        ModelMeta::new("MiniMax-Text-01", 1_000_000)
            .with_pricing(0.20, 1.10)
            .with_tools()
            .with_streaming(),
    )
    // MiniMax-VL-01 -- vision-language, 1M context
    .add_model(
        ModelMeta::new("MiniMax-VL-01", 1_000_000)
            .with_pricing(0.80, 4.50)
            .with_vision()
            .with_streaming(),
    )
    // MiniMax M2 -- latest reasoning model
    .add_model(
        ModelMeta::new("MiniMax-M2", 131_072)
            .with_pricing(0.15, 0.60)
            .with_tools()
            .with_streaming(),
    )
    .add_alias("text-01", "MiniMax-Text-01")
    .add_alias("vl-01", "MiniMax-VL-01")
    .add_alias("m2", "MiniMax-M2")
}
