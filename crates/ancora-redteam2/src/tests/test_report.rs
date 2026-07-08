use crate::attack::{AttackLog, AttackOutcome, AttackStep, AttackVector};
use crate::audit::{RedTeamAction, RedTeamAuditEntry, RedTeamAuditLog};
use crate::detection::{DetectionEvent, DetectionLog, DetectionSource};
use crate::objective::{ObjectiveTracker, RedTeamObjective};
use crate::report::RedTeamReport;
use crate::scenario::{RedTeamScenario, ScenarioKind};
use crate::store::ScenarioStore;

#[test]
fn empty_report() {
    let store = ScenarioStore::new();
    let attacks = AttackLog::new();
    let detections = DetectionLog::new();
    let objectives = ObjectiveTracker::new();
    let audit = RedTeamAuditLog::new();
    let r = RedTeamReport::generate(&store, &attacks, &detections, &objectives, &audit, 99);
    assert_eq!(r.total_scenarios, 0);
    assert_eq!(r.active_scenarios, 0);
    assert_eq!(r.total_attack_steps, 0);
    assert_eq!(r.total_detections, 0);
    assert_eq!(r.total_objectives, 0);
    assert_eq!(r.achieved_objectives, 0);
    assert_eq!(r.total_audit_entries, 0);
    assert_eq!(r.tick, 99);
    assert!((r.objective_progress() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn populated_report() {
    let mut store = ScenarioStore::new();
    let mut s1 = RedTeamScenario::new("sc1", "t1", "N", ScenarioKind::LateralMovement, 1);
    s1.start();
    store.insert(s1);
    store.insert(RedTeamScenario::new(
        "sc2",
        "t1",
        "N",
        ScenarioKind::DataExfiltration,
        2,
    ));

    let mut attacks = AttackLog::new();
    attacks.record(AttackStep::new(
        "s1",
        "sc1",
        "N",
        AttackVector::Network,
        AttackOutcome::Success,
        "",
        "",
        1,
    ));

    let mut detections = DetectionLog::new();
    detections.record(DetectionEvent::new(
        "d1",
        "sc1",
        DetectionSource::Siem,
        "x",
        1,
        true,
    ));

    let mut objectives = ObjectiveTracker::new();
    objectives.add(RedTeamObjective::new("o1", "sc1", "d1"));
    objectives.get_mut("o1").unwrap().achieve(10);

    let mut audit = RedTeamAuditLog::new();
    audit.record(RedTeamAuditEntry::new(
        1,
        "t1",
        "sc1",
        RedTeamAction::ScenarioCreated,
        "op",
        "detail",
    ));

    let r = RedTeamReport::generate(&store, &attacks, &detections, &objectives, &audit, 100);
    assert_eq!(r.total_scenarios, 2);
    assert_eq!(r.active_scenarios, 1);
    assert_eq!(r.total_attack_steps, 1);
    assert_eq!(r.successful_steps, 1);
    assert_eq!(r.total_detections, 1);
    assert_eq!(r.true_positives, 1);
    assert_eq!(r.total_objectives, 1);
    assert_eq!(r.achieved_objectives, 1);
    assert_eq!(r.total_audit_entries, 1);
    assert!((r.objective_progress() - 1.0).abs() < 1e-9);
}
