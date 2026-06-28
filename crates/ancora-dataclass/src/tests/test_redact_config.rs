use crate::{RedactionConfig, SensitivityLevel};
#[test]
fn redacts_at_or_above_configured_level() {
    let config = RedactionConfig::new(SensitivityLevel::Confidential);
    assert!(config.should_redact(&SensitivityLevel::Confidential));
    assert!(config.should_redact(&SensitivityLevel::TopSecret));
    assert!(!config.should_redact(&SensitivityLevel::Internal));
}
#[test]
fn apply_returns_mask_for_high_level() {
    let config = RedactionConfig::new(SensitivityLevel::Restricted).with_mask("[HIDDEN]");
    let result = config.apply("secret-value", &SensitivityLevel::TopSecret);
    assert_eq!(result, "[HIDDEN]");
}
#[test]
fn apply_returns_value_for_low_level() {
    let config = RedactionConfig::new(SensitivityLevel::Restricted);
    let result = config.apply("public-value", &SensitivityLevel::Internal);
    assert_eq!(result, "public-value");
}
