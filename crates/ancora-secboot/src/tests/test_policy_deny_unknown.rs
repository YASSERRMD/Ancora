use crate::BootPolicy;
#[test]
fn allow_unknown_permits_any_digest() {
    let p = BootPolicy::new("t1").allow_unknown();
    assert!(p.is_digest_allowed("anything", "whatever"));
}
