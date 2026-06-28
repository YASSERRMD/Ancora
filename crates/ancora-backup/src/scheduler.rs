/// A scheduled backup job configuration.
#[derive(Debug, Clone)]
pub struct BackupSchedule {
    pub interval_secs: u64,
    pub last_run_at: u64,
}

impl BackupSchedule {
    pub fn new(interval_secs: u64) -> Self {
        Self { interval_secs, last_run_at: 0 }
    }

    pub fn is_due(&self, now: u64) -> bool {
        now >= self.last_run_at + self.interval_secs
    }

    pub fn record_run(&mut self, now: u64) {
        self.last_run_at = now;
    }
}
