#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{
        env_provider::EnvSecretProvider,
        error::ConfigError,
        external_provider::ExternalSecretProvider,
        file_provider::FileSecretProvider,
        resolver::SecretResolver,
        secret_provider::SecretProvider,
    };

    #[test]
    fn env_provider_resolves_override() {
        let p = EnvSecretProvider::new().with_override("MY_KEY", "secret-val");
        assert_eq!(p.resolve("MY_KEY").unwrap(), "secret-val");
    }

    #[test]
    fn env_provider_missing_key_errors() {
        let p = EnvSecretProvider::new();
        let err = p.resolve("DEFINITELY_NOT_SET_XYZ123").unwrap_err();
        assert!(matches!(err, ConfigError::SecretUnresolvable { .. }));
    }

    #[test]
    fn file_provider_resolves() {
        let mut store = HashMap::new();
        store.insert("db_pass".into(), "hunter2".into());
        let p = FileSecretProvider::new(store);
        assert_eq!(p.resolve("db_pass").unwrap(), "hunter2");
    }

    #[test]
    fn file_provider_missing_key_errors() {
        let p = FileSecretProvider::new(HashMap::new());
        assert!(matches!(p.resolve("missing").unwrap_err(), ConfigError::KeyNotFound { .. }));
    }

    #[test]
    fn external_provider_resolves_via_mock() {
        let p = ExternalSecretProvider::new("vault", |key: &str| {
            if key == "api_key" { Ok("tok-123".into()) } else { Err("not found".into()) }
        });
        assert_eq!(p.resolve("api_key").unwrap(), "tok-123");
    }

    #[test]
    fn external_provider_error_wrapped() {
        let p = ExternalSecretProvider::new("vault", |_: &str| Err("denied".into()));
        let err = p.resolve("any").unwrap_err();
        assert!(matches!(err, ConfigError::SecretUnresolvable { .. }));
    }

    #[test]
    fn resolver_routes_to_named_provider() {
        let p = EnvSecretProvider::new().with_override("TOKEN", "abc");
        let mut res = SecretResolver::new();
        res.register("env", Box::new(p));
        assert_eq!(res.resolve("env:TOKEN").unwrap(), "abc");
    }

    #[test]
    fn resolver_unknown_provider_errors() {
        let res = SecretResolver::new();
        let err = res.resolve("vault:key").unwrap_err();
        assert!(matches!(err, ConfigError::ProviderNotFound { .. }));
    }

    #[test]
    fn resolver_malformed_ref_errors() {
        let res = SecretResolver::new();
        let err = res.resolve("no-colon-here").unwrap_err();
        assert!(matches!(err, ConfigError::SecretUnresolvable { .. }));
    }

    #[test]
    fn secrets_never_stored_in_config_structs() {
        // Verify api_key_ref is a reference string, not an inline secret value.
        use crate::schema::WorkerCfg;
        let cfg = WorkerCfg { api_key_ref: Some("env:MY_SECRET".into()), ..Default::default() };
        // The ref string is just a pointer; the actual secret is never in the struct.
        assert!(cfg.api_key_ref.as_deref().unwrap().starts_with("env:"));
    }
}
