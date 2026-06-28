use crate::{Tenant, TenantStatus};
#[test]
fn display_includes_id_name_status() {
    let t = Tenant::new("acme", "Acme Corp", 1);
    let s = format!("{}", t);
    assert!(s.contains("acme") && s.contains("Acme Corp") && s.contains("active"));
}
#[test]
fn status_display_suspended() {
    let s = format!("{}", TenantStatus::Suspended);
    assert_eq!(s, "suspended");
}
#[test]
fn status_display_deleted() {
    let s = format!("{}", TenantStatus::Deleted);
    assert_eq!(s, "deleted");
}
