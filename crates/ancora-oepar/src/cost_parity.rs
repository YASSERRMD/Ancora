//! Cost attribute parity - validates cost attributes are uniform across language SDKs.

use std::collections::HashMap;

/// Required cost attributes that every SDK must emit.
pub const REQUIRED_COST_ATTRS: &[&str] = &[
    "gen_ai.usage.input_tokens",
    "gen_ai.usage.output_tokens",
    "gen_ai.cost.input_usd",
    "gen_ai.cost.output_usd",
    "gen_ai.cost.total_usd",
];

/// A cost record emitted as span attributes by an SDK.
#[derive(Debug, Clone)]
pub struct CostRecord {
    pub language: String,
    pub attributes: HashMap<String, f64>,
}

impl CostRecord {
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            attributes: HashMap::new(),
        }
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: f64) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }

    pub fn missing_attributes(&self) -> Vec<&'static str> {
        REQUIRED_COST_ATTRS
            .iter()
            .filter(|&&k| !self.attributes.contains_key(k))
            .copied()
            .collect()
    }

    pub fn is_complete(&self) -> bool {
        self.missing_attributes().is_empty()
    }
}

/// Tokens used in a single LLM call.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

impl TokenUsage {
    pub fn new(input: u64, output: u64) -> Self {
        Self {
            input_tokens: input,
            output_tokens: output,
        }
    }
}

/// Pricing rates per thousand tokens (USD).
#[derive(Debug, Clone, Copy)]
pub struct PricingRates {
    pub input_per_1k: f64,
    pub output_per_1k: f64,
}

impl PricingRates {
    pub fn new(input_per_1k: f64, output_per_1k: f64) -> Self {
        Self {
            input_per_1k,
            output_per_1k,
        }
    }

    pub fn compute_cost(&self, usage: TokenUsage) -> (f64, f64) {
        let input_cost = (usage.input_tokens as f64 / 1000.0) * self.input_per_1k;
        let output_cost = (usage.output_tokens as f64 / 1000.0) * self.output_per_1k;
        (input_cost, output_cost)
    }
}

/// Build a canonical cost record for testing.
pub fn reference_cost_record(language: impl Into<String>) -> CostRecord {
    let usage = TokenUsage::new(500, 200);
    let rates = PricingRates::new(0.03, 0.06);
    let (input_cost, output_cost) = rates.compute_cost(usage);
    CostRecord::new(language)
        .with_attr("gen_ai.usage.input_tokens", usage.input_tokens as f64)
        .with_attr("gen_ai.usage.output_tokens", usage.output_tokens as f64)
        .with_attr("gen_ai.cost.input_usd", input_cost)
        .with_attr("gen_ai.cost.output_usd", output_cost)
        .with_attr("gen_ai.cost.total_usd", input_cost + output_cost)
}

/// Compare cost records across languages and return any parity issues.
pub fn check_cost_parity(records: &[CostRecord]) -> Vec<String> {
    let mut issues = Vec::new();

    for record in records {
        let missing = record.missing_attributes();
        if !missing.is_empty() {
            issues.push(format!(
                "language {:?} missing cost attributes: {:?}",
                record.language, missing
            ));
        }
    }

    // Check that all records agree on token counts and costs.
    if let Some(first) = records.first() {
        for attr in REQUIRED_COST_ATTRS {
            let first_val = first.attributes.get(*attr).copied().unwrap_or(f64::NAN);
            for other in records.iter().skip(1) {
                let other_val = other.attributes.get(*attr).copied().unwrap_or(f64::NAN);
                if (first_val - other_val).abs() > 1e-9 {
                    issues.push(format!(
                        "attribute {:?} differs: {:?}={} vs {:?}={}",
                        attr, first.language, first_val, other.language, other_val
                    ));
                }
            }
        }
    }

    issues
}
