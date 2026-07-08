use std::collections::HashMap;

/// A checkpoint snapshot of agent state at a given tick.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub run_id: String,
    pub tick: u64,
    pub data: HashMap<String, String>,
}

impl Checkpoint {
    pub fn new(run_id: &str, tick: u64) -> Self {
        Self {
            run_id: run_id.to_string(),
            tick,
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
}

/// Controls when checkpoints are taken based on tick cadence.
pub struct CheckpointCadence {
    pub interval_ticks: u64,
    last_checkpoint_tick: u64,
}

impl CheckpointCadence {
    pub fn new(interval_ticks: u64) -> Self {
        Self {
            interval_ticks,
            last_checkpoint_tick: 0,
        }
    }

    pub fn should_checkpoint(&mut self, now: u64) -> bool {
        if now >= self.last_checkpoint_tick + self.interval_ticks {
            self.last_checkpoint_tick = now;
            true
        } else {
            false
        }
    }
}
