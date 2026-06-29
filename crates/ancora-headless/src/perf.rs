/// Boot-to-ready performance measurement for the headless agent.
///
/// Tracks and reports the time from process start to the point where
/// the agent signals readiness (socket open, models loaded, API serving).

use std::time::{Duration, Instant};

/// A single timing sample.
#[derive(Debug, Clone)]
pub struct TimingSample {
    pub label: String,
    pub duration: Duration,
}

impl TimingSample {
    pub fn new(label: impl Into<String>, duration: Duration) -> Self {
        TimingSample { label: label.into(), duration }
    }

    pub fn ms(&self) -> u64 {
        self.duration.as_millis() as u64
    }
}

/// Accumulated boot-to-ready timing report.
#[derive(Debug)]
pub struct BootTimingReport {
    pub samples: Vec<TimingSample>,
    pub total: Duration,
    pub target_ms: u64,
}

impl BootTimingReport {
    pub fn new(samples: Vec<TimingSample>, total: Duration, target_ms: u64) -> Self {
        BootTimingReport { samples, total, target_ms }
    }

    pub fn total_ms(&self) -> u64 {
        self.total.as_millis() as u64
    }

    pub fn within_target(&self) -> bool {
        self.total_ms() <= self.target_ms
    }

    pub fn slowest_phase(&self) -> Option<&TimingSample> {
        self.samples.iter().max_by_key(|s| s.duration)
    }

    pub fn summary(&self) -> String {
        let status = if self.within_target() { "PASS" } else { "FAIL" };
        format!(
            "boot-to-ready: {}ms (target: {}ms) [{}]",
            self.total_ms(),
            self.target_ms,
            status
        )
    }
}

/// A stopwatch for measuring individual boot phases.
pub struct PhaseStopwatch {
    label: String,
    start: Instant,
}

impl PhaseStopwatch {
    pub fn start(label: impl Into<String>) -> Self {
        PhaseStopwatch { label: label.into(), start: Instant::now() }
    }

    pub fn stop(self) -> TimingSample {
        TimingSample::new(self.label, self.start.elapsed())
    }
}

/// Accumulates timing samples across the boot sequence.
pub struct BootTimer {
    start: Instant,
    samples: Vec<TimingSample>,
    target_ms: u64,
}

impl BootTimer {
    pub fn new(target_ms: u64) -> Self {
        BootTimer {
            start: Instant::now(),
            samples: Vec::new(),
            target_ms,
        }
    }

    pub fn record(&mut self, sample: TimingSample) {
        self.samples.push(sample);
    }

    pub fn finish(self) -> BootTimingReport {
        let total = self.start.elapsed();
        BootTimingReport::new(self.samples, total, self.target_ms)
    }
}

/// Default boot-to-ready target in milliseconds for the headless agent.
pub const DEFAULT_BOOT_TARGET_MS: u64 = 5_000;

/// Computes the p95 latency from a slice of duration samples.
pub fn p95_ms(samples: &[Duration]) -> u64 {
    if samples.is_empty() {
        return 0;
    }
    let mut sorted: Vec<u64> = samples.iter().map(|d| d.as_millis() as u64).collect();
    sorted.sort_unstable();
    let idx = ((sorted.len() as f64) * 0.95) as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Returns a histogram bucket label for a given duration.
pub fn histogram_bucket(ms: u64) -> &'static str {
    match ms {
        0..=100 => "<100ms",
        101..=500 => "100-500ms",
        501..=1000 => "500ms-1s",
        1001..=5000 => "1s-5s",
        _ => ">5s",
    }
}
