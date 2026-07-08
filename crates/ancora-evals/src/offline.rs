use crate::grader::{Grader, Score};

/// Offline judge configuration.
///
/// When network access is unavailable, this module provides a local judge
/// that uses heuristic scoring without any external API calls.
#[derive(Debug, Clone)]
pub struct OfflineJudge {
    /// Minimum word overlap ratio required to consider the candidate acceptable.
    pub overlap_threshold: f64,
}

impl Default for OfflineJudge {
    fn default() -> Self {
        Self {
            overlap_threshold: 0.5,
        }
    }
}

impl OfflineJudge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_overlap_threshold(mut self, threshold: f64) -> Self {
        self.overlap_threshold = threshold;
        self
    }

    /// Compute a local score without any network calls.
    fn local_score(candidate: &str, expected: &str) -> f64 {
        let exp_words: Vec<&str> = expected.split_whitespace().collect();
        if exp_words.is_empty() {
            return 1.0;
        }
        let cand_lower = candidate.to_lowercase();
        let hits = exp_words
            .iter()
            .filter(|w| cand_lower.contains(&w.to_lowercase()))
            .count();
        hits as f64 / exp_words.len() as f64
    }
}

impl Grader for OfflineJudge {
    fn grade(&self, candidate: &str, expected: &str) -> Score {
        let raw = Self::local_score(candidate, expected);
        let value = if raw >= self.overlap_threshold {
            raw
        } else {
            0.0
        };
        Score::new(value.clamp(0.0, 1.0)).with_rationale(format!(
            "Offline heuristic: {:.3} (threshold: {:.3})",
            raw, self.overlap_threshold
        ))
    }

    fn name(&self) -> &str {
        "offline_judge"
    }
}

/// Run a batch of evaluations using the offline judge.
///
/// Returns a vector of `(id, score)` pairs.
pub fn run_offline_batch<'a>(
    judge: &OfflineJudge,
    examples: impl Iterator<Item = (&'a str, &'a str, &'a str)>,
) -> Vec<(String, Score)> {
    examples
        .map(|(id, candidate, expected)| {
            let score = judge.grade(candidate, expected);
            (id.to_string(), score)
        })
        .collect()
}
