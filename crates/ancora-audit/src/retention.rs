use crate::log::ImmutableAuditLog;

pub struct RetentionPolicy {
    pub max_age_ticks: u64,
}

impl RetentionPolicy {
    pub fn new(max_age_ticks: u64) -> Self { Self { max_age_ticks } }

    pub fn evict(&self, log: &ImmutableAuditLog, current_tick: u64) -> Vec<u64> {
        let cutoff = current_tick.saturating_sub(self.max_age_ticks);
        log.entries()
            .filter(|e| e.tick < cutoff)
            .map(|e| e.id)
            .collect()
    }

    pub fn count_expired(&self, log: &ImmutableAuditLog, current_tick: u64) -> usize {
        self.evict(log, current_tick).len()
    }
}
