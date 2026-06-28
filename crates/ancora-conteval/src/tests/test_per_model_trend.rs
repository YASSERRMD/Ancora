use crate::model_tracking::ModelTracker;

#[test]
fn test_tracker_creates_model_on_first_record() {
    let mut tracker = ModelTracker::new(10);
    tracker.record("gpt-4", 1000, 0.9);
    assert_eq!(tracker.model_count(), 1);
}

#[test]
fn test_tracker_accumulates_scores() {
    let mut tracker = ModelTracker::new(10);
    tracker.record("gpt-4", 1000, 0.8);
    tracker.record("gpt-4", 2000, 0.9);
    tracker.record("gpt-4", 3000, 1.0);
    let mq = tracker.get("gpt-4").unwrap();
    assert_eq!(mq.observation_count(), 3);
    let mean = mq.mean().unwrap();
    assert!((mean - 0.9).abs() < 1e-10);
}

#[test]
fn test_tracker_tracks_multiple_models() {
    let mut tracker = ModelTracker::new(10);
    tracker.record("gpt-4", 1, 0.8);
    tracker.record("claude-3", 1, 0.9);
    tracker.record("llama-3", 1, 0.7);
    assert_eq!(tracker.model_count(), 3);
}

#[test]
fn test_tracker_returns_none_for_unknown_model() {
    let tracker = ModelTracker::new(10);
    assert!(tracker.get("unknown").is_none());
}

#[test]
fn test_tracker_latest_score() {
    let mut tracker = ModelTracker::new(10);
    tracker.record("m1", 1, 0.5);
    tracker.record("m1", 2, 0.75);
    let mq = tracker.get("m1").unwrap();
    let latest = mq.latest_score().unwrap();
    assert!((latest - 0.75).abs() < 1e-10);
}

#[test]
fn test_tracker_respects_window_capacity() {
    let mut tracker = ModelTracker::new(3);
    for i in 0u64..10 {
        tracker.record("m1", i, i as f64 * 0.1);
    }
    let mq = tracker.get("m1").unwrap();
    // Only 3 observations should be kept.
    assert_eq!(mq.observation_count(), 3);
}

#[test]
fn test_tracker_models_list() {
    let mut tracker = ModelTracker::new(5);
    tracker.record("a", 1, 0.5);
    tracker.record("b", 1, 0.5);
    let mut models = tracker.models();
    models.sort();
    assert_eq!(models, vec!["a", "b"]);
}
