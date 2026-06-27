use std::time::Duration;

/// Errors returned by inference adapter implementations.
#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("model refused: {0}")]
    Refused(String),
    #[error("http error {status}: {body}")]
    Http { status: u16, body: String },
    #[error("parse error: {0}")]
    Parse(String),
    #[error("endpoint unreachable: {0}")]
    Unreachable(String),
    #[error("internal error: {0}")]
    Internal(String),
    /// Provider rate-limit with an optional retry-after hint parsed from the response header.
    #[error("rate limited{}", .retry_after.map(|d| format!(" (retry after {}s)", d.as_secs())).unwrap_or_default())]
    RateLimit { retry_after: Option<Duration> },
    /// Provider rejected the credentials.
    #[error("auth rejected: {0}")]
    AuthRejected(String),
    /// A required credential was absent from the environment.
    #[error("missing credential: {0}")]
    MissingCredential(String),
}

impl InferenceError {
    /// Parse a `Retry-After` header value (integer seconds) into a `Duration`.
    pub fn parse_retry_after(value: &str) -> Option<Duration> {
        value.trim().parse::<u64>().ok().map(Duration::from_secs)
    }

    /// Normalise a raw HTTP status and body into a structured `InferenceError`.
    ///
    /// Covers the common patterns across providers:
    /// - 401/403 -> `AuthRejected`
    /// - 429     -> `RateLimit` with parsed `Retry-After` if present
    /// - other   -> `Http`
    pub fn from_http(status: u16, body: &str, retry_after: Option<&str>) -> Self {
        match status {
            401 | 403 => Self::AuthRejected(body.to_owned()),
            429 => Self::RateLimit {
                retry_after: retry_after.and_then(Self::parse_retry_after),
            },
            _ => Self::Http { status, body: body.to_owned() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_retry_after_integer_seconds() {
        let d = InferenceError::parse_retry_after("30").unwrap();
        assert_eq!(d.as_secs(), 30);
    }

    #[test]
    fn parse_retry_after_with_whitespace() {
        let d = InferenceError::parse_retry_after("  60  ").unwrap();
        assert_eq!(d.as_secs(), 60);
    }

    #[test]
    fn parse_retry_after_invalid_returns_none() {
        assert!(InferenceError::parse_retry_after("Thu, 01 Jan").is_none());
    }

    #[test]
    fn from_http_401_produces_auth_rejected() {
        let e = InferenceError::from_http(401, "bad key", None);
        assert!(matches!(e, InferenceError::AuthRejected(_)));
    }

    #[test]
    fn from_http_403_produces_auth_rejected() {
        let e = InferenceError::from_http(403, "forbidden", None);
        assert!(matches!(e, InferenceError::AuthRejected(_)));
    }

    #[test]
    fn from_http_429_produces_rate_limit_without_header() {
        let e = InferenceError::from_http(429, "too many", None);
        assert!(matches!(e, InferenceError::RateLimit { retry_after: None }));
    }

    #[test]
    fn from_http_429_with_retry_after_parses_duration() {
        let e = InferenceError::from_http(429, "too many", Some("45"));
        match e {
            InferenceError::RateLimit { retry_after: Some(d) } => assert_eq!(d.as_secs(), 45),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn from_http_500_produces_http_variant() {
        let e = InferenceError::from_http(500, "server error", None);
        assert!(matches!(e, InferenceError::Http { status: 500, .. }));
    }
}
