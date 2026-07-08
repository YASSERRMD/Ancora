/// Feedback module: collects user feedback and feeds it into eval datasets.

/// A single feedback item from a user or reviewer.
#[derive(Debug, Clone)]
pub struct FeedbackItem {
    pub run_id: String,
    pub span_id: Option<String>,
    pub rating: i8,
    pub comment: Option<String>,
    pub label: Option<String>,
}

impl FeedbackItem {
    pub fn new(run_id: impl Into<String>, rating: i8) -> Self {
        Self {
            run_id: run_id.into(),
            span_id: None,
            rating,
            comment: None,
            label: None,
        }
    }

    pub fn with_span(mut self, span_id: impl Into<String>) -> Self {
        self.span_id = Some(span_id.into());
        self
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn is_positive(&self) -> bool {
        self.rating > 0
    }

    pub fn is_negative(&self) -> bool {
        self.rating < 0
    }
}

/// An eval dataset entry derived from feedback.
#[derive(Debug, Clone)]
pub struct EvalDatasetEntry {
    pub source_run_id: String,
    pub input: String,
    pub expected_output: String,
    pub label: String,
}

/// A dataset of eval entries derived from feedback.
#[derive(Debug, Default)]
pub struct FeedbackEvalDataset {
    pub entries: Vec<EvalDatasetEntry>,
}

impl FeedbackEvalDataset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: EvalDatasetEntry) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn filter_by_label(&self, label: &str) -> Vec<&EvalDatasetEntry> {
        self.entries.iter().filter(|e| e.label == label).collect()
    }
}

/// Converts feedback items into eval dataset entries.
pub fn feedback_to_eval_entries(
    feedback: &[FeedbackItem],
    run_inputs: &std::collections::HashMap<String, String>,
    run_outputs: &std::collections::HashMap<String, String>,
) -> FeedbackEvalDataset {
    let mut dataset = FeedbackEvalDataset::new();
    for item in feedback {
        if let (Some(input), Some(output)) =
            (run_inputs.get(&item.run_id), run_outputs.get(&item.run_id))
        {
            let label = item.label.clone().unwrap_or_else(|| {
                if item.is_positive() {
                    "positive".to_string()
                } else {
                    "negative".to_string()
                }
            });
            dataset.add(EvalDatasetEntry {
                source_run_id: item.run_id.clone(),
                input: input.clone(),
                expected_output: output.clone(),
                label,
            });
        }
    }
    dataset
}
