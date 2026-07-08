use crate::SensitivityLevel;
#[test]
fn display_values() {
    assert_eq!(format!("{}", SensitivityLevel::Public), "PUBLIC");
    assert_eq!(format!("{}", SensitivityLevel::TopSecret), "TOP_SECRET");
    assert_eq!(
        format!("{}", SensitivityLevel::Confidential),
        "CONFIDENTIAL"
    );
}
