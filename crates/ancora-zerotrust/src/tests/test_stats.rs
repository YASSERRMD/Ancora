use crate::identity::{Identity, IdentityKind};
use crate::stats::ZeroTrustStats;

#[test]
fn stats_empty() {
    let s = ZeroTrustStats::for_tenant(&[], "t1");
    assert_eq!(s.total_identities, 0);
    assert_eq!(s.active_identities, 0);
}

#[test]
fn stats_for_tenant() {
    let i1 = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let i2 = Identity::new("i2", "t1", IdentityKind::Service, 1);
    let i3 = Identity::new("i3", "t2", IdentityKind::Human, 1);
    let v: Vec<&Identity> = vec![&i1, &i2, &i3];
    let s = ZeroTrustStats::for_tenant(&v, "t1");
    assert_eq!(s.total_identities, 2);
    assert_eq!(s.active_identities, 2);
}

#[test]
fn stats_suspended() {
    let i1 = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let mut i2 = Identity::new("i2", "t1", IdentityKind::Human, 1);
    i2.suspend();
    let v: Vec<&Identity> = vec![&i1, &i2];
    let s = ZeroTrustStats::for_tenant(&v, "t1");
    assert_eq!(s.suspended_identities, 1);
}
