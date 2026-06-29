use crate::egress::{EdgeEgress, EgressPolicy};

#[test]
fn test_edge_egress_zero_by_default() {
    let egress = EdgeEgress::new();
    // By default, all egress is blocked.
    assert!(!egress.is_allowed("example.com", 443), "egress should be blocked by default");
    assert!(!egress.is_allowed("api.internal", 8080), "egress should be blocked by default");
}

#[test]
fn test_edge_egress_allow_explicit() {
    let mut egress = EdgeEgress::new();
    egress.allow("attestation-server.local", 8443);
    assert!(egress.is_allowed("attestation-server.local", 8443), "explicitly allowed endpoint should pass");
    assert!(!egress.is_allowed("other.com", 80), "non-allowed endpoint should still fail");
}

#[test]
fn test_edge_egress_policy_deny_all() {
    let egress = EdgeEgress::with_policy(EgressPolicy::DenyAll);
    assert!(!egress.is_allowed("anywhere.com", 443));
}

#[test]
fn test_edge_egress_count() {
    let mut egress = EdgeEgress::new();
    assert_eq!(egress.allowed_count(), 0);
    egress.allow("host-a", 1000);
    egress.allow("host-b", 2000);
    assert_eq!(egress.allowed_count(), 2);
}
