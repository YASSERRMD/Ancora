/// Advisory migration lock: prevents two runners from migrating simultaneously.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockState {
    Free,
    Held { holder: String, acquired_at: u64 },
}

pub struct MigrationLock {
    state: LockState,
    pub ttl_secs: u64,
}

impl MigrationLock {
    pub fn new(ttl_secs: u64) -> Self {
        Self { state: LockState::Free, ttl_secs }
    }

    pub fn acquire(&mut self, holder: &str, now: u64) -> bool {
        if self.is_free(now) {
            self.state = LockState::Held { holder: holder.to_string(), acquired_at: now };
            true
        } else {
            false
        }
    }

    pub fn release(&mut self, holder: &str) -> bool {
        if let LockState::Held { holder: h, .. } = &self.state {
            if h == holder {
                self.state = LockState::Free;
                return true;
            }
        }
        false
    }

    pub fn is_free(&self, now: u64) -> bool {
        match &self.state {
            LockState::Free => true,
            LockState::Held { acquired_at, .. } => {
                now.saturating_sub(*acquired_at) >= self.ttl_secs
            }
        }
    }

    pub fn holder(&self) -> Option<&str> {
        if let LockState::Held { holder, .. } = &self.state {
            Some(holder.as_str())
        } else {
            None
        }
    }
}
