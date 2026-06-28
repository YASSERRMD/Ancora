use crate::{error::ConfigError, schema::AncoraCfg};

/// Validate a loaded config and return all errors collected.
pub fn validate(cfg: &AncoraCfg) -> Result<(), ConfigError> {
    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&cfg.core.log_level.as_str()) {
        return Err(ConfigError::Validation {
            field: "core.log_level".into(),
            reason: format!("must be one of {:?}", valid_levels),
        });
    }
    if cfg.core.data_dir.is_empty() {
        return Err(ConfigError::Validation {
            field: "core.data_dir".into(),
            reason: "must not be empty".into(),
        });
    }
    if cfg.core.max_concurrent_runs == 0 {
        return Err(ConfigError::Validation {
            field: "core.max_concurrent_runs".into(),
            reason: "must be >= 1".into(),
        });
    }
    if cfg.journal.flush_interval_ms == 0 {
        return Err(ConfigError::Validation {
            field: "journal.flush_interval_ms".into(),
            reason: "must be > 0".into(),
        });
    }
    if cfg.worker.concurrency == 0 {
        return Err(ConfigError::Validation {
            field: "worker.concurrency".into(),
            reason: "must be >= 1".into(),
        });
    }
    if cfg.worker.provider.is_empty() {
        return Err(ConfigError::Validation {
            field: "worker.provider".into(),
            reason: "must not be empty".into(),
        });
    }
    Ok(())
}
