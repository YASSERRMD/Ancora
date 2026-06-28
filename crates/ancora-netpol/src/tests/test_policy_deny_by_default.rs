use crate::{DefaultPosture, NetworkPolicy};
#[test]
fn deny_by_default_sets_posture() {
    let policy = NetworkPolicy::deny_by_default("t1");
    assert_eq!(policy.default_posture, DefaultPosture::DenyAll);
}
