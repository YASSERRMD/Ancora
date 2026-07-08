/// A single conversation turn stored before summarization.
#[derive(Debug, Clone)]
pub struct Turn {
    pub index: usize,
    pub role: String,
    pub content: String,
}

/// Policy controlling when to trigger summarization.
#[derive(Debug, Clone)]
pub struct SummarizationPolicy {
    pub turns_before_summary: usize,
    pub keep_last_n: usize,
}

impl SummarizationPolicy {
    pub fn new(turns_before_summary: usize, keep_last_n: usize) -> Self {
        Self {
            turns_before_summary,
            keep_last_n,
        }
    }

    pub fn should_summarize(&self, total_turns: usize) -> bool {
        total_turns >= self.turns_before_summary
    }
}

/// Produces a rolling summary of turns older than keep_last_n.
pub struct ConversationSummarizer {
    pub policy: SummarizationPolicy,
}

impl ConversationSummarizer {
    pub fn new(policy: SummarizationPolicy) -> Self {
        Self { policy }
    }

    pub fn summarize(&self, turns: &[Turn]) -> SummaryResult {
        if turns.len() <= self.policy.keep_last_n {
            return SummaryResult {
                summary: String::new(),
                kept: turns.to_vec(),
                dropped_count: 0,
            };
        }
        let split = turns.len() - self.policy.keep_last_n;
        let to_summarize = &turns[..split];
        let kept = turns[split..].to_vec();

        let parts: Vec<String> = to_summarize
            .iter()
            .map(|t| format!("[{}] {}", t.role, t.content))
            .collect();
        let summary = format!(
            "Summary of {} turns: {}",
            to_summarize.len(),
            parts.join("; ")
        );
        SummaryResult {
            summary,
            kept,
            dropped_count: split,
        }
    }
}

/// Output of a summarization run.
#[derive(Debug)]
pub struct SummaryResult {
    pub summary: String,
    pub kept: Vec<Turn>,
    pub dropped_count: usize,
}
