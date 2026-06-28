use crate::SensitivityLevel;
#[test]
fn levels_are_ordered_correctly() {
    assert!(SensitivityLevel::Public < SensitivityLevel::Internal);
    assert!(SensitivityLevel::Internal < SensitivityLevel::Confidential);
    assert!(SensitivityLevel::Confidential < SensitivityLevel::Restricted);
    assert!(SensitivityLevel::Restricted < SensitivityLevel::TopSecret);
}
