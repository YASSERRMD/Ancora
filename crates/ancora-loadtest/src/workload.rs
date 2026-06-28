/// Describes a synthetic workload for load testing.
#[derive(Debug, Clone)]
pub struct WorkloadSpec {
    pub name: String,
    pub target_rps: f64,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub payload_size_bytes: usize,
}

impl WorkloadSpec {
    pub fn new(name: &str, target_rps: f64, duration_secs: u64, concurrency: usize) -> Self {
        Self {
            name: name.to_string(),
            target_rps,
            duration_secs,
            concurrency,
            payload_size_bytes: 256,
        }
    }

    pub fn with_payload(mut self, bytes: usize) -> Self {
        self.payload_size_bytes = bytes;
        self
    }

    pub fn total_expected_requests(&self) -> u64 {
        (self.target_rps * self.duration_secs as f64) as u64
    }
}
