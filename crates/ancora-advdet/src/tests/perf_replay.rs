use ancora_ageval::{PlanningMetric, RoutingMetric};
use ancora_coord::CoordJournal;
use ancora_orchestrate::fan_out;
use ancora_reason::{ContradictionDetector, StepDecomposer};
use serde_json::json;
use std::time::Instant;

#[test]
fn perf_fan_out_100_tasks_replay() {
    let inputs: Vec<_> = (0..100).map(|i| json!(format!("task-{i}"))).collect();
    let start = Instant::now();
    let tasks1 = fan_out("o", "a", inputs.clone(), "root");
    let tasks2 = fan_out("o", "a", inputs, "root");
    let elapsed = start.elapsed();

    assert_eq!(tasks1.len(), 100);
    assert_eq!(tasks1.len(), tasks2.len());
    assert!(elapsed.as_millis() < 500, "replay took {}ms", elapsed.as_millis());
}

#[test]
fn perf_50_reasoning_steps_replay() {
    let claims: Vec<String> = (0..50).map(|i| format!("claim-{i}")).collect();
    let start = Instant::now();
    let s1 = StepDecomposer::decompose(claims.clone());
    let s2 = StepDecomposer::decompose(claims);
    let elapsed = start.elapsed();

    assert_eq!(s1.len(), 50);
    assert_eq!(s1.len(), s2.len());
    assert!(elapsed.as_millis() < 200, "replay took {}ms", elapsed.as_millis());
}

#[test]
fn perf_100_coord_journal_entries_replay() {
    let start = Instant::now();
    let mut j1 = CoordJournal::default();
    let mut j2 = CoordJournal::default();
    for i in 0..100_u64 {
        j1.record(i, "assign", &format!("task-{i}"));
        j2.record(i, "assign", &format!("task-{i}"));
    }
    let r1 = j1.replay();
    let r2 = j2.replay();
    let elapsed = start.elapsed();

    assert_eq!(r1.len(), r2.len());
    assert!(elapsed.as_millis() < 100, "replay took {}ms", elapsed.as_millis());
}

#[test]
fn perf_routing_score_1000_calls_stable() {
    let start = Instant::now();
    let mut scores = Vec::with_capacity(1000);
    for i in 0..1000_u64 {
        scores.push(RoutingMetric::score(0.9, i % 1000, 1000));
    }
    let elapsed = start.elapsed();
    assert_eq!(scores.len(), 1000);
    assert!(elapsed.as_millis() < 100, "1000 routing scores took {}ms", elapsed.as_millis());
}
