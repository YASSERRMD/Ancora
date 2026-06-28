use crate::attach::FeedbackStore;
use crate::schema::{Feedback, ThumbsRating};

#[test]
fn feedback_attaches_to_run() {
    let mut store = FeedbackStore::new();
    let fb = Feedback::new(
        "f1",
        "run-abc",
        Some("step-1".into()),
        ThumbsRating::Up,
        Some("Looks good".into()),
        "reviewer",
        9999,
    );
    store.attach(fb);

    let records = store.for_run("run-abc");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].run_id, "run-abc");
    assert_eq!(records[0].step_id.as_deref(), Some("step-1"));
    assert!(records[0].is_positive());
}

#[test]
fn feedback_attaches_multiple_to_same_run() {
    let mut store = FeedbackStore::new();
    for i in 0..5 {
        store.attach(Feedback::new(
            format!("f{}", i),
            "run-xyz",
            None,
            ThumbsRating::Down,
            None,
            "user",
            i as u64,
        ));
    }
    assert_eq!(store.for_run("run-xyz").len(), 5);
    assert_eq!(store.total(), 5);
}

#[test]
fn feedback_step_filter_works() {
    let mut store = FeedbackStore::new();
    store.attach(Feedback::new("f1", "run-1", Some("step-A".into()), ThumbsRating::Up, None, "alice", 0));
    store.attach(Feedback::new("f2", "run-1", Some("step-B".into()), ThumbsRating::Down, None, "bob", 1));
    store.attach(Feedback::new("f3", "run-1", None, ThumbsRating::Up, None, "carol", 2));

    let step_a = store.for_step("run-1", "step-A");
    assert_eq!(step_a.len(), 1);
    assert_eq!(step_a[0].id, "f1");
}
