use crate::attack::{AttackLog, AttackOutcome, AttackStep, AttackVector};
use crate::detection::{DetectionEvent, DetectionLog, DetectionSource};
use crate::scenario::{RedTeamScenario, ScenarioKind};
use crate::stats::RedTeamStats;

#[test]
fn empty_stats() {
    let scenarios: Vec<&RedTeamScenario> = vec![];
    let attacks = AttackLog::new();
    let detections = DetectionLog::new();
    let stats = RedTeamStats::compute(&scenarios, &attacks, &detections);
    assert_eq!(stats.total_scenarios, 0);
    assert_eq!(stats.total_attack_steps, 0);
    assert!((stats.success_rate - 0.0).abs() < f64::EPSILON);
    assert!((stats.detection_rate - 0.0).abs() < f64::EPSILON);
    assert!((stats.evasion_rate() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn computes_correctly() {
    let sc1 = RedTeamScenario::new("sc1", "t1", "N", ScenarioKind::LateralMovement, 1);
    let mut sc2 = RedTeamScenario::new("sc2", "t1", "N", ScenarioKind::DataExfiltration, 1);
    sc2.complete(50);
    let scenarios = vec![&sc1, &sc2];

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
    attacks.record(AttackStep::new(
        "s2",
        "sc1",
        "N",
        AttackVector::Local,
        AttackOutcome::Detected,
        "",
        "",
        2,
    ));
    attacks.record(AttackStep::new(
        "s3",
        "sc1",
        "N",
        AttackVector::Local,
        AttackOutcome::Failure,
        "",
        "",
        3,
    ));

    let mut detections = DetectionLog::new();
    detections.record(DetectionEvent::new(
        "d1",
        "sc1",
        DetectionSource::Edr,
        "x",
        2,
        true,
    ));
    detections.record(DetectionEvent::new(
        "d2",
        "sc1",
        DetectionSource::Siem,
        "y",
        3,
        false,
    ));

    let stats = RedTeamStats::compute(&scenarios, &attacks, &detections);
    assert_eq!(stats.total_scenarios, 2);
    assert_eq!(stats.completed_scenarios, 1);
    assert_eq!(stats.total_attack_steps, 3);
    assert_eq!(stats.successful_steps, 1);
    assert_eq!(stats.detected_steps, 1);
    assert_eq!(stats.total_detections, 2);
    assert_eq!(stats.true_positive_detections, 1);
    assert!((stats.success_rate - 1.0 / 3.0).abs() < 1e-9);
    assert!((stats.detection_rate - 0.5).abs() < 1e-9);
    // evasion: 2 undetected out of 3 total
    assert!((stats.evasion_rate() - 2.0 / 3.0).abs() < 1e-9);
}
