use crate::error::MigrateError;
use crate::registry::MigrationRegistry;
use crate::tracker::MigrationTracker;

pub struct MigrationRunner {
    registry: MigrationRegistry,
    tracker: MigrationTracker,
}

impl MigrationRunner {
    pub fn new(registry: MigrationRegistry) -> Self {
        Self {
            registry,
            tracker: MigrationTracker::new(),
        }
    }

    pub fn migrate_to(&mut self, target: u32, now: u64) -> Result<u32, MigrateError> {
        let versions = self.registry.versions_asc();
        let pending: Vec<u32> = versions
            .into_iter()
            .filter(|&v| v <= target && !self.tracker.is_applied(v))
            .collect();

        let mut applied = 0u32;
        for v in pending {
            let m = self
                .registry
                .get(v)
                .ok_or(MigrateError::NotFound { version: v })?;
            m.apply()
                .map_err(|reason| MigrateError::MigrationFailed { version: v, reason })?;
            self.tracker.mark_applied(v, now);
            applied += 1;
        }
        Ok(applied)
    }

    pub fn rollback_to(&mut self, target: u32, now: u64) -> Result<u32, MigrateError> {
        let versions = self.registry.versions_asc();
        let to_rollback: Vec<u32> = versions
            .into_iter()
            .filter(|&v| v > target && self.tracker.is_applied(v))
            .rev()
            .collect();

        let mut rolled = 0u32;
        for v in to_rollback {
            let m = self
                .registry
                .get(v)
                .ok_or(MigrateError::NotFound { version: v })?;
            m.rollback()
                .map_err(|reason| MigrateError::RollbackFailed { version: v, reason })?;
            self.tracker.mark_rolled_back(v, now);
            rolled += 1;
        }
        Ok(rolled)
    }

    pub fn current_version(&self) -> u32 {
        self.tracker.current_version()
    }

    pub fn applied_count(&self) -> usize {
        self.tracker.applied_count()
    }
}
