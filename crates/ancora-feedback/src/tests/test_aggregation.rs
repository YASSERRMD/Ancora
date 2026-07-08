use crate::aggregation::{aggregate, aggregate_by_run};
use crate::schema::{Feedback, ThumbsRating};

#[test]
fn aggregation_accurate() {
    let feedback = vec![
        Feedback::new(
            "f1",
            "r1",
            None,
            ThumbsRating::Up,
            Some("nice".into()),
            "alice",
            0,
        ),
        Feedback::new("f2", "r1", None, ThumbsRating::Up, None, "bob", 1),
        Feedback::new(
            "f3",
            "r1",
            None,
            ThumbsRating::Down,
            Some("bad".into()),
            "carol",
            2,
        ),
        Feedback::new("f4", "r1", None, ThumbsRating::Down, None, "dave", 3),
    ];

    let metrics = aggregate(&feedback);
    assert_eq!(metrics.total, 4);
    assert_eq!(metrics.thumbs_up, 2);
    assert_eq!(metrics.thumbs_down, 2);
    assert_eq!(metrics.with_comment, 2);
    let rate = metrics.approval_rate.unwrap();
    assert!(
        (rate - 0.5).abs() < 1e-9,
        "Expected 50% approval rate, got {}",
        rate
    );
}

#[test]
fn aggregation_by_run_correct() {
    let feedback = vec![
        Feedback::new("f1", "run-A", None, ThumbsRating::Up, None, "u1", 0),
        Feedback::new("f2", "run-A", None, ThumbsRating::Up, None, "u2", 1),
        Feedback::new("f3", "run-B", None, ThumbsRating::Down, None, "u3", 2),
    ];

    let by_run = aggregate_by_run(&feedback);
    let a = by_run.get("run-A").unwrap();
    assert_eq!(a.total, 2);
    assert_eq!(a.thumbs_up, 2);

    let b = by_run.get("run-B").unwrap();
    assert_eq!(b.total, 1);
    assert_eq!(b.thumbs_down, 1);
}

#[test]
fn single_feedback_approval_rate_is_one() {
    let feedback = vec![Feedback::new(
        "f1",
        "r1",
        None,
        ThumbsRating::Up,
        None,
        "user",
        0,
    )];
    let metrics = aggregate(&feedback);
    assert!((metrics.approval_rate.unwrap() - 1.0).abs() < 1e-9);
}

#[test]
fn all_thumbs_down_approval_rate_is_zero() {
    let feedback = vec![
        Feedback::new("f1", "r1", None, ThumbsRating::Down, None, "u1", 0),
        Feedback::new("f2", "r1", None, ThumbsRating::Down, None, "u2", 1),
    ];
    let metrics = aggregate(&feedback);
    assert!((metrics.approval_rate.unwrap() - 0.0).abs() < 1e-9);
}
