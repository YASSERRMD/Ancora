use crate::error::LhError;

/// Resource throttle limiting operation rate for background runs.
#[derive(Debug)]
pub struct Throttle {
    pub max_ops_per_tick: u32,
    ops_this_tick: u32,
    current_tick: u64,
}

impl Throttle {
    pub fn new(max_ops_per_tick: u32) -> Self {
        Self { max_ops_per_tick, ops_this_tick: 0, current_tick: 0 }
    }

    pub fn try_op(&mut self, now: u64) -> Result<(), LhError> {
        if now != self.current_tick {
            self.current_tick = now;
            self.ops_this_tick = 0;
        }
        if self.ops_this_tick >= self.max_ops_per_tick {
            return Err(LhError::Throttled { ops_this_tick: self.ops_this_tick, max: self.max_ops_per_tick });
        }
        self.ops_this_tick += 1;
        Ok(())
    }

    pub fn ops_this_tick(&self) -> u32 {
        self.ops_this_tick
    }
}
