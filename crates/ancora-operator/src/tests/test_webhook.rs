#[cfg(test)]
mod tests {
    use crate::crd::cluster::{AncoraClusterSpec, JournalStoreConfig};
    use crate::crd::tenant::AncoraTenantSpec;
    use crate::webhook::{validate_cluster, validate_tenant};
    use std::collections::HashMap;

    fn valid_cluster_spec() -> AncoraClusterSpec {
        AncoraClusterSpec {
            control_plane_replicas: 2,
            worker_replicas: 4,
            worker_concurrency: 8,
            journal_store: JournalStoreConfig {
                backend: "sqlite".to_string(),
                connection_secret_ref: None,
            },
            autoscaler_enabled: true,
            min_workers: 1,
            max_workers: 10,
            image: "ancora:latest".to_string(),
            secret_refs: HashMap::new(),
        }
    }

    #[test]
    fn valid_cluster_spec_accepted() {
        validate_cluster(&valid_cluster_spec()).unwrap();
    }

    #[test]
    fn zero_control_plane_replicas_rejected() {
        let mut s = valid_cluster_spec();
        s.control_plane_replicas = 0;
        assert!(validate_cluster(&s).is_err());
    }

    #[test]
    fn min_greater_than_max_rejected() {
        let mut s = valid_cluster_spec();
        s.min_workers = 5;
        s.max_workers = 3;
        assert!(validate_cluster(&s).is_err());
    }

    #[test]
    fn empty_journal_backend_rejected() {
        let mut s = valid_cluster_spec();
        s.journal_store.backend = String::new();
        assert!(validate_cluster(&s).is_err());
    }

    #[test]
    fn invalid_tenant_cr_rejected_by_webhook() {
        let spec = AncoraTenantSpec {
            cluster_ref: String::new(),
            max_workers: 0,
            provider_allowlist: vec![],
            residency_region: None,
            admin_role_binding: None,
        };
        assert!(validate_tenant(&spec).is_err());
    }

    #[test]
    fn valid_tenant_spec_accepted() {
        let spec = AncoraTenantSpec {
            cluster_ref: "my-cluster".to_string(),
            max_workers: 5,
            provider_allowlist: vec![],
            residency_region: None,
            admin_role_binding: None,
        };
        validate_tenant(&spec).unwrap();
    }
}
