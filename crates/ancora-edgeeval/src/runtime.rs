//! Runtime metrics for edge evaluation: latency, memory footprint, power proxy.
//!
//! All measurements are designed to run offline without network access.
//! Latency and footprint are measured via std::time and std::mem utilities.

use std::time::{Duration, Instant};

/// A single on-device latency measurement.
#[derive(Debug, Clone)]
pub struct LatencyMeasurement {
    pub label: String,
    pub duration: Duration,
    pub token_count: u32,
}

impl LatencyMeasurement {
    /// Tokens per second derived from duration and token count.
    pub fn tokens_per_second(&self) -> f64 {
        if self.duration.as_secs_f64() == 0.0 {
            return f64::INFINITY;
        }
        self.token_count as f64 / self.duration.as_secs_f64()
    }

    /// Time to first token (approximated as duration / token_count).
    pub fn time_to_first_token_ms(&self) -> f64 {
        if self.token_count == 0 {
            return self.duration.as_secs_f64() * 1000.0;
        }
        (self.duration.as_secs_f64() / self.token_count as f64) * 1000.0
    }
}

/// On-device latency evaluator.
#[derive(Debug, Default)]
pub struct LatencyEvaluator {
    measurements: Vec<LatencyMeasurement>,
}

impl LatencyEvaluator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a measurement using an explicit duration.
    pub fn record(&mut self, label: impl Into<String>, duration: Duration, token_count: u32) {
        self.measurements.push(LatencyMeasurement {
            label: label.into(),
            duration,
            token_count,
        });
    }

    /// Time a closure and record its result.
    pub fn measure<F: FnOnce() -> u32>(&mut self, label: impl Into<String>, f: F) {
        let start = Instant::now();
        let token_count = f();
        let duration = start.elapsed();
        self.record(label, duration, token_count);
    }

    /// Return all measurements.
    pub fn measurements(&self) -> &[LatencyMeasurement] {
        &self.measurements
    }

    /// Mean latency across all measurements.
    pub fn mean_duration(&self) -> Duration {
        if self.measurements.is_empty() {
            return Duration::ZERO;
        }
        let total_nanos: u128 = self
            .measurements
            .iter()
            .map(|m| m.duration.as_nanos())
            .sum();
        Duration::from_nanos((total_nanos / self.measurements.len() as u128) as u64)
    }

    /// P50 latency.
    pub fn p50_duration(&self) -> Duration {
        self.percentile_duration(50)
    }

    /// P95 latency.
    pub fn p95_duration(&self) -> Duration {
        self.percentile_duration(95)
    }

    fn percentile_duration(&self, pct: usize) -> Duration {
        if self.measurements.is_empty() {
            return Duration::ZERO;
        }
        let mut durations: Vec<u64> = self
            .measurements
            .iter()
            .map(|m| m.duration.as_nanos() as u64)
            .collect();
        durations.sort_unstable();
        let idx = ((pct * durations.len()).saturating_sub(1)) / 100;
        Duration::from_nanos(durations[idx])
    }
}

/// Memory footprint metric.
#[derive(Debug, Clone)]
pub struct MemoryFootprint {
    pub label: String,
    /// Estimated bytes consumed by model weights.
    pub weight_bytes: u64,
    /// Estimated bytes consumed by KV cache.
    pub kv_cache_bytes: u64,
    /// Estimated bytes consumed by activations.
    pub activation_bytes: u64,
}

impl MemoryFootprint {
    pub fn new(
        label: impl Into<String>,
        weight_bytes: u64,
        kv_cache_bytes: u64,
        activation_bytes: u64,
    ) -> Self {
        Self {
            label: label.into(),
            weight_bytes,
            kv_cache_bytes,
            activation_bytes,
        }
    }

    /// Total estimated memory in bytes.
    pub fn total_bytes(&self) -> u64 {
        self.weight_bytes + self.kv_cache_bytes + self.activation_bytes
    }

    /// Total memory in MiB.
    pub fn total_mib(&self) -> f64 {
        self.total_bytes() as f64 / (1024.0 * 1024.0)
    }

    /// Returns true if footprint fits within a given limit in MiB.
    pub fn fits_within_mib(&self, limit_mib: f64) -> bool {
        self.total_mib() <= limit_mib
    }
}

/// Power proxy metric (energy-efficiency estimate for edge devices).
/// Uses tokens-per-joule as a proxy (higher is better).
#[derive(Debug, Clone)]
pub struct PowerProxy {
    pub label: String,
    /// Estimated milliwatt-hours consumed per 1000 tokens.
    pub mwh_per_1k_tokens: f64,
}

impl PowerProxy {
    pub fn new(label: impl Into<String>, mwh_per_1k_tokens: f64) -> Self {
        Self {
            label: label.into(),
            mwh_per_1k_tokens,
        }
    }

    /// Tokens per joule (1 Wh = 3600 J, 1 mWh = 3.6 J).
    pub fn tokens_per_joule(&self) -> f64 {
        if self.mwh_per_1k_tokens == 0.0 {
            return f64::INFINITY;
        }
        1000.0 / (self.mwh_per_1k_tokens * 3.6)
    }

    /// Estimated battery life in hours for a given battery capacity (mWh) and token rate.
    pub fn battery_life_hours(&self, battery_mwh: f64, tokens_per_second: f64) -> f64 {
        if self.mwh_per_1k_tokens == 0.0 || tokens_per_second == 0.0 {
            return f64::INFINITY;
        }
        let power_mw = (tokens_per_second * self.mwh_per_1k_tokens) / 1000.0 * 3600.0;
        if power_mw == 0.0 {
            return f64::INFINITY;
        }
        battery_mwh / power_mw
    }
}
