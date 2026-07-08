/// Estimates token footprint of in-memory state before and after consolidation.
pub struct TokenBudget {
    pub max_tokens: usize,
}

impl TokenBudget {
    pub fn new(max_tokens: usize) -> Self {
        Self { max_tokens }
    }

    /// Rough estimate: 1 token per 4 characters.
    pub fn estimate_tokens(content: &str) -> usize {
        content.len().div_ceil(4)
    }

    pub fn total_tokens(contents: &[String]) -> usize {
        contents.iter().map(|c| Self::estimate_tokens(c)).sum()
    }

    pub fn within_budget(&self, contents: &[String]) -> bool {
        Self::total_tokens(contents) <= self.max_tokens
    }
}
