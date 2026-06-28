use crate::{ClassificationPolicy, SensitivityLevel};
#[test]
fn permissive_policy_allows_top_secret() {
    let p = ClassificationPolicy::permissive("t1");
    assert_eq!(p.max_allowed_level, SensitivityLevel::TopSecret);
    assert!(!p.require_category_tag);
}
