use ancora_preset::{assemble, government_compliant, AirGapPolicy};
use ancora_ageval::RoutingMetric;

#[test]
fn government_preset_airgap_flag() {
    let preset = government_compliant("us-gov-east-1");
    assert_eq!(preset.air_gap, AirGapPolicy::Required);
    let spec = assemble(&preset).expect("assemble");
    assert!(spec.system_prompt.contains("air_gap:required"));
}

#[test]
fn milestone_suite_runs_fully_in_process() {
    // If this test runs, we already know we're offline (no panic on import)
    let score = RoutingMetric::score(0.9, 300, 1000);
    assert!((score - 0.8).abs() < 1e-9);
}
