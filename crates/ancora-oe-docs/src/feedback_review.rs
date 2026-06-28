//! Human feedback collection and review queue for agent outputs.

/// Thumbs-up or thumbs-down style feedback signal.
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackSignal {
    Positive,
    Negative,
    Neutral,
}

/// A piece of human feedback associated with an agent output.
#[derive(Debug, Clone)]
pub struct FeedbackItem {
    pub output_id: String,
    pub signal: FeedbackSignal,
    pub comment: Option<String>,
    pub reviewer_id: String,
}

impl FeedbackItem {
    pub fn new(
        output_id: impl Into<String>,
        signal: FeedbackSignal,
        reviewer_id: impl Into<String>,
    ) -> Self {
        Self {
            output_id: output_id.into(),
            signal,
            comment: None,
            reviewer_id: reviewer_id.into(),
        }
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

/// A queue of feedback items awaiting review.
#[derive(Debug, Default)]
pub struct ReviewQueue {
    items: Vec<FeedbackItem>,
}

impl ReviewQueue {
    pub fn enqueue(&mut self, item: FeedbackItem) {
        self.items.push(item);
    }

    pub fn pending_count(&self) -> usize {
        self.items.len()
    }

    /// Drain all items for batch processing.
    pub fn drain(&mut self) -> Vec<FeedbackItem> {
        std::mem::take(&mut self.items)
    }

    /// Compute the ratio of positive feedback (0.0 - 1.0).
    pub fn positive_ratio(&self) -> f64 {
        if self.items.is_empty() {
            return 0.0;
        }
        let pos = self
            .items
            .iter()
            .filter(|i| i.signal == FeedbackSignal::Positive)
            .count();
        pos as f64 / self.items.len() as f64
    }
}
