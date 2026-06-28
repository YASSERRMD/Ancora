use std::collections::HashMap;

/// Persists progress state for long-horizon runs.
#[derive(Debug, Default)]
pub struct ProgressStore {
    store: HashMap<String, RunProgress>,
}

#[derive(Debug, Clone)]
pub struct RunProgress {
    pub run_id: String,
    pub steps_completed: u32,
    pub total_steps: u32,
    pub last_tick: u64,
}

impl RunProgress {
    pub fn new(run_id: &str, total_steps: u32) -> Self {
        Self { run_id: run_id.to_string(), steps_completed: 0, total_steps, last_tick: 0 }
    }

    pub fn pct_complete(&self) -> f64 {
        if self.total_steps == 0 { return 100.0; }
        (self.steps_completed as f64 / self.total_steps as f64) * 100.0
    }
}

impl ProgressStore {
    pub fn init(&mut self, run_id: &str, total_steps: u32) {
        self.store.insert(run_id.to_string(), RunProgress::new(run_id, total_steps));
    }

    pub fn advance(&mut self, run_id: &str, tick: u64) -> Option<&RunProgress> {
        let p = self.store.get_mut(run_id)?;
        if p.steps_completed < p.total_steps {
            p.steps_completed += 1;
        }
        p.last_tick = tick;
        self.store.get(run_id)
    }

    pub fn get(&self, run_id: &str) -> Option<&RunProgress> {
        self.store.get(run_id)
    }
}
