/// Pinecone error classification and retry policy tests -- all offline.

#[cfg(test)]
mod pinecone_retry_tests {
    use crate::backends::pinecone::*;

    // ---- error variants -------------------------------------------------

    #[test]
    fn error_404_is_not_found() {
        let e = PineconeError::from_response(404, "index not found");
        assert!(matches!(e, PineconeError::NotFound(_)));
    }

    #[test]
    fn error_409_is_already_exists() {
        let e = PineconeError::from_response(409, "index already exists");
        assert!(matches!(e, PineconeError::AlreadyExists(_)));
    }

    #[test]
    fn error_400_is_bad_request() {
        let e = PineconeError::from_response(400, "bad dimension");
        assert!(matches!(e, PineconeError::BadRequest(_)));
    }

    #[test]
    fn error_422_is_bad_request() {
        let e = PineconeError::from_response(422, "unprocessable");
        assert!(matches!(e, PineconeError::BadRequest(_)));
    }

    #[test]
    fn error_401_is_unauthorized() {
        let e = PineconeError::from_response(401, "invalid api key");
        assert!(matches!(e, PineconeError::Unauthorized));
    }

    #[test]
    fn error_403_is_unauthorized() {
        let e = PineconeError::from_response(403, "forbidden");
        assert!(matches!(e, PineconeError::Unauthorized));
    }

    #[test]
    fn error_429_is_rate_limited() {
        let e = PineconeError::from_response(429, "too many requests");
        assert!(matches!(e, PineconeError::RateLimited));
    }

    #[test]
    fn error_500_is_internal_error() {
        let e = PineconeError::from_response(500, "internal error");
        assert!(matches!(e, PineconeError::InternalError(_)));
    }

    #[test]
    fn error_unknown_status() {
        let e = PineconeError::from_response(418, "teapot");
        assert!(matches!(e, PineconeError::Unknown(418, _)));
    }

    // ---- transience ------------------------------------------------------

    #[test]
    fn internal_error_is_transient() {
        let e = PineconeError::from_response(500, "transient");
        assert!(e.is_transient());
    }

    #[test]
    fn rate_limited_is_transient() {
        let e = PineconeError::from_response(429, "rate limited");
        assert!(e.is_transient());
    }

    #[test]
    fn not_found_is_not_transient() {
        let e = PineconeError::from_response(404, "not found");
        assert!(!e.is_transient());
    }

    #[test]
    fn already_exists_is_not_transient() {
        let e = PineconeError::from_response(409, "conflict");
        assert!(!e.is_transient());
    }

    #[test]
    fn bad_request_is_not_transient() {
        let e = PineconeError::from_response(400, "bad");
        assert!(!e.is_transient());
    }

    #[test]
    fn unauthorized_is_not_transient() {
        let e = PineconeError::from_response(401, "unauth");
        assert!(!e.is_transient());
    }

    // ---- retry delay ----------------------------------------------------

    #[test]
    fn retry_delay_first_attempt_nonzero() {
        assert!(pinecone_retry_delay_ms(0) > 0);
    }

    #[test]
    fn retry_delay_increases_with_attempt() {
        let d0 = pinecone_retry_delay_ms(0);
        let d1 = pinecone_retry_delay_ms(1);
        assert!(d1 >= d0, "d0={d0} d1={d1}");
    }

    #[test]
    fn retry_delay_capped() {
        assert!(pinecone_retry_delay_ms(100) <= 10_000);
    }

    // ---- MAX_RETRIES -----------------------------------------------------

    #[test]
    fn max_retries_at_least_three() {
        assert!(MAX_RETRIES >= 3);
    }

    // ---- PineconeConfig -------------------------------------------------

    #[test]
    fn config_auth_header_contains_api_key() {
        use crate::backends::pinecone::PineconeConfig;
        let cfg = PineconeConfig {
            api_key: "test-key".to_owned(),
            environment: "us-east1-gcp".to_owned(),
            timeout_secs: 30,
        };
        let hdr = cfg.auth_header();
        assert!(hdr.contains("test-key"), "header: {hdr}");
    }

    #[test]
    fn config_controller_url_is_https() {
        use crate::backends::pinecone::PineconeConfig;
        let cfg = PineconeConfig {
            api_key: "k".to_owned(),
            environment: "us-east1-gcp".to_owned(),
            timeout_secs: 30,
        };
        assert!(
            cfg.controller_url().starts_with("https://"),
            "url: {}",
            cfg.controller_url()
        );
    }
}
