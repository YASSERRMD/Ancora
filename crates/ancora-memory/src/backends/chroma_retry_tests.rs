/// Chroma retry policy and error classification tests -- all offline.

#[cfg(test)]
mod chroma_retry_tests {
    use crate::backends::chroma::*;

    // ---- error variants -------------------------------------------------

    #[test]
    fn error_404_is_not_found() {
        let e = ChromaError::from_response(404, "collection not found");
        assert!(matches!(e, ChromaError::NotFound(_)));
    }

    #[test]
    fn error_409_is_already_exists() {
        let e = ChromaError::from_response(409, "already exists");
        assert!(matches!(e, ChromaError::AlreadyExists(_)));
    }

    #[test]
    fn error_400_is_bad_request() {
        let e = ChromaError::from_response(400, "bad input");
        assert!(matches!(e, ChromaError::BadRequest(_)));
    }

    #[test]
    fn error_422_is_bad_request() {
        let e = ChromaError::from_response(422, "unprocessable");
        assert!(matches!(e, ChromaError::BadRequest(_)));
    }

    #[test]
    fn error_401_is_unauthorized() {
        let e = ChromaError::from_response(401, "unauthorized");
        assert!(matches!(e, ChromaError::Unauthorized));
    }

    #[test]
    fn error_403_is_unauthorized() {
        let e = ChromaError::from_response(403, "forbidden");
        assert!(matches!(e, ChromaError::Unauthorized));
    }

    #[test]
    fn error_500_is_internal_and_transient() {
        let e = ChromaError::from_response(500, "internal server error");
        assert!(matches!(e, ChromaError::InternalError(_)));
        assert!(e.is_transient(), "500 must be transient");
    }

    #[test]
    fn error_503_is_internal_and_transient() {
        let e = ChromaError::from_response(503, "service unavailable");
        assert!(e.is_transient(), "503 must be transient");
    }

    #[test]
    fn error_404_is_not_transient() {
        let e = ChromaError::from_response(404, "not found");
        assert!(!e.is_transient(), "404 must NOT be transient");
    }

    #[test]
    fn error_409_is_not_transient() {
        let e = ChromaError::from_response(409, "conflict");
        assert!(!e.is_transient(), "409 must NOT be transient");
    }

    #[test]
    fn error_422_is_not_transient() {
        let e = ChromaError::from_response(422, "bad input");
        assert!(!e.is_transient(), "422 must NOT be transient");
    }

    #[test]
    fn error_unknown_status_is_unknown_variant() {
        let e = ChromaError::from_response(418, "I'm a teapot");
        assert!(matches!(e, ChromaError::Unknown(418, _)));
    }

    // ---- retry delay ----------------------------------------------------

    #[test]
    fn retry_delay_increases_with_attempt() {
        let d0 = chroma_retry_delay_ms(0);
        let d1 = chroma_retry_delay_ms(1);
        assert!(d1 >= d0, "delay should not decrease: d0={d0} d1={d1}");
    }

    #[test]
    fn retry_delay_caps_below_reasonable_max() {
        assert!(chroma_retry_delay_ms(100) <= 10_000);
    }

    #[test]
    fn retry_delay_first_attempt_nonzero() {
        assert!(chroma_retry_delay_ms(0) > 0);
    }

    // ---- MAX_RETRIES ----------------------------------------------------

    #[test]
    fn max_retries_is_at_least_two() {
        assert!(MAX_RETRIES >= 2, "need at least 2 retries for transient errors");
    }

    // ---- URL builders ---------------------------------------------------

    #[test]
    fn collections_url_ends_with_collections() {
        let url = collections_url("http://localhost:8000", "default_tenant", "default_database");
        assert!(url.ends_with("/collections"), "url: {url}");
    }

    #[test]
    fn add_url_contains_collection_id() {
        let url = add_url("http://localhost:8000", "t", "db", "abc-123");
        assert!(url.contains("abc-123"), "url: {url}");
    }

    #[test]
    fn query_url_ends_with_query() {
        let url = query_url("http://localhost:8000", "t", "db", "col-id");
        assert!(url.ends_with("/query"), "url: {url}");
    }

    #[test]
    fn heartbeat_url_ends_with_heartbeat() {
        let url = heartbeat_url("http://localhost:8000");
        assert!(url.ends_with("/heartbeat"), "url: {url}");
    }
}
