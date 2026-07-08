//! Feedback and review view - human annotations and ratings on runs/steps.

#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackRating {
    ThumbsUp,
    ThumbsDown,
    Neutral,
    Score(u8), // 1-5
}

impl FeedbackRating {
    pub fn numeric(&self) -> f64 {
        match self {
            FeedbackRating::ThumbsUp => 1.0,
            FeedbackRating::ThumbsDown => 0.0,
            FeedbackRating::Neutral => 0.5,
            FeedbackRating::Score(s) => *s as f64 / 5.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeedbackEntry {
    pub id: String,
    pub run_id: String,
    pub step_index: Option<usize>,
    pub rating: FeedbackRating,
    pub comment: Option<String>,
    pub reviewer: String,
    pub created_at: u64,
    pub tags: Vec<String>,
}

pub struct FeedbackView {
    entries: Vec<FeedbackEntry>,
}

impl FeedbackView {
    pub fn new(entries: Vec<FeedbackEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[FeedbackEntry] {
        &self.entries
    }

    pub fn entries_for_run(&self, run_id: &str) -> Vec<&FeedbackEntry> {
        self.entries.iter().filter(|e| e.run_id == run_id).collect()
    }

    pub fn average_rating_for_run(&self, run_id: &str) -> Option<f64> {
        let scores: Vec<f64> = self
            .entries_for_run(run_id)
            .iter()
            .map(|e| e.rating.numeric())
            .collect();
        if scores.is_empty() {
            return None;
        }
        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }

    pub fn negative_entries(&self) -> Vec<&FeedbackEntry> {
        self.entries
            .iter()
            .filter(|e| e.rating == FeedbackRating::ThumbsDown)
            .collect()
    }

    pub fn entries_by_tag(&self, tag: &str) -> Vec<&FeedbackEntry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    pub fn add_entry(&mut self, entry: FeedbackEntry) {
        self.entries.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_view() -> FeedbackView {
        FeedbackView::new(vec![
            FeedbackEntry {
                id: "f1".into(),
                run_id: "r1".into(),
                step_index: Some(0),
                rating: FeedbackRating::ThumbsUp,
                comment: None,
                reviewer: "alice".into(),
                created_at: 1000,
                tags: vec!["accuracy".into()],
            },
            FeedbackEntry {
                id: "f2".into(),
                run_id: "r1".into(),
                step_index: None,
                rating: FeedbackRating::ThumbsDown,
                comment: Some("wrong answer".into()),
                reviewer: "bob".into(),
                created_at: 2000,
                tags: vec![],
            },
        ])
    }

    #[test]
    fn test_average_rating() {
        let view = sample_view();
        let avg = view.average_rating_for_run("r1").unwrap();
        assert!((avg - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_negative_entries() {
        let view = sample_view();
        let neg = view.negative_entries();
        assert_eq!(neg.len(), 1);
        assert_eq!(neg[0].id, "f2");
    }

    #[test]
    fn test_entries_by_tag() {
        let view = sample_view();
        let tagged = view.entries_by_tag("accuracy");
        assert_eq!(tagged.len(), 1);
    }
}
