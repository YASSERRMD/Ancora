use crate::network_policy::NetworkPolicy;

#[test]
fn plugin_network_blocked_by_default_deny() {
    let policy = NetworkPolicy::deny_all();
    // All outbound connections must be blocked when no rules exist and default is deny.
    assert!(!policy.permits("google.com", 443));
    assert!(!policy.permits("192.168.1.100", 8080));
    assert!(!policy.permits("localhost", 3000));
}

#[test]
fn specific_host_allowed_while_others_blocked() {
    let mut policy = NetworkPolicy::deny_all();
    policy.allow_host("api.trusted.example.com", Some(443));

    assert!(policy.permits("api.trusted.example.com", 443));
    assert!(!policy.permits("api.trusted.example.com", 80));
    assert!(!policy.permits("other.example.com", 443));
}

#[test]
fn blocked_host_denied_even_with_allow_default() {
    let mut policy = NetworkPolicy::allow_all();
    policy.deny_host("malicious.example.com", None);

    assert!(!policy.permits("malicious.example.com", 80));
    assert!(!policy.permits("malicious.example.com", 443));
    // Other hosts still allowed.
    assert!(policy.permits("safe.example.com", 443));
}

#[test]
fn deny_all_blocks_loopback() {
    let policy = NetworkPolicy::deny_all();
    assert!(!policy.permits("127.0.0.1", 4000));
    assert!(!policy.permits("::1", 4000));
}
