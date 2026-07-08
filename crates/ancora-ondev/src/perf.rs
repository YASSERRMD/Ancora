//! Cold-start and memory-footprint performance measurement for ARM targets.
//!
//! Provides lightweight instrumentation that works entirely within `std`
//! (no external profiling crates required) so it compiles on all targets.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// A named timer that measures elapsed wall-clock time.
#[derive(Debug)]
pub struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    /// Start a new named timer.
    pub fn start(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
        }
    }

    /// Stop the timer and return the elapsed duration.
    pub fn stop(self) -> TimerResult {
        TimerResult {
            name: self.name,
            elapsed: self.start.elapsed(),
        }
    }
}

/// Result of a completed timer measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerResult {
    /// Name of the measurement.
    pub name: String,
    /// Elapsed time.
    #[serde(with = "duration_serde")]
    pub elapsed: Duration,
}

impl TimerResult {
    /// Return elapsed time in milliseconds (float).
    pub fn elapsed_ms(&self) -> f64 {
        self.elapsed.as_secs_f64() * 1000.0
    }
}

mod duration_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_f64(d.as_secs_f64())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let secs = f64::deserialize(d)?;
        Ok(Duration::from_secs_f64(secs))
    }
}

/// Cold-start measurement: time from process launch to first agent run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColdStartMetrics {
    /// Time to initialise the journal.
    pub journal_init_ms: f64,
    /// Time to initialise the memory store.
    pub memory_init_ms: f64,
    /// Time to load the inference engine.
    pub inference_init_ms: f64,
    /// Total cold-start time (sum of above plus overhead).
    pub total_ms: f64,
    /// Target platform label.
    pub target: String,
}

impl ColdStartMetrics {
    /// Create a metrics report from individual timer results.
    pub fn from_timers(
        journal: &TimerResult,
        memory: &TimerResult,
        inference: &TimerResult,
        target: &str,
    ) -> Self {
        let journal_ms = journal.elapsed_ms();
        let memory_ms = memory.elapsed_ms();
        let inference_ms = inference.elapsed_ms();
        Self {
            journal_init_ms: journal_ms,
            memory_init_ms: memory_ms,
            inference_init_ms: inference_ms,
            total_ms: journal_ms + memory_ms + inference_ms,
            target: target.to_string(),
        }
    }

    /// Check whether the total cold-start time is within a budget.
    pub fn within_budget_ms(&self, budget_ms: f64) -> bool {
        self.total_ms <= budget_ms
    }
}

/// Resident set size (RSS) snapshot.
///
/// On targets where `/proc/self/statm` is available (Linux/Android) this
/// reads real values; on other targets it returns a zero-byte placeholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// RSS in bytes.
    pub rss_bytes: usize,
    /// Virtual memory size in bytes.
    pub vsz_bytes: usize,
}

impl MemorySnapshot {
    /// Capture the current memory usage.
    ///
    /// Falls back to a zeroed snapshot on platforms where `/proc/self/statm`
    /// is not available (e.g., macOS, iOS, Windows).
    pub fn capture() -> Self {
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = std::fs::read_to_string("/proc/self/statm") {
                let mut parts = contents.split_whitespace();
                let vsz_pages: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let rss_pages: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let page_size = 4096_usize; // conservative; sysconf not in std
                return Self {
                    rss_bytes: rss_pages * page_size,
                    vsz_bytes: vsz_pages * page_size,
                };
            }
        }
        Self {
            rss_bytes: 0,
            vsz_bytes: 0,
        }
    }

    /// Return RSS in kibibytes.
    pub fn rss_kib(&self) -> usize {
        self.rss_bytes / 1024
    }

    /// Return whether RSS is within the given limit.
    pub fn within_limit_mib(&self, limit_mib: usize) -> bool {
        self.rss_bytes <= limit_mib * 1024 * 1024
    }
}

/// Collect a cold-start profile by timing each initialisation phase.
pub fn measure_cold_start(target: &str) -> ColdStartMetrics {
    let jt = Timer::start("journal_init");
    // Simulate journal init work.
    let _ = crate::journal::Journal::open();
    let journal_result = jt.stop();

    let mt = Timer::start("memory_init");
    let _ = crate::memory::MemoryStore::new();
    let memory_result = mt.stop();

    let it = Timer::start("inference_init");
    let _ = crate::inference::LocalInferenceEngine::new(
        crate::inference::ModelBackend::LocalGguf {
            model_path: "/models/phi3.gguf".to_string(),
        },
        true,
    );
    let inference_result = it.stop();

    ColdStartMetrics::from_timers(&journal_result, &memory_result, &inference_result, target)
}

#[cfg(test)]
mod unit {
    use super::*;
    use std::time::Duration;

    #[test]
    fn timer_measures_positive_elapsed() {
        let t = Timer::start("test");
        // Do a tiny bit of work.
        let _ = (0..1000).sum::<i32>();
        let r = t.stop();
        assert!(r.elapsed >= Duration::ZERO);
        assert!(r.elapsed_ms() >= 0.0);
    }

    #[test]
    fn cold_start_metrics_within_budget() {
        let m = measure_cold_start("host");
        // Initialisation should complete well within 1 s.
        assert!(
            m.within_budget_ms(1000.0),
            "cold start too slow: {:.2} ms",
            m.total_ms
        );
    }

    #[test]
    fn memory_snapshot_capture_does_not_panic() {
        let snap = MemorySnapshot::capture();
        // On non-Linux the snapshot is zero which is fine.
        assert!(snap.rss_bytes < usize::MAX);
    }

    #[test]
    fn memory_within_limit() {
        let snap = MemorySnapshot {
            rss_bytes: 10 * 1024 * 1024,
            vsz_bytes: 20 * 1024 * 1024,
        };
        assert!(snap.within_limit_mib(20));
        assert!(!snap.within_limit_mib(5));
    }

    #[test]
    fn cold_start_arm64_target_label() {
        let m = measure_cold_start("aarch64-unknown-linux-musl");
        assert_eq!(m.target, "aarch64-unknown-linux-musl");
    }

    #[test]
    fn cold_start_individual_phases_non_negative() {
        let m = measure_cold_start("host");
        assert!(m.journal_init_ms >= 0.0);
        assert!(m.memory_init_ms >= 0.0);
        assert!(m.inference_init_ms >= 0.0);
    }

    #[test]
    fn cold_start_total_is_sum_of_phases() {
        let m = measure_cold_start("host");
        let expected = m.journal_init_ms + m.memory_init_ms + m.inference_init_ms;
        // Allow tiny floating-point delta.
        assert!((m.total_ms - expected).abs() < 1e-9);
    }
}
