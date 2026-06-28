use crate::sampler::LatencySampler;
use crate::throughput::ThroughputTracker;

/// Soak test harness: runs a synthetic workload for a fixed duration and collects stats.
pub struct SoakHarness {
    pub label: String,
    pub duration_secs: u64,
    pub sampler: LatencySampler,
    pub tracker: ThroughputTracker,
    pub started_at: u64,
    pub finished_at: Option<u64>,
}

impl SoakHarness {
    pub fn new(label: &str, duration_secs: u64, now: u64) -> Self {
        Self {
            label: label.to_string(),
            duration_secs,
            sampler: LatencySampler::new(label),
            tracker: ThroughputTracker::new(10),
            started_at: now,
            finished_at: None,
        }
    }

    pub fn record(&mut self, latency_ms: u64, error: bool, now: u64) {
        self.sampler.record(latency_ms);
        if error {
            self.tracker.record_error(now);
        } else {
            self.tracker.record_ok(now);
        }
    }

    pub fn finish(&mut self, now: u64) {
        self.finished_at = Some(now);
    }

    pub fn elapsed_secs(&self, now: u64) -> u64 {
        now.saturating_sub(self.started_at)
    }

    pub fn is_complete(&self, now: u64) -> bool {
        self.elapsed_secs(now) >= self.duration_secs
    }

    pub fn summary(&self) -> SoakSummary {
        SoakSummary {
            label: self.label.clone(),
            total_requests: self.tracker.total_requests,
            total_errors: self.tracker.total_errors,
            error_rate: self.tracker.error_rate(),
            peak_rps: self.tracker.peak_rps(),
            p50_ms: self.sampler.p50().unwrap_or(0),
            p95_ms: self.sampler.p95().unwrap_or(0),
            p99_ms: self.sampler.p99().unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoakSummary {
    pub label: String,
    pub total_requests: u64,
    pub total_errors: u64,
    pub error_rate: f64,
    pub peak_rps: f64,
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
}

impl SoakSummary {
    pub fn passes_slo(&self, max_error_rate: f64, max_p99_ms: u64) -> bool {
        self.error_rate <= max_error_rate && self.p99_ms <= max_p99_ms
    }
}
