use crate::provider::ProviderProfile;

/// Common characteristics shared by high-throughput inference hosts.
///
/// Groq, Together AI, and Fireworks AI are all:
/// - OpenAI wire-compatible (use `OpenAiClient` without modification)
/// - Bearer-token authenticated
/// - Optimized for open-source model serving at high RPS
///
/// This module provides shared metadata accessors used by tests and
/// diagnostics to verify throughput-host profiles conform to expectations.

/// Verify that a provider profile looks like a throughput host.
///
/// Returns `true` when:
/// - The base URL is HTTPS
/// - The auth strategy is a BearerToken
/// - At least one model supports streaming
pub fn is_throughput_host(profile: &ProviderProfile) -> bool {
    let has_https = profile.base_url.starts_with("https://");
    let has_streaming = profile.model_catalog.values().any(|m| m.capabilities.streaming);
    let has_bearer = matches!(
        profile.auth,
        crate::provider::AuthStrategy::BearerToken { .. }
    );
    has_https && has_streaming && has_bearer
}

/// Collect the model IDs that support tool use in a profile.
pub fn tool_capable_model_ids(profile: &ProviderProfile) -> Vec<String> {
    let mut ids: Vec<String> = profile
        .model_catalog
        .iter()
        .filter(|(_, m)| m.capabilities.tools)
        .map(|(id, _)| id.clone())
        .collect();
    ids.sort();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groq_is_throughput_host() {
        use crate::providers::groq::build_groq_profile;
        assert!(is_throughput_host(&build_groq_profile()));
    }

    #[test]
    fn together_is_throughput_host() {
        use crate::providers::together::build_together_profile;
        assert!(is_throughput_host(&build_together_profile()));
    }

    #[test]
    fn fireworks_is_throughput_host() {
        use crate::providers::fireworks::build_fireworks_profile;
        assert!(is_throughput_host(&build_fireworks_profile()));
    }

    #[test]
    fn groq_has_tool_capable_models() {
        use crate::providers::groq::build_groq_profile;
        let p = build_groq_profile();
        let ids = tool_capable_model_ids(&p);
        assert!(!ids.is_empty());
    }

    #[test]
    fn together_has_tool_capable_models() {
        use crate::providers::together::build_together_profile;
        let p = build_together_profile();
        let ids = tool_capable_model_ids(&p);
        assert!(!ids.is_empty());
    }

    #[test]
    fn fireworks_has_tool_capable_models() {
        use crate::providers::fireworks::build_fireworks_profile;
        let p = build_fireworks_profile();
        let ids = tool_capable_model_ids(&p);
        assert!(!ids.is_empty());
    }
}
