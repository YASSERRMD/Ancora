use std::collections::HashMap;

pub struct RunHeartbeat {
    run_id: String,
    started_at: u64,
    last_tick: u64,
    timeout_secs: u64,
}

impl RunHeartbeat {
    pub fn new(run_id: &str, started_at: u64, timeout_secs: u64) -> Self {
        Self {
            run_id: run_id.to_string(),
            started_at,
            last_tick: started_at,
            timeout_secs,
        }
    }

    pub fn tick(&mut self, now: u64) {
        self.last_tick = now;
    }

    pub fn is_stuck(&self, now: u64) -> bool {
        now.saturating_sub(self.last_tick) > self.timeout_secs
    }
}

pub struct StuckRunDetector {
    runs: HashMap<String, RunHeartbeat>,
}

impl StuckRunDetector {
    pub fn new() -> Self {
        Self { runs: HashMap::new() }
    }

    pub fn register(&mut self, run_id: &str, started_at: u64, timeout_secs: u64) {
        self.runs.insert(
            run_id.to_string(),
            RunHeartbeat::new(run_id, started_at, timeout_secs),
        );
    }

    pub fn tick(&mut self, run_id: &str, now: u64) {
        if let Some(h) = self.runs.get_mut(run_id) {
            h.tick(now);
        }
    }

    pub fn stuck_runs(&self, now: u64) -> Vec<&str> {
        self.runs
            .iter()
            .filter(|(_, h)| h.is_stuck(now))
            .map(|(id, _)| id.as_str())
            .collect()
    }

    pub fn remove(&mut self, run_id: &str) {
        self.runs.remove(run_id);
    }
}

impl Default for StuckRunDetector {
    fn default() -> Self {
        Self::new()
    }
}
