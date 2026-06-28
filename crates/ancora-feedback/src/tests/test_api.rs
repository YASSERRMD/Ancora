use crate::api::{FeedbackApi, SubmitDecisionRequest, SubmitFeedbackRequest};
use crate::decision::DecisionOutcome;
use crate::schema::ThumbsRating;

#[test]
fn feedback_api_works() {
    let mut api = FeedbackApi::new(0.6);

    // Submit positive feedback on a high-confidence run
    let req = SubmitFeedbackRequest {
        run_id: "run-1".into(),
        step_id: None,
        rating: ThumbsRating::Up,
        comment: Some("Excellent response".into()),
        author: "alice".into(),
    };
    let resp = api.submit_feedback(req, 0.95, 1000);
    assert_eq!(resp.feedback_id, "fb-1");
    assert!(!resp.queued_for_review);
    assert_eq!(api.feedback_for_run("run-1").len(), 1);
}

#[test]
fn low_confidence_queues_in_api() {
    let mut api = FeedbackApi::new(0.6);
    let req = SubmitFeedbackRequest {
        run_id: "run-2".into(),
        step_id: Some("step-1".into()),
        rating: ThumbsRating::Down,
        comment: Some("Hallucinated".into()),
        author: "bob".into(),
    };
    let resp = api.submit_feedback(req, 0.4, 2000);
    assert!(resp.queued_for_review);
    assert_eq!(api.pending_reviews().len(), 1);
}

#[test]
fn decision_recorded_via_api() {
    let mut api = FeedbackApi::new(0.5);

    // First submit feedback and queue
    let req = SubmitFeedbackRequest {
        run_id: "run-3".into(),
        step_id: None,
        rating: ThumbsRating::Down,
        comment: None,
        author: "carol".into(),
    };
    api.submit_feedback(req, 0.2, 100);

    // Then reviewer approves
    let dec_req = SubmitDecisionRequest {
        run_id: "run-3".into(),
        reviewer_id: "reviewer-1".into(),
        outcome: DecisionOutcome::Approve,
        notes: Some("Verified manually".into()),
    };
    api.submit_decision(dec_req, 200);

    let decision = api.latest_decision("run-3").unwrap();
    assert_eq!(decision.outcome, DecisionOutcome::Approve);
    assert_eq!(decision.reviewer_id, "reviewer-1");
}

#[test]
fn sequential_feedback_ids() {
    let mut api = FeedbackApi::new(0.5);
    for i in 1..=5 {
        let req = SubmitFeedbackRequest {
            run_id: format!("run-{}", i),
            step_id: None,
            rating: ThumbsRating::Up,
            comment: None,
            author: "user".into(),
        };
        let resp = api.submit_feedback(req, 0.9, i as u64);
        assert_eq!(resp.feedback_id, format!("fb-{}", i));
    }
}
