use crate::identity::{Identity, IdentityKind, IdentityStatus};

#[test]
fn new_identity_active() {
    let i = Identity::new("i1", "t1", IdentityKind::Human, 100);
    assert_eq!(i.id, "i1");
    assert!(i.is_active());
    assert!(i.groups.is_empty());
}

#[test]
fn suspend_identity() {
    let mut i = Identity::new("i1", "t1", IdentityKind::Service, 1);
    i.suspend();
    assert!(!i.is_active());
    assert_eq!(i.status, IdentityStatus::Suspended);
}

#[test]
fn revoke_identity() {
    let mut i = Identity::new("i1", "t1", IdentityKind::Device, 1);
    i.revoke();
    assert_eq!(i.status, IdentityStatus::Revoked);
}

#[test]
fn add_group() {
    let mut i = Identity::new("i1", "t1", IdentityKind::Human, 1);
    i.add_group("admin");
    assert!(i.in_group("admin"));
    assert!(!i.in_group("finance"));
}

#[test]
fn with_metadata() {
    let i = Identity::new("i1", "t1", IdentityKind::Workload, 1)
        .with_metadata("env", "prod");
    assert_eq!(i.metadata.get("env").map(|s| s.as_str()), Some("prod"));
}
