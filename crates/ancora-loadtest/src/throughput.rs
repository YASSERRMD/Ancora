/// Tracks request throughput over time using fixed-width time buckets.
pub struct ThroughputTracker {
    pub bucket_secs: u64,
    buckets: std::collections::BTreeMap<u64, u64>,
    pub total_requests: u64,
    pub total_errors: u64,
}

impl ThroughputTracker {
    pub fn new(bucket_secs: u64) -> Self {
        Self {
            bucket_secs,
            buckets: Default::default(),
            total_requests: 0,
            total_errors: 0,
        }
    }

    fn bucket_key(&self, ts: u64) -> u64 {
        ts / self.bucket_secs
    }

    pub fn record_ok(&mut self, ts: u64) {
        *self.buckets.entry(self.bucket_key(ts)).or_insert(0) += 1;
        self.total_requests += 1;
    }

    pub fn record_error(&mut self, ts: u64) {
        *self.buckets.entry(self.bucket_key(ts)).or_insert(0) += 1;
        self.total_requests += 1;
        self.total_errors += 1;
    }

    pub fn rps_in_bucket(&self, ts: u64) -> f64 {
        let key = self.bucket_key(ts);
        let count = self.buckets.get(&key).copied().unwrap_or(0);
        count as f64 / self.bucket_secs as f64
    }

    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_errors as f64 / self.total_requests as f64
    }

    pub fn peak_rps(&self) -> f64 {
        self.buckets.values().copied().max().unwrap_or(0) as f64 / self.bucket_secs as f64
    }
}
