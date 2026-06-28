//! Reference distribution capture.
//!
//! Builds summary statistics from a baseline window of traces so that
//! later windows can be compared for drift.

use std::collections::HashMap;

/// Summary statistics over a numeric metric.
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub count: usize,
    pub mean: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
}

impl Stats {
    /// Compute statistics from a non-empty slice.
    pub fn from_slice(values: &[f64]) -> Option<Self> {
        if values.is_empty() {
            return None;
        }
        let count = values.len();
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = values.iter().sum::<f64>() / count as f64;
        let variance = values
            .iter()
            .map(|v| (v - mean) * (v - mean))
            .sum::<f64>()
            / count as f64;
        Some(Self { count, mean, variance, min, max })
    }

    /// Standard deviation.
    pub fn std_dev(&self) -> f64 {
        self.variance.sqrt()
    }
}

/// Captured reference distribution built from a baseline corpus of traces.
#[derive(Debug, Clone)]
pub struct ReferenceDistribution {
    /// Stats for input token count proxy (character length).
    pub input_len: Stats,
    /// Stats for output token count proxy (character length).
    pub output_len: Stats,
    /// Stats for cost in micro-dollars.
    pub cost_micros: Stats,
    /// Stats for latency in milliseconds.
    pub latency_ms: Stats,
    /// Relative frequency of each tool name.
    pub tool_frequencies: HashMap<String, f64>,
    /// Relative frequency of each provider.
    pub provider_frequencies: HashMap<String, f64>,
    /// Number of traces used to build this distribution.
    pub n: usize,
}

/// Builder that accumulates trace data before finalising the reference.
#[derive(Debug, Default)]
pub struct ReferenceBuilder {
    input_lens: Vec<f64>,
    output_lens: Vec<f64>,
    cost_micros: Vec<f64>,
    latency_ms: Vec<f64>,
    tool_counts: HashMap<String, usize>,
    provider_counts: HashMap<String, usize>,
}

impl ReferenceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single observation.
    pub fn add(
        &mut self,
        input: &str,
        output: &str,
        cost_micros: u64,
        latency_ms: u64,
        tools: &[String],
        provider: &str,
    ) {
        self.input_lens.push(input.len() as f64);
        self.output_lens.push(output.len() as f64);
        self.cost_micros.push(cost_micros as f64);
        self.latency_ms.push(latency_ms as f64);
        for t in tools {
            *self.tool_counts.entry(t.clone()).or_insert(0) += 1;
        }
        *self.provider_counts.entry(provider.to_owned()).or_insert(0) += 1;
    }

    /// Finalise into a [`ReferenceDistribution`].
    ///
    /// Returns `None` if no observations have been added.
    pub fn build(self) -> Option<ReferenceDistribution> {
        let n = self.input_lens.len();
        if n == 0 {
            return None;
        }
        let total_tools: usize = self.tool_counts.values().sum();
        let tool_frequencies = if total_tools == 0 {
            HashMap::new()
        } else {
            self.tool_counts
                .into_iter()
                .map(|(k, v)| (k, v as f64 / total_tools as f64))
                .collect()
        };
        let provider_frequencies = self
            .provider_counts
            .into_iter()
            .map(|(k, v)| (k, v as f64 / n as f64))
            .collect();
        Some(ReferenceDistribution {
            input_len: Stats::from_slice(&self.input_lens)?,
            output_len: Stats::from_slice(&self.output_lens)?,
            cost_micros: Stats::from_slice(&self.cost_micros)?,
            latency_ms: Stats::from_slice(&self.latency_ms)?,
            tool_frequencies,
            provider_frequencies,
            n,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_from_slice_basic() {
        let s = Stats::from_slice(&[1.0, 2.0, 3.0]).unwrap();
        assert_eq!(s.mean, 2.0);
        assert_eq!(s.min, 1.0);
        assert_eq!(s.max, 3.0);
        assert!((s.std_dev() - 0.8164965809277261).abs() < 1e-9);
    }

    #[test]
    fn builder_empty_returns_none() {
        let b = ReferenceBuilder::new();
        assert!(b.build().is_none());
    }

    #[test]
    fn builder_builds_correctly() {
        let mut b = ReferenceBuilder::new();
        b.add("hello", "world", 100, 50, &["search".to_string()], "openai");
        b.add("hi", "there", 200, 60, &[], "openai");
        let dist = b.build().unwrap();
        assert_eq!(dist.n, 2);
        assert_eq!(*dist.provider_frequencies.get("openai").unwrap(), 1.0);
        assert_eq!(*dist.tool_frequencies.get("search").unwrap(), 1.0);
    }
}
