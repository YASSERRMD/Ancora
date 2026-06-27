use crate::provider::ProviderProfile;

/// Per-host rate-limit metadata.
///
/// Throughput hosts expose different rate-limit tiers depending on the model.
/// This struct captures the publicly documented limits so they can be checked
/// in tests and surfaced in health probes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimitMeta {
    /// Requests per minute for the default (free) tier.
    pub requests_per_minute: u32,
    /// Tokens per minute for the default tier.
    pub tokens_per_minute: u32,
    /// Whether the host returns a `Retry-After` header on 429 responses.
    pub retry_after_header: bool,
}

/// Documented rate limits for the three throughput hosts (free/default tier).
pub fn rate_limit_meta(provider_name: &str) -> Option<RateLimitMeta> {
    match provider_name {
        "groq" => Some(RateLimitMeta {
            requests_per_minute: 30,
            tokens_per_minute: 6_000,
            retry_after_header: true,
        }),
        "together" => Some(RateLimitMeta {
            requests_per_minute: 60,
            tokens_per_minute: 200_000,
            retry_after_header: false,
        }),
        "fireworks" => Some(RateLimitMeta {
            requests_per_minute: 600,
            tokens_per_minute: 1_000_000,
            retry_after_header: false,
        }),
        _ => None,
    }
}

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

    #[test]
    fn groq_rate_limit_meta_known() {
        let meta = rate_limit_meta("groq").unwrap();
        assert!(meta.requests_per_minute > 0);
        assert!(meta.tokens_per_minute > 0);
    }

    #[test]
    fn together_rate_limit_meta_known() {
        let meta = rate_limit_meta("together").unwrap();
        assert!(meta.requests_per_minute > 0);
    }

    #[test]
    fn fireworks_rate_limit_meta_known() {
        let meta = rate_limit_meta("fireworks").unwrap();
        assert!(meta.tokens_per_minute > 0);
    }

    #[test]
    fn groq_retry_after_header_present() {
        let meta = rate_limit_meta("groq").unwrap();
        assert!(meta.retry_after_header);
    }

    #[test]
    fn unknown_provider_has_no_rate_limit_meta() {
        assert!(rate_limit_meta("unknown-host").is_none());
    }

    #[test]
    fn groq_backoff_recommended_from_retry_after_header() {
        let meta = rate_limit_meta("groq").unwrap();
        // Groq provides Retry-After; backoff logic should honor it
        assert!(
            meta.retry_after_header,
            "groq 429 responses carry Retry-After"
        );
    }

    #[test]
    fn together_backoff_uses_exponential_without_header() {
        let meta = rate_limit_meta("together").unwrap();
        // Together does not provide Retry-After; caller should use exponential backoff
        assert!(
            !meta.retry_after_header,
            "together 429 lacks Retry-After; use exponential backoff"
        );
    }

    #[test]
    fn fireworks_backoff_uses_exponential_without_header() {
        let meta = rate_limit_meta("fireworks").unwrap();
        assert!(
            !meta.retry_after_header,
            "fireworks 429 lacks Retry-After; use exponential backoff"
        );
    }

    #[test]
    fn groq_rpm_lower_than_fireworks() {
        let groq = rate_limit_meta("groq").unwrap();
        let fw = rate_limit_meta("fireworks").unwrap();
        assert!(
            groq.requests_per_minute < fw.requests_per_minute,
            "groq free tier is more restrictive than fireworks"
        );
    }
}
