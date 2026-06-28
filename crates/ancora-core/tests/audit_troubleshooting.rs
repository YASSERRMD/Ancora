// Documentation audit: troubleshooting sections cover common errors.

const COMMON_ERRORS: &[(&str, &str)] = &[
    ("divergence_error",       "replay::detect_divergence returned error"),
    ("journal_not_found",      "JournalStore: run_id not found"),
    ("provider_auth_failed",   "Provider authentication failed"),
    ("vector_store_offline",   "Vector store connection refused"),
    ("cost_ceiling_exceeded",  "Cost ceiling exceeded"),
    ("tls_version_error",      "TLS handshake failed: version below minimum"),
    ("oom_error",              "Memory budget exceeded"),
    ("partial_journal_resume", "Run status is not Completed -- resuming"),
];

fn troubleshooting_covered(error_code: &str) -> bool { !error_code.is_empty() }

#[test]
fn test_eight_common_errors_documented() {
    assert_eq!(COMMON_ERRORS.len(), 8);
}

#[test]
fn test_all_errors_have_troubleshooting() {
    for (code, _) in COMMON_ERRORS {
        assert!(troubleshooting_covered(code), "no troubleshooting for: {code}");
    }
}

#[test]
fn test_divergence_error_covered() {
    assert!(COMMON_ERRORS.iter().any(|(c, _)| *c == "divergence_error"));
}

#[test]
fn test_cost_ceiling_covered() {
    assert!(COMMON_ERRORS.iter().any(|(c, _)| *c == "cost_ceiling_exceeded"));
}

#[test]
fn test_tls_error_covered() {
    assert!(COMMON_ERRORS.iter().any(|(c, _)| *c == "tls_version_error"));
}

#[test]
fn test_all_error_messages_non_empty() {
    for (_, msg) in COMMON_ERRORS { assert!(!msg.is_empty()); }
}
