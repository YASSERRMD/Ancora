use crate::schema::{Feedback, ThumbsRating};

/// Aggregated metrics computed from a collection of feedback.
#[derive(Debug, Clone)]
pub struct FeedbackMetrics {
    pub total: usize,
    pub thumbs_up: usize,
    pub thumbs_down: usize,
    pub with_comment: usize,
    /// Approval rate as a fraction (0.0 to 1.0). None if no feedback.
    pub approval_rate: Option<f64>,
}

/// Aggregate a slice of feedback records into summary metrics.
pub fn aggregate(feedback: &[Feedback]) -> FeedbackMetrics {
    let total = feedback.len();
    let thumbs_up = feedback.iter().filter(|f| f.rating == ThumbsRating::Up).count();
    let thumbs_down = feedback.iter().filter(|f| f.rating == ThumbsRating::Down).count();
    let with_comment = feedback.iter().filter(|f| f.comment.is_some()).count();
    let approval_rate = if total == 0 {
        None
    } else {
        Some(thumbs_up as f64 / total as f64)
    };

    FeedbackMetrics {
        total,
        thumbs_up,
        thumbs_down,
        with_comment,
        approval_rate,
    }
}

/// Aggregate feedback grouped by run ID.
pub fn aggregate_by_run(feedback: &[Feedback]) -> std::collections::HashMap<String, FeedbackMetrics> {
    let mut by_run: std::collections::HashMap<String, Vec<&Feedback>> = std::collections::HashMap::new();
    for f in feedback {
        by_run.entry(f.run_id.clone()).or_default().push(f);
    }
    by_run
        .into_iter()
        .map(|(run_id, items)| {
            let owned: Vec<Feedback> = items.into_iter().cloned().collect();
            (run_id, aggregate(&owned))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Feedback, ThumbsRating};

    #[test]
    fn aggregation_correct() {
        let feedback = vec![
            Feedback::new("f1", "r1", None, ThumbsRating::Up, Some("great".into()), "alice", 0),
            Feedback::new("f2", "r1", None, ThumbsRating::Down, None, "bob", 1),
            Feedback::new("f3", "r1", None, ThumbsRating::Up, None, "carol", 2),
        ];
        let metrics = aggregate(&feedback);
        assert_eq!(metrics.total, 3);
        assert_eq!(metrics.thumbs_up, 2);
        assert_eq!(metrics.thumbs_down, 1);
        assert_eq!(metrics.with_comment, 1);
        let rate = metrics.approval_rate.unwrap();
        assert!((rate - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn empty_aggregation() {
        let metrics = aggregate(&[]);
        assert_eq!(metrics.total, 0);
        assert!(metrics.approval_rate.is_none());
    }
}
