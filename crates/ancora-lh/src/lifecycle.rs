/// States of a long-horizon background run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunState {
    Created,
    Running,
    Sleeping { until_tick: u64 },
    Woken,
    Completed,
    Failed(String),
}

/// Lifecycle tracker for a long-horizon background run.
#[derive(Debug, Clone)]
pub struct BackgroundRun {
    pub run_id: String,
    pub state: RunState,
    pub started_at: u64,
    pub effects_applied: Vec<String>,
}

impl BackgroundRun {
    pub fn new(run_id: &str, started_at: u64) -> Self {
        Self {
            run_id: run_id.to_string(),
            state: RunState::Created,
            started_at,
            effects_applied: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.state = RunState::Running;
    }

    pub fn sleep_until(&mut self, tick: u64) {
        self.state = RunState::Sleeping { until_tick: tick };
    }

    pub fn wake(&mut self, now: u64) -> bool {
        if let RunState::Sleeping { until_tick } = self.state {
            if now >= until_tick {
                self.state = RunState::Woken;
                return true;
            }
        }
        false
    }

    pub fn complete(&mut self) {
        self.state = RunState::Completed;
    }

    pub fn fail(&mut self, reason: &str) {
        self.state = RunState::Failed(reason.to_string());
    }

    pub fn apply_effect(&mut self, effect: &str) -> bool {
        if self.effects_applied.contains(&effect.to_string()) {
            return false;
        }
        self.effects_applied.push(effect.to_string());
        true
    }
}
