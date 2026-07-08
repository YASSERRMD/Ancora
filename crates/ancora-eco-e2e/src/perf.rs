/// Performance measurement for plugin overhead.
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct PerfSample {
    pub label: String,
    pub duration: Duration,
    pub iterations: u64,
}

impl PerfSample {
    pub fn new(label: &str, duration: Duration, iterations: u64) -> Self {
        PerfSample {
            label: label.to_string(),
            duration,
            iterations,
        }
    }

    pub fn avg_ns(&self) -> u64 {
        if self.iterations == 0 {
            return 0;
        }
        self.duration.as_nanos() as u64 / self.iterations
    }

    pub fn throughput_per_sec(&self) -> f64 {
        let secs = self.duration.as_secs_f64();
        if secs == 0.0 {
            return 0.0;
        }
        self.iterations as f64 / secs
    }
}

#[derive(Debug, Default)]
pub struct PerfReport {
    samples: Vec<PerfSample>,
}

impl PerfReport {
    pub fn new() -> Self {
        PerfReport {
            samples: Vec::new(),
        }
    }

    pub fn add(&mut self, sample: PerfSample) {
        self.samples.push(sample);
    }

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn max_avg_ns(&self) -> Option<u64> {
        self.samples.iter().map(|s| s.avg_ns()).max()
    }

    pub fn min_avg_ns(&self) -> Option<u64> {
        self.samples.iter().map(|s| s.avg_ns()).min()
    }

    pub fn all_within_budget(&self, budget_ns: u64) -> bool {
        self.samples.iter().all(|s| s.avg_ns() <= budget_ns)
    }
}

pub fn measure<F: FnMut()>(label: &str, iterations: u64, mut f: F) -> PerfSample {
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();
    PerfSample::new(label, duration, iterations)
}
