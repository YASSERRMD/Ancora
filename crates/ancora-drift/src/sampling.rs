//! Online quality sampling for drift monitoring.
//!
//! Collects a fraction of live traces for offline evaluation and reference
//! distribution construction.

use std::collections::VecDeque;

/// Configuration for the sampling subsystem.
#[derive(Debug, Clone)]
pub struct SamplingConfig {
    /// Fraction of requests to sample (0.0 - 1.0).
    pub rate: f64,
    /// Maximum number of traces to buffer before flushing.
    pub buffer_size: usize,
    /// Seed for deterministic sampling in tests.
    pub seed: u64,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            rate: 0.05,
            buffer_size: 1_000,
            seed: 42,
        }
    }
}

/// A single captured trace.
#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    /// Unique identifier for this trace.
    pub id: String,
    /// Raw input text sent to the model.
    pub input: String,
    /// Raw output text returned by the model.
    pub output: String,
    /// Total cost in micro-dollars.
    pub cost_micros: u64,
    /// Wall-clock duration in milliseconds.
    pub latency_ms: u64,
    /// Name of the provider used.
    pub provider: String,
    /// Names of tools called during the trace.
    pub tools_called: Vec<String>,
}

/// Reservoir-based online sampler.
///
/// Uses a simple linear-congruential generator so results are reproducible
/// without pulling in external crates.
pub struct Sampler {
    config: SamplingConfig,
    buffer: VecDeque<Trace>,
    /// LCG state for pseudo-random sampling decisions.
    rng_state: u64,
    total_seen: u64,
}

impl Sampler {
    /// Create a new sampler with the given configuration.
    pub fn new(config: SamplingConfig) -> Self {
        let rng_state = config.seed;
        Self {
            config,
            buffer: VecDeque::new(),
            rng_state,
            total_seen: 0,
        }
    }

    /// Advance the LCG and return a value in [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        // LCG parameters from Knuth Vol. 2
        self.rng_state = self
            .rng_state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.rng_state >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Offer a trace to the sampler; it may or may not be retained.
    ///
    /// Returns `true` if the trace was accepted into the buffer.
    pub fn offer(&mut self, trace: Trace) -> bool {
        self.total_seen += 1;
        if self.next_f64() < self.config.rate {
            if self.buffer.len() >= self.config.buffer_size {
                self.buffer.pop_front();
            }
            self.buffer.push_back(trace);
            true
        } else {
            false
        }
    }

    /// Drain all buffered traces, clearing the buffer.
    pub fn drain(&mut self) -> Vec<Trace> {
        self.buffer.drain(..).collect()
    }

    /// Number of traces currently buffered.
    pub fn buffered(&self) -> usize {
        self.buffer.len()
    }

    /// Total traces offered (sampled or not).
    pub fn total_seen(&self) -> u64 {
        self.total_seen
    }
}

/// A trace converted into an eval case for offline evaluation.
#[derive(Debug, Clone)]
pub struct EvalCase {
    /// Identifier matching the originating trace.
    pub trace_id: String,
    /// Input prompt passed to the model.
    pub input: String,
    /// Actual model output.
    pub actual_output: String,
    /// Optional human or automated label added after sampling.
    pub expected_output: Option<String>,
    /// Cost in micro-dollars.
    pub cost_micros: u64,
    /// Provider that served the trace.
    pub provider: String,
}

impl From<Trace> for EvalCase {
    fn from(t: Trace) -> Self {
        EvalCase {
            trace_id: t.id,
            input: t.input,
            actual_output: t.output,
            expected_output: None,
            cost_micros: t.cost_micros,
            provider: t.provider,
        }
    }
}

/// Drain the sampler and convert all buffered traces into eval cases.
pub fn drain_as_eval_cases(sampler: &mut Sampler) -> Vec<EvalCase> {
    sampler.drain().into_iter().map(EvalCase::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampling_rate_roughly_correct() {
        let cfg = SamplingConfig { rate: 0.1, buffer_size: 10_000, seed: 0 };
        let mut sampler = Sampler::new(cfg);
        let n = 10_000u32;
        let mut accepted = 0u32;
        for i in 0..n {
            let t = Trace {
                id: i.to_string(),
                input: "hello".into(),
                output: "world".into(),
                cost_micros: 100,
                latency_ms: 50,
                provider: "test".into(),
                tools_called: vec![],
            };
            if sampler.offer(t) {
                accepted += 1;
            }
        }
        let rate = accepted as f64 / n as f64;
        // Allow +-3% tolerance
        assert!(rate > 0.07 && rate < 0.13, "rate out of range: {rate}");
    }

    #[test]
    fn drain_clears_buffer() {
        let cfg = SamplingConfig { rate: 1.0, buffer_size: 100, seed: 1 };
        let mut sampler = Sampler::new(cfg);
        for i in 0..10 {
            sampler.offer(Trace {
                id: i.to_string(),
                input: "x".into(),
                output: "y".into(),
                cost_micros: 0,
                latency_ms: 0,
                provider: "p".into(),
                tools_called: vec![],
            });
        }
        let drained = sampler.drain();
        assert_eq!(drained.len(), 10);
        assert_eq!(sampler.buffered(), 0);
    }
}
