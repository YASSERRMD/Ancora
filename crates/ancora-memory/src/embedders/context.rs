/// Context assembly with token budget.
///
/// Assembles a context window from retrieved passages, respecting a token
/// budget so the final context fits within an LLM's context length.
/// Token counting is approximate (whitespace-split word count multiplied by
/// a tokens-per-word factor).

// ---- token estimation ---------------------------------------------------

/// Estimate token count for a string using the "words * factor" heuristic.
/// GPT-family models average ~0.75 tokens per word; we round up to 1.5 for
/// safety (handles code, CJK, and punctuation-heavy text).
pub fn estimate_tokens(text: &str, tokens_per_word: f32) -> usize {
    let words = text.split_whitespace().count();
    (words as f32 * tokens_per_word).ceil() as usize
}

/// Estimate tokens using the default GPT factor (1.5 words per token).
pub fn estimate_tokens_default(text: &str) -> usize {
    estimate_tokens(text, 1.5)
}

// ---- assembled context -------------------------------------------------

#[derive(Debug, Clone)]
pub struct AssembledContext {
    /// The final concatenated context string.
    pub text: String,
    /// Number of passages included.
    pub passages_included: usize,
    /// Number of passages dropped due to budget.
    pub passages_dropped: usize,
    /// Estimated token count of the assembled context.
    pub estimated_tokens: usize,
}

impl AssembledContext {
    pub fn is_budget_exhausted(&self) -> bool {
        self.passages_dropped > 0
    }
}

// ---- context assembler -------------------------------------------------

#[derive(Debug, Clone)]
pub struct ContextAssembler {
    /// Maximum total tokens allowed (approximate).
    pub max_tokens: usize,
    /// Separator inserted between passages.
    pub separator: String,
    /// Header prepended to each passage (use `{index}` as placeholder).
    pub passage_header: Option<String>,
    /// Tokens-per-word factor for estimation.
    pub tokens_per_word: f32,
}

impl ContextAssembler {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            separator: "\n\n---\n\n".to_owned(),
            passage_header: Some("[Passage {index}]\n".to_owned()),
            tokens_per_word: 1.5,
        }
    }

    pub fn with_separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn without_headers(mut self) -> Self {
        self.passage_header = None;
        self
    }

    pub fn with_tokens_per_word(mut self, factor: f32) -> Self {
        self.tokens_per_word = factor;
        self
    }

    /// Assemble passages into a context window respecting `max_tokens`.
    /// Passages are taken in order; the first passage that would exceed the
    /// budget stops the assembly.
    pub fn assemble(&self, passages: &[&str]) -> AssembledContext {
        let mut parts: Vec<String> = Vec::new();
        let mut total_tokens = 0usize;
        let sep_tokens = estimate_tokens(&self.separator, self.tokens_per_word);
        let mut included = 0;
        let mut dropped = 0;

        for (i, passage) in passages.iter().enumerate() {
            let header = self
                .passage_header
                .as_ref()
                .map(|h| h.replace("{index}", &(i + 1).to_string()))
                .unwrap_or_default();
            let entry = format!("{header}{passage}");
            let entry_tokens = estimate_tokens(&entry, self.tokens_per_word);
            let sep_cost = if parts.is_empty() { 0 } else { sep_tokens };
            if total_tokens + sep_cost + entry_tokens > self.max_tokens {
                dropped += 1;
                continue;
            }
            total_tokens += sep_cost + entry_tokens;
            parts.push(entry);
            included += 1;
        }

        let text = parts.join(&self.separator);
        AssembledContext {
            estimated_tokens: total_tokens,
            text,
            passages_included: included,
            passages_dropped: dropped,
        }
    }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn estimate_tokens_empty_string() {
        assert_eq!(estimate_tokens("", 1.5), 0);
    }

    #[test]
    fn estimate_tokens_single_word() {
        let t = estimate_tokens("hello", 1.5);
        assert!(t >= 1, "single word must cost at least 1 token");
    }

    #[test]
    fn estimate_tokens_scales_with_words() {
        let t4 = estimate_tokens("one two three four", 1.5);
        let t8 = estimate_tokens("one two three four five six seven eight", 1.5);
        assert!(t8 > t4, "more words must cost more tokens");
    }

    #[test]
    fn assembler_includes_all_passages_in_budget() {
        let a = ContextAssembler::new(10000);
        let passages = &["Hello world.", "This is a test.", "Goodbye."];
        let ctx = a.assemble(passages);
        assert_eq!(ctx.passages_included, 3);
        assert_eq!(ctx.passages_dropped, 0);
    }

    #[test]
    fn assembler_drops_passages_over_budget() {
        // Very small budget so only one passage fits.
        let a = ContextAssembler::new(5);
        let passages = &["This is the first passage.", "This is the second passage."];
        let ctx = a.assemble(passages);
        assert!(ctx.passages_included < 2, "some passages should be dropped");
        assert!(ctx.passages_dropped > 0);
    }

    #[test]
    fn assembler_no_headers() {
        let a = ContextAssembler::new(9999).without_headers();
        let ctx = a.assemble(&["passage text"]);
        assert!(!ctx.text.contains("[Passage"), "text: {}", ctx.text);
    }

    #[test]
    fn assembler_custom_separator() {
        let a = ContextAssembler::new(9999).with_separator(" | ");
        let ctx = a.assemble(&["a", "b"]);
        assert!(ctx.text.contains(" | "), "text: {}", ctx.text);
    }

    #[test]
    fn assembled_context_is_budget_exhausted() {
        let a = ContextAssembler::new(5);
        let ctx = a.assemble(&["many words here", "another long passage"]);
        assert!(ctx.is_budget_exhausted());
    }

    #[test]
    fn assembled_context_not_budget_exhausted_when_all_fit() {
        let a = ContextAssembler::new(100_000);
        let ctx = a.assemble(&["short"]);
        assert!(!ctx.is_budget_exhausted());
    }

    #[test]
    fn estimated_tokens_nonzero_for_nonempty_context() {
        let a = ContextAssembler::new(9999).without_headers();
        let ctx = a.assemble(&["word"]);
        assert!(ctx.estimated_tokens > 0);
    }
}
