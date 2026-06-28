use crate::queue::{QueueEntry, ReviewQueue};

/// Summary view of a queue entry returned by the review queue API.
#[derive(Debug, Clone)]
pub struct QueueSummary {
    pub run_id: String,
    pub confidence: f64,
    pub claimed: bool,
}

impl From<&QueueEntry> for QueueSummary {
    fn from(e: &QueueEntry) -> Self {
        Self {
            run_id: e.run_id.clone(),
            confidence: e.confidence,
            claimed: e.claimed,
        }
    }
}

/// A thin API layer over the review queue for external callers.
pub struct ReviewQueueApi {
    queue: ReviewQueue,
}

impl ReviewQueueApi {
    /// Create a new API wrapping a queue with the given threshold.
    pub fn new(threshold: f64) -> Self {
        Self {
            queue: ReviewQueue::with_threshold(threshold),
        }
    }

    /// Submit a run to the queue. Returns true if the run was added.
    pub fn enqueue(&mut self, run_id: impl Into<String>, confidence: f64) -> bool {
        self.queue.submit(run_id, confidence)
    }

    /// List all pending entries as summaries.
    pub fn list_pending(&self) -> Vec<QueueSummary> {
        self.queue.pending().into_iter().map(QueueSummary::from).collect()
    }

    /// Claim a pending entry for review.
    pub fn claim(&mut self, run_id: &str) -> bool {
        self.queue.claim(run_id)
    }

    /// Total entries in the queue.
    pub fn total(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_queue_api_enqueue_and_list() {
        let mut api = ReviewQueueApi::new(0.5);
        api.enqueue("run-1", 0.2);
        api.enqueue("run-2", 0.4);
        api.enqueue("run-3", 0.9); // above threshold, not added
        assert_eq!(api.total(), 2);

        let pending = api.list_pending();
        assert_eq!(pending.len(), 2);
        assert!(pending.iter().all(|e| !e.claimed));
    }

    #[test]
    fn review_queue_api_claim_works() {
        let mut api = ReviewQueueApi::new(0.5);
        api.enqueue("run-x", 0.1);
        assert!(api.claim("run-x"));
        let pending = api.list_pending();
        assert_eq!(pending.len(), 0);
    }
}
