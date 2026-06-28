/// Feedback rating: thumbs up or thumbs down.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbsRating {
    Up,
    Down,
}

/// A single piece of human feedback on an agent run or step.
#[derive(Debug, Clone)]
pub struct Feedback {
    /// Unique identifier for this feedback record.
    pub id: String,
    /// The run this feedback is attached to.
    pub run_id: String,
    /// Optional step within the run.
    pub step_id: Option<String>,
    /// Thumbs rating.
    pub rating: ThumbsRating,
    /// Free-text comment.
    pub comment: Option<String>,
    /// Reviewer or user who submitted the feedback.
    pub author: String,
    /// Unix timestamp (seconds) when the feedback was created.
    pub created_at: u64,
}

impl Feedback {
    /// Create a new feedback record.
    pub fn new(
        id: impl Into<String>,
        run_id: impl Into<String>,
        step_id: Option<String>,
        rating: ThumbsRating,
        comment: Option<String>,
        author: impl Into<String>,
        created_at: u64,
    ) -> Self {
        Self {
            id: id.into(),
            run_id: run_id.into(),
            step_id,
            rating,
            comment,
            author: author.into(),
            created_at,
        }
    }

    /// Returns true if the feedback is positive (thumbs up).
    pub fn is_positive(&self) -> bool {
        self.rating == ThumbsRating::Up
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumbs_up_is_positive() {
        let fb = Feedback::new("f1", "r1", None, ThumbsRating::Up, None, "alice", 1_000_000);
        assert!(fb.is_positive());
    }

    #[test]
    fn thumbs_down_is_not_positive() {
        let fb = Feedback::new("f2", "r1", None, ThumbsRating::Down, None, "bob", 1_000_001);
        assert!(!fb.is_positive());
    }
}
