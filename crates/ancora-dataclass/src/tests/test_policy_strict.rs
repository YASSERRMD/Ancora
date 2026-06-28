use crate::{ClassificationPolicy, SensitivityLevel};
#[test]
fn strict_policy_sets_confidential_ceiling() {
    let p = ClassificationPolicy::strict("t1");
    assert_eq!(p.max_allowed_level, SensitivityLevel::Confidential);
    assert!(p.require_category_tag);
    assert!(p.deny_public_write);
}
