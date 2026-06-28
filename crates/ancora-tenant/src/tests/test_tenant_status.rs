use crate::TenantStatus;
#[test]
fn status_variants_are_distinct() {
    assert_ne!(TenantStatus::Active, TenantStatus::Suspended);
    assert_ne!(TenantStatus::Suspended, TenantStatus::Deleted);
    assert_ne!(TenantStatus::Active, TenantStatus::Deleted);
}
