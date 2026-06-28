use crate::conteval_e2e::{ContEvalEngine, RollingTracker};

#[test]
fn continuous_eval_tracks_quality() {
    let mut engine = ContEvalEngine::new();
    engine.add_tracker(RollingTracker::new("pass_rate", 5));

    // Record multiple observations.
    for _ in 0..5 {
        engine.record("pass_rate", 0.90);
    }

    let mean = engine.mean("pass_rate").expect("mean must be available");
    assert!((mean - 0.90).abs() < f64::EPSILON);
}

#[test]
fn rolling_tracker_evicts_old_values() {
    let mut tracker = RollingTracker::new("m", 3);
    tracker.record(1.0);
    tracker.record(2.0);
    tracker.record(3.0);
    // Window is full; next record evicts 1.0.
    tracker.record(4.0);

    assert_eq!(tracker.count(), 3);
    // Mean should now be (2+3+4)/3 = 3.0.
    let mean = tracker.mean().unwrap();
    assert!((mean - 3.0).abs() < f64::EPSILON);
}

#[test]
fn rolling_tracker_min_max_are_correct() {
    let mut tracker = RollingTracker::new("m", 5);
    for v in [3.0, 1.0, 4.0, 1.0, 5.0] {
        tracker.record(v);
    }
    assert!((tracker.min().unwrap() - 1.0).abs() < f64::EPSILON);
    assert!((tracker.max().unwrap() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn engine_returns_none_for_unknown_metric() {
    let engine = ContEvalEngine::new();
    assert!(engine.mean("nonexistent").is_none());
}

#[test]
fn engine_record_returns_false_for_unregistered_metric() {
    let mut engine = ContEvalEngine::new();
    let recorded = engine.record("ghost_metric", 1.0);
    assert!(!recorded);
}
