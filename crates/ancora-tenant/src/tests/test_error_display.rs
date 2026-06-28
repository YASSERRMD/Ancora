use crate::TenantError;
#[test]
fn error_not_found_display() {
    let e = TenantError::NotFound("t1".into());
    let s = format!("{}", e);
    assert!(s.contains("t1"));
}
#[test]
fn error_quota_exceeded_display() {
    let e = TenantError::QuotaExceeded { resource: "agent".into(), used: 11, max: 10 };
    let s = format!("{}", e);
    assert!(s.contains("agent") && s.contains("11"));
}
