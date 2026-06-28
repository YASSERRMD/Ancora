use crate::policy::ZeroTrustPolicy;

#[test]
fn deny_multiple_resources() {
    let p = ZeroTrustPolicy::new("t1")
        .deny_resource("admin/secrets")
        .deny_resource("internal/keys");
    assert!(p.resource_denied("admin/secrets"));
    assert!(p.resource_denied("internal/keys"));
    assert!(!p.resource_denied("api/data"));
}
