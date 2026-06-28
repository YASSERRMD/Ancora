use crate::attach::FeedbackStore;
use crate::decision::{DecisionOutcome, DecisionStore, ReviewDecision};
use crate::queue::ReviewQueue;
use crate::schema::{Feedback, ThumbsRating};

/// Request to submit new feedback.
#[derive(Debug, Clone)]
pub struct SubmitFeedbackRequest {
    pub run_id: String,
    pub step_id: Option<String>,
    pub rating: ThumbsRating,
    pub comment: Option<String>,
    pub author: String,
}

/// Response after submitting feedback.
#[derive(Debug, Clone)]
pub struct SubmitFeedbackResponse {
    pub feedback_id: String,
    pub queued_for_review: bool,
}

/// Request to submit a review decision.
#[derive(Debug, Clone)]
pub struct SubmitDecisionRequest {
    pub run_id: String,
    pub reviewer_id: String,
    pub outcome: DecisionOutcome,
    pub notes: Option<String>,
}

/// The unified feedback API surface.
pub struct FeedbackApi {
    feedback_store: FeedbackStore,
    review_queue: ReviewQueue,
    decision_store: DecisionStore,
    next_id: u64,
    confidence_threshold: f64,
}

impl FeedbackApi {
    /// Create a new API instance.
    pub fn new(confidence_threshold: f64) -> Self {
        Self {
            feedback_store: FeedbackStore::new(),
            review_queue: ReviewQueue::with_threshold(confidence_threshold),
            decision_store: DecisionStore::new(),
            next_id: 1,
            confidence_threshold,
        }
    }

    /// Submit feedback for a run.
    /// If the run's confidence is below the threshold the run is queued for review.
    pub fn submit_feedback(
        &mut self,
        req: SubmitFeedbackRequest,
        run_confidence: f64,
        timestamp: u64,
    ) -> SubmitFeedbackResponse {
        let feedback_id = format!("fb-{}", self.next_id);
        self.next_id += 1;

        let feedback = Feedback::new(
            &feedback_id,
            &req.run_id,
            req.step_id,
            req.rating,
            req.comment,
            req.author,
            timestamp,
        );
        self.feedback_store.attach(feedback);

        let queued_for_review = self.review_queue.submit(&req.run_id, run_confidence);

        SubmitFeedbackResponse {
            feedback_id,
            queued_for_review,
        }
    }

    /// Submit a review decision for a queued run.
    pub fn submit_decision(&mut self, req: SubmitDecisionRequest, timestamp: u64) {
        self.decision_store.record(ReviewDecision {
            run_id: req.run_id,
            reviewer_id: req.reviewer_id,
            outcome: req.outcome,
            notes: req.notes,
            decided_at: timestamp,
        });
    }

    /// Get all feedback for a run.
    pub fn feedback_for_run(&self, run_id: &str) -> &[Feedback] {
        self.feedback_store.for_run(run_id)
    }

    /// Get pending review queue entries.
    pub fn pending_reviews(&self) -> Vec<&crate::queue::QueueEntry> {
        self.review_queue.pending()
    }

    /// Get the latest decision for a run.
    pub fn latest_decision(&self, run_id: &str) -> Option<&ReviewDecision> {
        self.decision_store.latest_for_run(run_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_feedback_returns_id() {
        let mut api = FeedbackApi::new(0.7);
        let req = SubmitFeedbackRequest {
            run_id: "run-1".into(),
            step_id: None,
            rating: ThumbsRating::Up,
            comment: None,
            author: "alice".into(),
        };
        let resp = api.submit_feedback(req, 0.9, 1000);
        assert_eq!(resp.feedback_id, "fb-1");
        assert!(!resp.queued_for_review); // confidence 0.9 >= 0.7
    }

    #[test]
    fn low_confidence_queues_for_review() {
        let mut api = FeedbackApi::new(0.7);
        let req = SubmitFeedbackRequest {
            run_id: "run-2".into(),
            step_id: None,
            rating: ThumbsRating::Down,
            comment: Some("unclear".into()),
            author: "bob".into(),
        };
        let resp = api.submit_feedback(req, 0.3, 2000);
        assert!(resp.queued_for_review);
        assert_eq!(api.pending_reviews().len(), 1);
    }
}
