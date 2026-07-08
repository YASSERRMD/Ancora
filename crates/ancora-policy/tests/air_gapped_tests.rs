/// Air-gapped egress policy tests.
///
/// An air-gapped policy must block every outbound call unconditionally,
/// including calls to endpoints that are listed in `allowed_endpoints`.
/// This prevents accidental data exfiltration from isolated deployments.
use ancora_policy::{error::PolicyError, eval::check_endpoint, policy::Policy};

const ANY_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const INTERNAL: &str = "https://internal.corp.example/api";

#[test]
fn air_gapped_blocks_all_egress() {
    let policy = Policy::new().air_gapped();
    let err = check_endpoint(&policy, ANY_ENDPOINT).unwrap_err();
    assert!(
        matches!(err, PolicyError::EgressBlocked(_)),
        "expected EgressBlocked, got: {err}"
    );
}

#[test]
fn air_gapped_blocks_internal_endpoints_too() {
    let policy = Policy::new().air_gapped();
    let err = check_endpoint(&policy, INTERNAL).unwrap_err();
    assert!(matches!(err, PolicyError::EgressBlocked(_)));
}

#[test]
fn air_gapped_overrides_allow_endpoint() {
    let policy = Policy::new().air_gapped().allow_endpoint(ANY_ENDPOINT);
    let err = check_endpoint(&policy, ANY_ENDPOINT).unwrap_err();
    assert!(
        matches!(err, PolicyError::EgressBlocked(_)),
        "allow_endpoint must not override air_gapped"
    );
}

#[test]
fn air_gapped_blocks_localhost() {
    let policy = Policy::new().air_gapped();
    let err = check_endpoint(&policy, "http://127.0.0.1:11434/api/generate").unwrap_err();
    assert!(matches!(err, PolicyError::EgressBlocked(_)));
}

#[test]
fn open_policy_allows_all_when_list_is_empty() {
    let policy = Policy::new();
    assert!(check_endpoint(&policy, ANY_ENDPOINT).is_ok());
}

#[test]
fn allow_list_policy_permits_prefix_match() {
    let policy = Policy::new().allow_endpoint("https://api.openai.com");
    assert!(check_endpoint(&policy, ANY_ENDPOINT).is_ok());
}

#[test]
fn allow_list_policy_blocks_unmatched_endpoint() {
    let policy = Policy::new().allow_endpoint("https://eu.api.example.com");
    let err = check_endpoint(&policy, ANY_ENDPOINT).unwrap_err();
    assert!(matches!(err, PolicyError::ResidencyViolation(_)));
}

#[test]
fn error_message_contains_blocked_endpoint() {
    let policy = Policy::new().air_gapped();
    let err = check_endpoint(&policy, ANY_ENDPOINT).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains(ANY_ENDPOINT),
        "error must name the blocked endpoint, got: {msg}"
    );
}
