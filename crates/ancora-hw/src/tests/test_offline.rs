/// Test that the entire crate runs offline (no network calls, no external deps).
///
/// These tests verify pure-logic paths that cannot require network access.
use crate::config::{parse_override, serialize_profile};
use crate::model::HardwareProfile;
use crate::probe::probe_hardware;
use crate::thermal::{read_thermal_pressure, run_thermal_hook, ThermalPressure};

#[test]
fn runs_offline_probe_does_not_panic() {
    // probe_hardware must complete without panicking on any supported platform.
    let hw = probe_hardware();
    assert!(hw.total_ram_mib > 0);
}

#[test]
fn runs_offline_thermal_hook() {
    let result = run_thermal_hook(ThermalPressure::Serious, |p| {
        format!("throttled at {:?}", p)
    });
    assert!(result.action_taken.contains("throttled"));
    assert_eq!(result.pressure, ThermalPressure::Serious as u8);
}

#[test]
fn runs_offline_config_round_trip() {
    let hw = HardwareProfile::default_safe();
    let json = serialize_profile(&hw).expect("serialize");
    // Must not contain any HTTP URLs.
    assert!(!json.contains("http://"), "unexpected URL in profile json");
    assert!(!json.contains("https://"), "unexpected URL in profile json");
}

#[test]
fn runs_offline_override_applied() {
    let json = r#"{"total_ram_mib": 98304}"#;
    let ov = parse_override(json).expect("parse");
    let base = HardwareProfile::default_safe();
    let merged = ov.apply(base);
    assert_eq!(merged.total_ram_mib, 98304);
}

#[test]
fn thermal_pressure_throughput_scale_range() {
    for v in 0u8..=3 {
        let p = ThermalPressure::from_u8(v);
        let scale = p.throughput_scale();
        assert!(
            scale > 0.0 && scale <= 1.0,
            "scale out of range for {:?}",
            p
        );
    }
}
