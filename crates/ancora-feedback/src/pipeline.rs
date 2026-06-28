use crate::schema::{Feedback, ThumbsRating};

/// An eval case derived from a feedback record.
#[derive(Debug, Clone)]
pub struct EvalCase {
    /// Unique ID for the eval case.
    pub id: String,
    /// The source run.
    pub run_id: String,
    /// The source step (if applicable).
    pub step_id: Option<String>,
    /// Whether this case is a positive example.
    pub is_positive: bool,
    /// Optional reviewer comment used as eval annotation.
    pub annotation: Option<String>,
}

/// Pipeline that converts feedback into eval dataset entries.
#[derive(Debug, Default)]
pub struct FeedbackToEvalPipeline {
    cases: Vec<EvalCase>,
}

impl FeedbackToEvalPipeline {
    /// Create a new pipeline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ingest a feedback record and produce an eval case.
    pub fn ingest(&mut self, feedback: &Feedback) {
        let case = EvalCase {
            id: format!("eval-{}", feedback.id),
            run_id: feedback.run_id.clone(),
            step_id: feedback.step_id.clone(),
            is_positive: feedback.rating == ThumbsRating::Up,
            annotation: feedback.comment.clone(),
        };
        self.cases.push(case);
    }

    /// Return all eval cases produced so far.
    pub fn cases(&self) -> &[EvalCase] {
        &self.cases
    }

    /// Drain all cases, consuming them.
    pub fn drain(&mut self) -> Vec<EvalCase> {
        std::mem::take(&mut self.cases)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Feedback, ThumbsRating};

    #[test]
    fn feedback_produces_eval_case() {
        let mut pipeline = FeedbackToEvalPipeline::new();
        let fb = Feedback::new(
            "f1",
            "run-1",
            None,
            ThumbsRating::Down,
            Some("wrong answer".into()),
            "alice",
            0,
        );
        pipeline.ingest(&fb);
        let cases = pipeline.cases();
        assert_eq!(cases.len(), 1);
        assert!(!cases[0].is_positive);
        assert_eq!(cases[0].annotation.as_deref(), Some("wrong answer"));
    }
}
