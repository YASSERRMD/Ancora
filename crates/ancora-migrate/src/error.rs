use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrateError {
    #[error("migration already applied: {version}")]
    AlreadyApplied { version: u32 },
    #[error("migration not found: {version}")]
    NotFound { version: u32 },
    #[error("version gap: expected {expected}, got {got}")]
    VersionGap { expected: u32, got: u32 },
    #[error("migration failed at version {version}: {reason}")]
    MigrationFailed { version: u32, reason: String },
    #[error("rollback failed at version {version}: {reason}")]
    RollbackFailed { version: u32, reason: String },
    #[error("system is in maintenance mode")]
    MaintenanceMode,
}
