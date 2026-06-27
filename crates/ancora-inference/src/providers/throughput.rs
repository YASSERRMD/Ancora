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

/// Result from a host health probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// Profile is structurally valid and credentials are present in the environment.
    Ready,
    /// Credential environment variable is missing.
    MissingCredential(String),
    /// Profile is structurally invalid (e.g. no models registered, no HTTPS).
    InvalidProfile(String),
}

/// Check whether a throughput host profile is ready to serve requests.
///
/// This is a structural health probe -- it does NOT make a live HTTP call.
/// It verifies:
/// 1. The base URL uses HTTPS.
/// 2. At least one model is registered in the catalog.
/// 3. The required API key environment variable is set.
pub fn health_probe(profile: &ProviderProfile) -> HealthStatus {
    if !profile.base_url.starts_with("https://") {
        return HealthStatus::InvalidProfile("base_url must use HTTPS".to_owned());
    }
    if profile.model_catalog.is_empty() {
        return HealthStatus::InvalidProfile("no models registered".to_owned());
    }
    match profile.auth.resolve() {
        Some(_) => HealthStatus::Ready,
        None => {
            let env_var = match &profile.auth {
                crate::provider::AuthStrategy::BearerToken { env_var } => env_var.clone(),
                crate::provider::AuthStrategy::HeaderKey { env_var, .. } => env_var.clone(),
                crate::provider::AuthStrategy::QueryParam { env_var, .. } => env_var.clone(),
                crate::provider::AuthStrategy::None => return HealthStatus::Ready,
            };
            HealthStatus::MissingCredential(env_var)
        }
    }
}

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

    #[test]
    fn health_probe_missing_credential_for_groq() {
        use crate::providers::groq::build_groq_profile;
        // GROQ_API_KEY is not set in test environment
        if std::env::var("GROQ_API_KEY").is_ok() {
            return; // skip if key happens to be in environment
        }
        let p = build_groq_profile();
        let status = health_probe(&p);
        assert_eq!(status, HealthStatus::MissingCredential("GROQ_API_KEY".to_owned()));
    }

    #[test]
    fn health_probe_invalid_profile_no_https() {
        use crate::provider::AuthStrategy;
        let p = ProviderProfile::new("test", "http://insecure.example.com", AuthStrategy::None);
        let status = health_probe(&p);
        assert_eq!(status, HealthStatus::InvalidProfile("base_url must use HTTPS".to_owned()));
    }

    #[test]
    fn health_probe_invalid_profile_no_models() {
        use crate::provider::AuthStrategy;
        let p = ProviderProfile::new("test", "https://example.com", AuthStrategy::None);
        let status = health_probe(&p);
        assert_eq!(status, HealthStatus::InvalidProfile("no models registered".to_owned()));
    }

    #[test]
    fn groq_streaming_ordered() {
        use crate::openai::OpenAiClient;
        use crate::providers::groq::build_groq_profile;
        use std::sync::Arc;
        let _client = OpenAiClient::new(Arc::new(build_groq_profile()));
        let lines = [
            r#"data: {"choices":[{"delta":{"content":"one"},"finish_reason":null}]}"#,
            r#"data: {"choices":[{"delta":{"content":" two"},"finish_reason":null}]}"#,
            r#"data: {"choices":[{"delta":{"content":" three"},"finish_reason":"stop"}]}"#,
            "data: [DONE]",
        ];
        let tokens: Vec<String> = lines.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(tokens, vec!["one", " two", " three"]);
    }

    #[test]
    fn together_streaming_ordered() {
        use crate::openai::OpenAiClient;
        let lines = [
            r#"data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#,
            r#"data: {"choices":[{"delta":{"content":" Together"},"finish_reason":"stop"}]}"#,
            "data: [DONE]",
        ];
        let tokens: Vec<String> = lines.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(tokens, vec!["Hello", " Together"]);
    }

    #[test]
    fn fireworks_streaming_ordered() {
        use crate::openai::OpenAiClient;
        let lines = [
            r#"data: {"choices":[{"delta":{"content":"Spark"},"finish_reason":null}]}"#,
            r#"data: {"choices":[{"delta":{"content":"s"},"finish_reason":"stop"}]}"#,
            "data: [DONE]",
        ];
        let tokens: Vec<String> = lines.iter()
            .filter_map(|l| OpenAiClient::parse_sse_line(l))
            .filter(|ev| !ev.text.is_empty())
            .map(|ev| ev.text.clone())
            .collect();
        assert_eq!(tokens, vec!["Spark", "s"]);
    }
}
