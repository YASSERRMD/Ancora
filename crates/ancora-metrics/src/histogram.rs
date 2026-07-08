/// A simple histogram with configurable bucket boundaries (in ms).
#[derive(Clone, Debug)]
pub struct Histogram {
    pub label: String,
    pub buckets: Vec<u64>,
    counts: Vec<u64>,
    sum: u64,
    total: u64,
}

impl Histogram {
    /// Create a histogram with bucket upper bounds in ms.
    pub fn new(label: impl Into<String>, buckets: Vec<u64>) -> Self {
        let n = buckets.len();
        Self {
            label: label.into(),
            buckets,
            counts: vec![0; n],
            sum: 0,
            total: 0,
        }
    }

    /// Record a latency observation in ms.
    pub fn observe(&mut self, value_ms: u64) {
        for (i, &bound) in self.buckets.iter().enumerate() {
            if value_ms <= bound {
                self.counts[i] += 1;
            }
        }
        self.sum += value_ms;
        self.total += 1;
    }

    pub fn count(&self) -> u64 {
        self.total
    }

    pub fn sum_ms(&self) -> u64 {
        self.sum
    }

    pub fn mean_ms(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.sum as f64 / self.total as f64
        }
    }

    /// Return count of observations within the given bucket bound.
    pub fn bucket_count(&self, bound_ms: u64) -> u64 {
        self.buckets
            .iter()
            .position(|&b| b == bound_ms)
            .map(|i| self.counts[i])
            .unwrap_or(0)
    }
}
