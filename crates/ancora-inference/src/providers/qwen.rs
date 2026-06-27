use crate::provider::{AuthStrategy, ModelMeta, ProviderProfile};

/// DashScope international endpoint (Singapore).
pub const QWEN_URL_SINGAPORE: &str =
    "https://dashscope-intl.aliyuncs.com/compatible-mode";

/// Build the Alibaba Qwen (DashScope) provider profile.
///
/// Uses the Singapore international endpoint by default. Auth is read from
/// `DASHSCOPE_API_KEY`. Call `.base_url_for_region(Some("eu"))` on the
/// returned profile to direct requests to a different region.
pub fn build_qwen_profile() -> ProviderProfile {
    ProviderProfile::new(
        "qwen",
        QWEN_URL_SINGAPORE,
        AuthStrategy::BearerToken { env_var: "DASHSCOPE_API_KEY".to_owned() },
    )
}
