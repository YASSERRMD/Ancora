#[cfg(test)]
mod tests {
    use crate::{
        env_provider::EnvSecretProvider, error::ConfigError, resolver::SecretResolver,
        rotation::RotationLog, secret_provider::SecretProvider,
    };

    #[test]
    fn rotation_invalidates_old_secret() {
        let mut p = EnvSecretProvider::new().with_override("KEY", "old-secret");
        assert_eq!(p.resolve("KEY").unwrap(), "old-secret");
        p.on_rotation("KEY");
        let err = p.resolve("KEY").unwrap_err();
        assert!(matches!(err, ConfigError::SecretUnresolvable { .. }));
    }

    #[test]
    fn rotation_log_records_event() {
        let mut log = RotationLog::default();
        log.record("env", "API_KEY", 1000);
        let rec = log.last_rotation_for("API_KEY").unwrap();
        assert_eq!(rec.provider, "env");
        assert_eq!(rec.rotated_at_secs, 1000);
    }

    #[test]
    fn rotation_log_last_rotation_latest() {
        let mut log = RotationLog::default();
        log.record("env", "API_KEY", 1000);
        log.record("env", "API_KEY", 2000);
        let rec = log.last_rotation_for("API_KEY").unwrap();
        assert_eq!(rec.rotated_at_secs, 2000);
    }

    #[test]
    fn resolver_notifies_rotation() {
        let p = EnvSecretProvider::new().with_override("K", "v");
        let mut res = SecretResolver::new();
        res.register("env", Box::new(p));
        assert!(res.resolve("env:K").is_ok());
        res.notify_rotation("env", "K").unwrap();
        assert!(matches!(
            res.resolve("env:K").unwrap_err(),
            ConfigError::SecretUnresolvable { .. }
        ));
    }
}
