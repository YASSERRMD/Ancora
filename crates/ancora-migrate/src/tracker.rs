use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppliedStatus {
    Applied { at_secs: u64 },
    RolledBack { at_secs: u64 },
}

pub struct MigrationTracker {
    applied: HashMap<u32, AppliedStatus>,
}

impl MigrationTracker {
    pub fn new() -> Self {
        Self { applied: HashMap::new() }
    }

    pub fn mark_applied(&mut self, version: u32, at: u64) {
        self.applied.insert(version, AppliedStatus::Applied { at_secs: at });
    }

    pub fn mark_rolled_back(&mut self, version: u32, at: u64) {
        self.applied.insert(version, AppliedStatus::RolledBack { at_secs: at });
    }

    pub fn is_applied(&self, version: u32) -> bool {
        matches!(self.applied.get(&version), Some(AppliedStatus::Applied { .. }))
    }

    pub fn current_version(&self) -> u32 {
        self.applied
            .iter()
            .filter(|(_, s)| matches!(s, AppliedStatus::Applied { .. }))
            .map(|(v, _)| *v)
            .max()
            .unwrap_or(0)
    }

    pub fn applied_count(&self) -> usize {
        self.applied
            .values()
            .filter(|s| matches!(s, AppliedStatus::Applied { .. }))
            .count()
    }
}

impl Default for MigrationTracker {
    fn default() -> Self {
        Self::new()
    }
}
