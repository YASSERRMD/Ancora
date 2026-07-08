/// Cost analytics module for tracking token usage and costs per run.
use std::collections::HashMap;

/// Token usage for a single LLM call.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
}

impl TokenUsage {
    pub fn new(prompt_tokens: u64, completion_tokens: u64) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
        }
    }

    pub fn total(&self) -> u64 {
        self.prompt_tokens + self.completion_tokens
    }
}

/// Cost entry for a single model call.
#[derive(Debug, Clone)]
pub struct CostEntry {
    pub run_id: String,
    pub model: String,
    pub usage: TokenUsage,
    /// Cost in microdollars (1e-6 USD).
    pub cost_microdollars: u64,
}

impl CostEntry {
    pub fn new(
        run_id: impl Into<String>,
        model: impl Into<String>,
        usage: TokenUsage,
        cost_microdollars: u64,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            model: model.into(),
            usage,
            cost_microdollars,
        }
    }

    pub fn cost_dollars(&self) -> f64 {
        self.cost_microdollars as f64 / 1_000_000.0
    }
}

/// Aggregated cost report for a run.
#[derive(Debug, Default)]
pub struct CostReport {
    entries: Vec<CostEntry>,
}

impl CostReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, entry: CostEntry) {
        self.entries.push(entry);
    }

    pub fn total_tokens(&self) -> u64 {
        self.entries.iter().map(|e| e.usage.total()).sum()
    }

    pub fn total_cost_microdollars(&self) -> u64 {
        self.entries.iter().map(|e| e.cost_microdollars).sum()
    }

    pub fn by_model(&self) -> HashMap<String, u64> {
        let mut map: HashMap<String, u64> = HashMap::new();
        for entry in &self.entries {
            *map.entry(entry.model.clone()).or_default() += entry.cost_microdollars;
        }
        map
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

/// Simulates a run and returns cost analytics.
pub fn simulate_run_cost(run_id: &str) -> CostReport {
    let mut report = CostReport::new();

    let usage1 = TokenUsage::new(512, 128);
    report.record(CostEntry::new(run_id, "local-judge", usage1, 320));

    let usage2 = TokenUsage::new(256, 64);
    report.record(CostEntry::new(run_id, "local-judge", usage2, 160));

    report
}
