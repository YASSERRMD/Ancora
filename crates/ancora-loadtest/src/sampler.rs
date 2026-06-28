/// Records raw latency samples for a single operation.
pub struct LatencySampler {
    pub label: String,
    samples: Vec<u64>,
}

impl LatencySampler {
    pub fn new(label: &str) -> Self {
        Self { label: label.to_string(), samples: vec![] }
    }

    pub fn record(&mut self, latency_ms: u64) {
        self.samples.push(latency_ms);
    }

    pub fn count(&self) -> usize {
        self.samples.len()
    }

    pub fn min(&self) -> Option<u64> {
        self.samples.iter().copied().min()
    }

    pub fn max(&self) -> Option<u64> {
        self.samples.iter().copied().max()
    }

    pub fn mean(&self) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        Some(self.samples.iter().sum::<u64>() as f64 / self.samples.len() as f64)
    }

    pub fn percentile(&self, pct: f64) -> Option<u64> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_unstable();
        let idx = ((pct / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
        Some(sorted[idx.min(sorted.len() - 1)])
    }

    pub fn p50(&self) -> Option<u64> { self.percentile(50.0) }
    pub fn p95(&self) -> Option<u64> { self.percentile(95.0) }
    pub fn p99(&self) -> Option<u64> { self.percentile(99.0) }
}
