use crate::{Tenant, TenantStatus};
#[test]
fn new_tenant_is_active() {
    let t = Tenant::new("t1", "Acme Corp", 1);
    assert_eq!(t.status, TenantStatus::Active);
    assert!(t.is_active());
}
#[test]
fn suspend_changes_status() {
    let mut t = Tenant::new("t1", "Acme", 1);
    t.suspend();
    assert!(t.is_suspended());
    assert!(!t.is_active());
}
#[test]
fn delete_marks_deleted() {
    let mut t = Tenant::new("t1", "Acme", 1);
    t.delete();
    assert!(t.is_deleted());
}
