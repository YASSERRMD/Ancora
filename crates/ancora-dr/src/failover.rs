use crate::store::JournalStore;
use crate::replication::replicate;
use crate::error::DrError;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Primary,
    Secondary,
    Standby,
}

pub struct FailoverController {
    pub primary_role: Role,
    pub secondary_role: Role,
}

impl FailoverController {
    pub fn new() -> Self {
        Self {
            primary_role: Role::Primary,
            secondary_role: Role::Secondary,
        }
    }

    /// Fence old primary and promote secondary. Returns error if secondary is not current.
    pub fn failover(
        &mut self,
        primary: &mut JournalStore,
        secondary: &mut JournalStore,
        max_allowed_lag: u64,
    ) -> Result<(), DrError> {
        let lag = crate::replication::replication_lag(primary, secondary);
        if lag > max_allowed_lag {
            return Err(DrError::LagTooHigh { lag, max_allowed_lag });
        }
        // Fence primary to prevent further writes (split-brain prevention)
        primary.fence();
        // Promote secondary
        secondary.unfence();
        self.primary_role = Role::Standby;
        self.secondary_role = Role::Primary;
        Ok(())
    }

    /// Failback: re-sync old primary from secondary and restore roles.
    pub fn failback(
        &mut self,
        old_primary: &mut JournalStore,
        new_primary: &mut JournalStore,
    ) -> Result<(), DrError> {
        if self.secondary_role != Role::Primary {
            return Err(DrError::NotInFailoverState);
        }
        // Sync old primary from current primary
        old_primary.unfence();
        replicate(new_primary, old_primary);
        // Restore roles
        self.primary_role = Role::Primary;
        self.secondary_role = Role::Secondary;
        new_primary.fence();
        old_primary.unfence();
        Ok(())
    }
}

impl Default for FailoverController {
    fn default() -> Self {
        Self::new()
    }
}
