/// Liveness probe: reports whether the process is alive and not deadlocked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeStatus {
    Alive,
    Dead { reason: String },
}

pub struct LivenessProbe {
    pub max_stall_secs: u64,
    pub last_heartbeat_secs: u64,
}

impl LivenessProbe {
    pub fn new(max_stall_secs: u64) -> Self {
        Self {
            max_stall_secs,
            last_heartbeat_secs: 0,
        }
    }

    pub fn heartbeat(&mut self, now: u64) {
        self.last_heartbeat_secs = now;
    }

    pub fn check(&self, now: u64) -> ProbeStatus {
        let elapsed = now.saturating_sub(self.last_heartbeat_secs);
        if elapsed > self.max_stall_secs {
            ProbeStatus::Dead {
                reason: format!(
                    "no heartbeat for {elapsed}s (limit {}s)",
                    self.max_stall_secs
                ),
            }
        } else {
            ProbeStatus::Alive
        }
    }
}

/// Readiness probe: reports whether the process can accept work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadinessStatus {
    Ready,
    NotReady { reason: String },
}

pub struct ReadinessProbe {
    pub deps_healthy: bool,
    pub queue_saturated: bool,
}

impl ReadinessProbe {
    pub fn new() -> Self {
        Self {
            deps_healthy: true,
            queue_saturated: false,
        }
    }

    pub fn check(&self) -> ReadinessStatus {
        if !self.deps_healthy {
            return ReadinessStatus::NotReady {
                reason: "one or more dependencies unhealthy".into(),
            };
        }
        if self.queue_saturated {
            return ReadinessStatus::NotReady {
                reason: "queue saturated".into(),
            };
        }
        ReadinessStatus::Ready
    }
}

impl Default for ReadinessProbe {
    fn default() -> Self {
        Self::new()
    }
}
