use crate::probe::{LivenessProbe, ProbeStatus, ReadinessProbe, ReadinessStatus};

#[test]
fn liveness_alive_within_threshold() {
    let mut p = LivenessProbe::new(30);
    p.heartbeat(100);
    assert_eq!(p.check(120), ProbeStatus::Alive);
}

#[test]
fn liveness_dead_after_stall() {
    let mut p = LivenessProbe::new(10);
    p.heartbeat(0);
    assert!(matches!(p.check(20), ProbeStatus::Dead { .. }));
}

#[test]
fn readiness_ready_by_default() {
    let p = ReadinessProbe::new();
    assert_eq!(p.check(), ReadinessStatus::Ready);
}

#[test]
fn readiness_not_ready_when_dep_unhealthy() {
    let mut p = ReadinessProbe::new();
    p.deps_healthy = false;
    assert!(matches!(p.check(), ReadinessStatus::NotReady { .. }));
}

#[test]
fn readiness_not_ready_when_queue_saturated() {
    let mut p = ReadinessProbe::new();
    p.queue_saturated = true;
    assert!(matches!(p.check(), ReadinessStatus::NotReady { .. }));
}
