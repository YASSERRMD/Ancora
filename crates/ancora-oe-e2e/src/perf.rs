/// Performance measurement module: measures observability overhead.

use std::time::Instant;

/// Result of a single overhead measurement.
#[derive(Debug, Clone)]
pub struct OverheadMeasurement {
    pub label: String,
    pub baseline_ns: u64,
    pub instrumented_ns: u64,
}

impl OverheadMeasurement {
    pub fn new(label: impl Into<String>, baseline_ns: u64, instrumented_ns: u64) -> Self {
        Self {
            label: label.into(),
            baseline_ns,
            instrumented_ns,
        }
    }

    pub fn overhead_ns(&self) -> u64 {
        self.instrumented_ns.saturating_sub(self.baseline_ns)
    }

    pub fn overhead_fraction(&self) -> f64 {
        if self.baseline_ns == 0 {
            return 0.0;
        }
        self.overhead_ns() as f64 / self.baseline_ns as f64
    }

    pub fn overhead_pct(&self) -> f64 {
        self.overhead_fraction() * 100.0
    }

    /// Returns true if overhead is within the acceptable fraction (e.g. 0.05 for 5%).
    pub fn within_budget(&self, max_fraction: f64) -> bool {
        self.overhead_fraction() <= max_fraction
    }
}

/// Benchmark report aggregating multiple measurements.
#[derive(Debug, Default)]
pub struct BenchmarkReport {
    pub measurements: Vec<OverheadMeasurement>,
}

impl BenchmarkReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, m: OverheadMeasurement) {
        self.measurements.push(m);
    }

    pub fn mean_overhead_pct(&self) -> f64 {
        if self.measurements.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.measurements.iter().map(|m| m.overhead_pct()).sum();
        sum / self.measurements.len() as f64
    }

    pub fn all_within_budget(&self, max_fraction: f64) -> bool {
        self.measurements.iter().all(|m| m.within_budget(max_fraction))
    }

    pub fn worst_case(&self) -> Option<&OverheadMeasurement> {
        self.measurements.iter().max_by(|a, b| {
            a.overhead_fraction()
                .partial_cmp(&b.overhead_fraction())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

/// Times a closure and returns elapsed nanoseconds.
pub fn time_ns<F: FnOnce()>(f: F) -> u64 {
    let start = Instant::now();
    f();
    start.elapsed().as_nanos() as u64
}

/// Simulates measuring observability overhead for span creation.
pub fn measure_span_overhead(iterations: usize) -> OverheadMeasurement {
    // Baseline: no-op loop.
    let baseline_ns = time_ns(|| {
        let mut sum = 0u64;
        for i in 0..iterations {
            sum = sum.wrapping_add(i as u64);
        }
        let _ = sum;
    });

    // Instrumented: loop with span bookkeeping.
    let instrumented_ns = time_ns(|| {
        let mut spans: Vec<(u64, u64)> = Vec::with_capacity(iterations);
        for i in 0..iterations {
            let start = i as u64 * 1000;
            let end = start + 100;
            spans.push((start, end));
        }
        let _ = spans;
    });

    OverheadMeasurement::new("span_creation", baseline_ns, instrumented_ns)
}
