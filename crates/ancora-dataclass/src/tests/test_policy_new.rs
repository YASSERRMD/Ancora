use crate::{ClassificationPolicy, SensitivityLevel};
#[test]
fn policy_new_stores_tenant_and_level() {
    let p = ClassificationPolicy::new("t1", SensitivityLevel::Confidential);
    assert_eq!(p.tenant_id, "t1");
    assert_eq!(p.max_allowed_level, SensitivityLevel::Confidential);
    assert!(!p.require_category_tag);
}
