/// Scheduled wakeup at a given monotonic tick.
#[derive(Debug, Clone)]
pub struct ScheduledWakeup {
    pub run_id: String,
    pub wake_at_tick: u64,
}

impl ScheduledWakeup {
    pub fn new(run_id: &str, wake_at_tick: u64) -> Self {
        Self {
            run_id: run_id.to_string(),
            wake_at_tick,
        }
    }

    pub fn should_fire(&self, now: u64) -> bool {
        now >= self.wake_at_tick
    }
}

/// Event-driven wakeup triggered by a named signal.
#[derive(Debug, Clone)]
pub struct EventWakeup {
    pub run_id: String,
    pub event_name: String,
    fired: bool,
}

impl EventWakeup {
    pub fn new(run_id: &str, event_name: &str) -> Self {
        Self {
            run_id: run_id.to_string(),
            event_name: event_name.to_string(),
            fired: false,
        }
    }

    pub fn trigger(&mut self, event_name: &str) -> bool {
        if self.event_name == event_name && !self.fired {
            self.fired = true;
            true
        } else {
            false
        }
    }

    pub fn has_fired(&self) -> bool {
        self.fired
    }
}
