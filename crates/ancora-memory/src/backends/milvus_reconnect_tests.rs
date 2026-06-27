/// Reconnect and retry-policy tests for the Milvus backend.
/// All offline -- verifies retry delay math and status classification.

#[cfg(test)]
mod milvus_reconnect_tests {
    use crate::backends::milvus::*;

    #[test]
    fn retry_delay_starts_at_150ms() {
        assert_eq!(milvus_retry_delay_ms(0), 150);
    }

    #[test]
    fn retry_delay_doubles_each_attempt() {
        assert_eq!(milvus_retry_delay_ms(1), 300);
        assert_eq!(milvus_retry_delay_ms(2), 600);
        assert_eq!(milvus_retry_delay_ms(3), 1_200);
    }

    #[test]
    fn retry_delay_caps_at_12s() {
        for attempt in 7..=20 {
            assert!(milvus_retry_delay_ms(attempt) <= 12_000, "attempt {attempt} exceeded cap");
        }
    }

    #[test]
    fn should_retry_429_rate_limited() {
        assert!(should_retry_status(429));
    }

    #[test]
    fn should_retry_503_unavailable() {
        assert!(should_retry_status(503));
    }

    #[test]
    fn should_not_retry_400_bad_request() {
        assert!(!should_retry_status(400));
    }

    #[test]
    fn should_not_retry_404_not_found() {
        assert!(!should_retry_status(404));
    }

    #[test]
    fn should_not_retry_409_conflict() {
        assert!(!should_retry_status(409));
    }

    #[test]
    fn internal_error_is_transient() {
        let err = MilvusError::InternalError("disk full".to_owned());
        assert!(err.is_transient());
    }

    #[test]
    fn overloaded_is_transient() {
        let err = MilvusError::Overloaded;
        assert!(err.is_transient());
    }

    #[test]
    fn not_found_is_not_transient() {
        let err = MilvusError::NotFound("x".to_owned());
        assert!(!err.is_transient());
    }

    #[test]
    fn max_retries_is_four() {
        assert_eq!(MAX_RETRIES, 4);
    }
}
