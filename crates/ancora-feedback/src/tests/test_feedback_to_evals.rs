use crate::pipeline::FeedbackToEvalPipeline;
use crate::schema::{Feedback, ThumbsRating};

#[test]
fn feedback_becomes_eval_cases() {
    let mut pipeline = FeedbackToEvalPipeline::new();

    let fb1 = Feedback::new(
        "f1",
        "run-1",
        None,
        ThumbsRating::Up,
        Some("perfect".into()),
        "alice",
        0,
    );
    let fb2 = Feedback::new(
        "f2",
        "run-2",
        Some("step-3".into()),
        ThumbsRating::Down,
        None,
        "bob",
        1,
    );
    pipeline.ingest(&fb1);
    pipeline.ingest(&fb2);

    let cases = pipeline.cases();
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0].id, "eval-f1");
    assert!(cases[0].is_positive);
    assert_eq!(cases[0].annotation.as_deref(), Some("perfect"));

    assert_eq!(cases[1].id, "eval-f2");
    assert!(!cases[1].is_positive);
    assert_eq!(cases[1].step_id.as_deref(), Some("step-3"));
}

#[test]
fn drain_empties_pipeline() {
    let mut pipeline = FeedbackToEvalPipeline::new();
    let fb = Feedback::new("f1", "run-1", None, ThumbsRating::Up, None, "user", 0);
    pipeline.ingest(&fb);

    let drained = pipeline.drain();
    assert_eq!(drained.len(), 1);
    assert_eq!(pipeline.cases().len(), 0);
}

#[test]
fn multiple_feedback_all_converted() {
    let mut pipeline = FeedbackToEvalPipeline::new();
    for i in 0..10 {
        let fb = Feedback::new(
            format!("f{}", i),
            format!("run-{}", i),
            None,
            if i % 2 == 0 {
                ThumbsRating::Up
            } else {
                ThumbsRating::Down
            },
            None,
            "user",
            i as u64,
        );
        pipeline.ingest(&fb);
    }
    assert_eq!(pipeline.cases().len(), 10);
    let positive_count = pipeline.cases().iter().filter(|c| c.is_positive).count();
    assert_eq!(positive_count, 5);
}
