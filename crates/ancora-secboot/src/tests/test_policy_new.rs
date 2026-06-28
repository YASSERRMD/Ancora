use crate::BootPolicy;
#[test]
fn new_policy_denies_unknown_by_default() {
    let p = BootPolicy::new("t1");
    assert!(p.deny_unknown);
    assert_eq!(p.tenant_id, "t1");
}
