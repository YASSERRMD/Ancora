/// Pricing in USD per one million tokens.
#[derive(Debug, Clone)]
pub struct PricingMeta {
    pub input_per_million: f64,
    pub output_per_million: f64,
    /// Discounted rate for cache-hit input tokens (e.g. DeepSeek cache-hit tier).
    pub cached_per_million: Option<f64>,
}

/// Feature flags for a model.
#[derive(Debug, Clone, Default)]
pub struct CapabilityFlags {
    pub tools: bool,
    pub vision: bool,
    pub streaming: bool,
}

/// All metadata for a single canonical model identifier.
#[derive(Debug, Clone)]
pub struct ModelMeta {
    pub model_id: String,
    /// Maximum context window in tokens.
    pub context_window: u32,
    pub pricing: Option<PricingMeta>,
    pub capabilities: CapabilityFlags,
}

impl ModelMeta {
    pub fn new(model_id: impl Into<String>, context_window: u32) -> Self {
        Self {
            model_id: model_id.into(),
            context_window,
            pricing: None,
            capabilities: CapabilityFlags::default(),
        }
    }

    pub fn with_pricing(mut self, input: f64, output: f64) -> Self {
        self.pricing = Some(PricingMeta {
            input_per_million: input,
            output_per_million: output,
            cached_per_million: None,
        });
        self
    }

    pub fn with_cached_pricing(mut self, cached: f64) -> Self {
        if let Some(ref mut p) = self.pricing {
            p.cached_per_million = Some(cached);
        }
        self
    }

    pub fn with_tools(mut self) -> Self {
        self.capabilities.tools = true;
        self
    }

    pub fn with_vision(mut self) -> Self {
        self.capabilities.vision = true;
        self
    }

    pub fn with_streaming(mut self) -> Self {
        self.capabilities.streaming = true;
        self
    }

    /// Compute the USD cost of a completion given token counts.
    ///
    /// Returns `None` if no pricing is registered for this model.
    pub fn compute_cost(&self, tokens_in: u64, tokens_out: u64, cached_in: u64) -> Option<f64> {
        let p = self.pricing.as_ref()?;
        let cost = (tokens_in as f64) * p.input_per_million / 1_000_000.0
            + (tokens_out as f64) * p.output_per_million / 1_000_000.0
            + (cached_in as f64) * p.cached_per_million.unwrap_or(0.0) / 1_000_000.0;
        Some(cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_cost_with_pricing() {
        let meta = ModelMeta::new("gpt-4o", 128_000)
            .with_pricing(5.0, 15.0)
            .with_cached_pricing(2.5);
        let cost = meta.compute_cost(1_000_000, 500_000, 200_000).unwrap();
        // 1M * $5/M + 0.5M * $15/M + 0.2M * $2.5/M = 5 + 7.5 + 0.5 = 13.0
        assert!((cost - 13.0).abs() < 1e-9);
    }

    #[test]
    fn compute_cost_without_pricing_returns_none() {
        let meta = ModelMeta::new("local", 4096);
        assert!(meta.compute_cost(1000, 500, 0).is_none());
    }

    #[test]
    fn capability_flags_default_all_false() {
        let meta = ModelMeta::new("base", 8192);
        assert!(!meta.capabilities.tools);
        assert!(!meta.capabilities.vision);
        assert!(!meta.capabilities.streaming);
    }

    #[test]
    fn builder_sets_capability_flags() {
        let meta = ModelMeta::new("full", 128_000)
            .with_tools()
            .with_vision()
            .with_streaming();
        assert!(meta.capabilities.tools);
        assert!(meta.capabilities.vision);
        assert!(meta.capabilities.streaming);
    }
}
