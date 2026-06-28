use crate::presets::{
    exfil_scenario, lateral_movement_scenario, network_attack_steps, priv_esc_scenario,
    siem_detections, standard_objectives,
};
use crate::scenario::ScenarioKind;

#[test]
fn priv_esc_preset() {
    let s = priv_esc_scenario("sc1", "t1", 1);
    assert_eq!(s.kind, ScenarioKind::PrivilegeEscalation);
    assert_eq!(s.mitre_tactic.as_deref(), Some("TA0004"));
    assert!(s.name.contains("Privilege"));
}

#[test]
fn lateral_movement_preset() {
    let s = lateral_movement_scenario("sc2", "t1", 5);
    assert_eq!(s.kind, ScenarioKind::LateralMovement);
    assert_eq!(s.mitre_tactic.as_deref(), Some("TA0008"));
}

#[test]
fn exfil_preset() {
    let s = exfil_scenario("sc3", "t1", 10);
    assert_eq!(s.kind, ScenarioKind::DataExfiltration);
    assert_eq!(s.mitre_tactic.as_deref(), Some("TA0010"));
}

#[test]
fn standard_objectives_preset() {
    let t = standard_objectives("sc1");
    assert_eq!(t.count(), 4);
    assert_eq!(t.pending_count(), 4);
}

#[test]
fn network_attack_steps_preset() {
    let log = network_attack_steps("sc1");
    assert_eq!(log.count(), 4);
    assert_eq!(log.successful().len(), 3);
    assert_eq!(log.detected().len(), 1);
}

#[test]
fn siem_detections_preset() {
    let log = siem_detections("sc1");
    assert_eq!(log.count(), 2);
    assert_eq!(log.true_positives().len(), 1);
    assert_eq!(log.false_positives().len(), 1);
}
