use crate::builder::{IdentityBuilder, SessionBuilder};
use crate::identity::IdentityKind;

#[test]
fn identity_builder() {
    let i = IdentityBuilder::new("i1", "t1", IdentityKind::Human)
        .tick(500)
        .group("admin")
        .group("security")
        .build();
    assert_eq!(i.id, "i1");
    assert_eq!(i.created_tick, 500);
    assert!(i.in_group("admin"));
    assert!(i.in_group("security"));
}

#[test]
fn session_builder() {
    let s = SessionBuilder::new("s1", "t1", "i1")
        .created_at(100)
        .expires_at(1000)
        .device("d1")
        .build();
    assert_eq!(s.id, "s1");
    assert_eq!(s.created_tick, 100);
    assert_eq!(s.expires_tick, 1000);
    assert_eq!(s.device_id.as_deref(), Some("d1"));
}
