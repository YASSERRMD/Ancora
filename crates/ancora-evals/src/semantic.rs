use crate::grader::{Grader, Score};

/// Offline token-overlap semantic similarity grader.
///
/// Uses Jaccard similarity over word sets as a lightweight approximation.
/// No network calls required.
#[derive(Debug, Clone, Default)]
pub struct SemanticGrader {
    /// Minimum score threshold; outputs below this value are clamped to 0.0.
    pub threshold: f64,
}

impl SemanticGrader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }

    fn tokenize(text: &str) -> std::collections::HashSet<String> {
        text.split_whitespace()
            .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| !w.is_empty())
            .collect()
    }

    fn jaccard(a: &std::collections::HashSet<String>, b: &std::collections::HashSet<String>) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 1.0;
        }
        let intersection = a.intersection(b).count() as f64;
        let union = a.union(b).count() as f64;
        if union == 0.0 { 0.0 } else { intersection / union }
    }
}

impl Grader for SemanticGrader {
    fn grade(&self, candidate: &str, expected: &str) -> Score {
        let cand_tokens = Self::tokenize(candidate);
        let exp_tokens = Self::tokenize(expected);
        let raw = Self::jaccard(&cand_tokens, &exp_tokens);
        let value = if raw < self.threshold { 0.0 } else { raw };
        Score::new(value).with_rationale(format!(
            "Jaccard similarity: {:.3} (threshold: {:.3})",
            raw, self.threshold
        ))
    }

    fn name(&self) -> &str {
        "semantic_jaccard"
    }
}
