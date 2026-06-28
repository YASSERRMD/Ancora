use crate::identity::{Identity, IdentityKind};
use crate::stats::ZeroTrustStats;

#[test]
fn by_kind_breakdown() {
    let i1 = Identity::new("i1", "t1", IdentityKind::Human, 1);
    let i2 = Identity::new("i2", "t1", IdentityKind::Human, 1);
    let i3 = Identity::new("i3", "t1", IdentityKind::Service, 1);
    let v: Vec<&Identity> = vec![&i1, &i2, &i3];
    let s = ZeroTrustStats::for_tenant(&v, "t1");
    assert_eq!(s.by_kind.get("HUMAN").copied(), Some(2));
    assert_eq!(s.by_kind.get("SERVICE").copied(), Some(1));
}
