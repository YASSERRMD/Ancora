#[cfg(test)]
mod tests {
    use crate::{error::ConfigError, schema::AncoraCfg, validator::validate};

    #[test]
    fn valid_default_passes() {
        assert!(validate(&AncoraCfg::default()).is_ok());
    }

    #[test]
    fn invalid_log_level_rejected() {
        let mut cfg = AncoraCfg::default();
        cfg.core.log_level = "verbose".into();
        let err = validate(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::Validation { field, .. } if field == "core.log_level"));
    }

    #[test]
    fn empty_data_dir_rejected() {
        let mut cfg = AncoraCfg::default();
        cfg.core.data_dir = String::new();
        let err = validate(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::Validation { field, .. } if field == "core.data_dir"));
    }

    #[test]
    fn zero_concurrency_rejected() {
        let mut cfg = AncoraCfg::default();
        cfg.worker.concurrency = 0;
        let err = validate(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::Validation { field, .. } if field == "worker.concurrency"));
    }

    #[test]
    fn zero_max_concurrent_runs_rejected() {
        let mut cfg = AncoraCfg::default();
        cfg.core.max_concurrent_runs = 0;
        let err = validate(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::Validation { field, .. } if field == "core.max_concurrent_runs"));
    }
}
