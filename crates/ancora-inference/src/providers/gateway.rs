/// An ordered chain of `(provider_name, model_id)` pairs for gateway fallback routing.
///
/// When a request to the primary fails, the caller can iterate `next()` to
/// try the next entry in priority order. This type is provider-agnostic --
/// combine it with any gateway profile (OpenRouter, LiteLLM, Portkey, etc.).
#[derive(Debug, Clone)]
pub struct FallbackChain {
    entries: Vec<(String, String)>,
    cursor: usize,
}

impl FallbackChain {
    /// Create an empty fallback chain.
    pub fn new() -> Self {
        Self { entries: Vec::new(), cursor: 0 }
    }

    /// Append a `(provider, model)` pair to the chain.
    pub fn push(mut self, provider: impl Into<String>, model: impl Into<String>) -> Self {
        self.entries.push((provider.into(), model.into()));
        self
    }

    /// Return the primary (first) entry, or `None` if the chain is empty.
    pub fn primary(&self) -> Option<(&str, &str)> {
        self.entries.first().map(|(p, m)| (p.as_str(), m.as_str()))
    }

    /// Advance and return the next fallback entry, or `None` when exhausted.
    pub fn next_fallback(&mut self) -> Option<(&str, &str)> {
        self.cursor += 1;
        self.entries
            .get(self.cursor)
            .map(|(p, m)| (p.as_str(), m.as_str()))
    }

    /// Reset the cursor back to the primary.
    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    /// Return the number of entries in the chain.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` if the chain has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Build a flat list of `"provider/model"` strings suitable for OpenRouter's
    /// `models` request field.
    pub fn to_openrouter_models(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|(p, m)| format!("{}/{}", p, m))
            .collect()
    }
}

impl Default for FallbackChain {
    fn default() -> Self { Self::new() }
}

/// Parse the cost USD value from an OpenRouter `x-openrouter-cost` response header.
///
/// OpenRouter includes the actual cost of a request in a response header.
/// Returns `None` if the header is absent, empty, or not a valid float.
pub fn parse_openrouter_cost_header(header_value: Option<&str>) -> Option<f64> {
    header_value?.trim().parse::<f64>().ok()
}

/// Parse the model actually used from an OpenRouter `x-openrouter-model` response header.
///
/// When a fallback fires, OpenRouter reports the model that was actually used.
pub fn parse_openrouter_model_header(header_value: Option<&str>) -> Option<&str> {
    header_value.map(|s| s.trim()).filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_chain() -> FallbackChain {
        FallbackChain::new()
            .push("openai", "gpt-4o")
            .push("anthropic", "claude-3-5-haiku")
            .push("mistral", "mistral-small-latest")
    }

    #[test]
    fn fallback_chain_primary_returns_first() {
        let chain = sample_chain();
        assert_eq!(chain.primary(), Some(("openai", "gpt-4o")));
    }

    #[test]
    fn fallback_chain_next_advances_cursor() {
        let mut chain = sample_chain();
        assert_eq!(chain.next_fallback(), Some(("anthropic", "claude-3-5-haiku")));
        assert_eq!(chain.next_fallback(), Some(("mistral", "mistral-small-latest")));
        assert_eq!(chain.next_fallback(), None);
    }

    #[test]
    fn fallback_chain_reset_goes_back_to_start() {
        let mut chain = sample_chain();
        chain.next_fallback();
        chain.reset();
        assert_eq!(chain.next_fallback(), Some(("anthropic", "claude-3-5-haiku")));
    }

    #[test]
    fn fallback_chain_to_openrouter_models() {
        let chain = sample_chain();
        let models = chain.to_openrouter_models();
        assert_eq!(models[0], "openai/gpt-4o");
        assert_eq!(models[1], "anthropic/claude-3-5-haiku");
    }

    #[test]
    fn fallback_chain_len_and_is_empty() {
        assert!(FallbackChain::new().is_empty());
        assert_eq!(sample_chain().len(), 3);
    }
}
