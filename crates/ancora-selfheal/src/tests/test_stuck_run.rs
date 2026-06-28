use crate::stuck_run::StuckRunDetector;

#[test]
fn no_stuck_runs_initially() {
    let mut d = StuckRunDetector::new();
    d.register("r1", 0, 60);
    assert!(d.stuck_runs(30).is_empty());
}

#[test]
fn stuck_run_detected_after_timeout() {
    let mut d = StuckRunDetector::new();
    d.register("r1", 0, 60);
    let stuck = d.stuck_runs(100);
    assert!(stuck.contains(&"r1"));
}

#[test]
fn tick_resets_stuck_timer() {
    let mut d = StuckRunDetector::new();
    d.register("r1", 0, 60);
    d.tick("r1", 80);
    assert!(d.stuck_runs(100).is_empty());
}

#[test]
fn remove_clears_run() {
    let mut d = StuckRunDetector::new();
    d.register("r1", 0, 10);
    d.remove("r1");
    assert!(d.stuck_runs(100).is_empty());
}
