//! Benchmark harness.
//!
//! Provides a lightweight, deterministic harness for running timed
//! microbenchmarks without relying on `criterion` or any external crate. The
//! harness runs each benchmark function a configurable number of times and
//! collects timing statistics.

use std::time::{Duration, Instant};

/// Configuration for a benchmark run.
#[derive(Debug, Clone)]
pub struct BenchConfig {
    /// Number of warm-up iterations (results discarded).
    pub warmup_iters: u32,
    /// Number of measured iterations.
    pub measure_iters: u32,
    /// Human-readable benchmark name.
    pub name: String,
}

impl BenchConfig {
    /// Create a default configuration with 3 warm-up and 10 measured iterations.
    pub fn new(name: &str) -> Self {
        Self {
            warmup_iters: 3,
            measure_iters: 10,
            name: name.to_owned(),
        }
    }

    /// Set the number of warm-up iterations.
    pub fn with_warmup(mut self, n: u32) -> Self {
        self.warmup_iters = n;
        self
    }

    /// Set the number of measured iterations.
    pub fn with_iters(mut self, n: u32) -> Self {
        self.measure_iters = n;
        self
    }
}

/// Statistical summary of a benchmark run.
#[derive(Debug, Clone)]
pub struct BenchStats {
    /// Benchmark name.
    pub name: String,
    /// Minimum observed duration.
    pub min: Duration,
    /// Maximum observed duration.
    pub max: Duration,
    /// Arithmetic mean.
    pub mean: Duration,
    /// Median.
    pub median: Duration,
    /// Number of measured samples.
    pub sample_count: u32,
}

impl BenchStats {
    /// Returns `true` if the mean is at or below the given threshold.
    pub fn mean_within(&self, threshold: Duration) -> bool {
        self.mean <= threshold
    }
}

/// Run the benchmark defined by `f` according to `config`.
///
/// The closure `f` receives no arguments and should return `()`. Its execution
/// is timed per iteration.
pub fn run_bench<F>(config: &BenchConfig, mut f: F) -> BenchStats
where
    F: FnMut(),
{
    // Warm-up phase: results are discarded.
    for _ in 0..config.warmup_iters {
        f();
    }

    // Measurement phase.
    let mut samples: Vec<Duration> = Vec::with_capacity(config.measure_iters as usize);
    for _ in 0..config.measure_iters {
        let t = Instant::now();
        f();
        samples.push(t.elapsed());
    }

    // Compute statistics.
    samples.sort();
    let min = *samples.first().unwrap_or(&Duration::ZERO);
    let max = *samples.last().unwrap_or(&Duration::ZERO);
    let total_ns: u128 = samples.iter().map(|d| d.as_nanos()).sum();
    let mean = Duration::from_nanos((total_ns / samples.len().max(1) as u128) as u64);
    let median = if samples.is_empty() {
        Duration::ZERO
    } else {
        samples[samples.len() / 2]
    };

    BenchStats {
        name: config.name.clone(),
        min,
        max,
        mean,
        median,
        sample_count: samples.len() as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_collects_samples() {
        let cfg = BenchConfig::new("noop").with_warmup(1).with_iters(5);
        let stats = run_bench(&cfg, || {
            let _ = 1 + 1;
        });
        assert_eq!(stats.sample_count, 5);
        assert_eq!(stats.name, "noop");
    }

    #[test]
    fn min_le_max() {
        let cfg = BenchConfig::new("check").with_warmup(0).with_iters(4);
        let stats = run_bench(&cfg, || {
            let _ = vec![0u8; 64];
        });
        assert!(stats.min <= stats.max);
    }
}
