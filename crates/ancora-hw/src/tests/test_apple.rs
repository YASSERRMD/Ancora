use crate::apple::{
    apple_silicon_tuning, build_apple_silicon_profile, detect_apple_silicon_tier, AppleSiliconTier,
};
use crate::armnpu::{arm_npu_scheduling_hints, detect_arm_npu_capability};
use crate::model::HardwareProfile;

#[test]
fn apple_silicon_path_selected_for_m_profile() {
    let hw = build_apple_silicon_profile(8, 16384);
    assert!(hw.is_apple_silicon);
    assert!(hw.has_arm_npu);
    let tuning = apple_silicon_tuning(&hw);
    assert!(tuning.is_some());
    let t = tuning.unwrap();
    assert_eq!(t.tier, AppleSiliconTier::MBase);
    assert!(t.prefer_ane);
}

#[test]
fn apple_silicon_tuning_none_for_non_apple() {
    let hw = HardwareProfile::default_safe();
    assert!(!hw.is_apple_silicon);
    assert!(apple_silicon_tuning(&hw).is_none());
}

#[test]
fn tier_ultra_for_many_cores() {
    let hw = build_apple_silicon_profile(24, 192_000);
    assert_eq!(detect_apple_silicon_tier(&hw), AppleSiliconTier::MUltra);
}

#[test]
fn tier_pro_for_10_cores() {
    let hw = build_apple_silicon_profile(10, 32768);
    assert_eq!(detect_apple_silicon_tier(&hw), AppleSiliconTier::MPro);
}

#[test]
fn arm_npu_capability_detected_on_apple_silicon() {
    let hw = build_apple_silicon_profile(8, 16384);
    let cap = detect_arm_npu_capability(&hw);
    assert!(cap.is_some());
    let c = cap.unwrap();
    assert!(c.estimated_tops > 0);
}

#[test]
fn arm_npu_scheduling_hints_not_empty() {
    let hw = build_apple_silicon_profile(8, 16384);
    let cap = detect_arm_npu_capability(&hw).unwrap();
    let hints = arm_npu_scheduling_hints(&cap);
    assert!(!hints.is_empty());
}

#[test]
fn arm_npu_capability_none_when_no_npu() {
    let hw = HardwareProfile::default_safe();
    assert!(detect_arm_npu_capability(&hw).is_none());
}
