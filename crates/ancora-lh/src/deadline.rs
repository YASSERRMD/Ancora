use crate::error::LhError;

/// Enforces a deadline for a background run.
#[derive(Debug, Clone)]
pub struct Deadline {
    pub run_id: String,
    pub deadline_tick: u64,
}

impl Deadline {
    pub fn new(run_id: &str, deadline_tick: u64) -> Self {
        Self { run_id: run_id.to_string(), deadline_tick }
    }

    pub fn check(&self, now: u64) -> Result<(), LhError> {
        if now > self.deadline_tick {
            Err(LhError::DeadlineExceeded { run_id: self.run_id.clone(), at: now })
        } else {
            Ok(())
        }
    }

    pub fn remaining_ticks(&self, now: u64) -> u64 {
        self.deadline_tick.saturating_sub(now)
    }
}
