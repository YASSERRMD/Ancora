use crate::{DefaultPosture, NetworkPolicy};
#[test]
fn allow_by_default_sets_posture() {
    let policy = NetworkPolicy::allow_by_default("t1");
    assert_eq!(policy.default_posture, DefaultPosture::AllowAll);
}
