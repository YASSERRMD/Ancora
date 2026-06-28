#[cfg(test)]
mod tests {
    use crate::crd::cluster::ObjectMeta;
    use crate::crd::tenant::{AncoraTenant, AncoraTenantSpec, AncoraTenantStatus};
    use crate::reconciler::Reconciler;
    use std::collections::HashMap;

    fn make_tenant(name: &str) -> AncoraTenant {
        AncoraTenant {
            metadata: ObjectMeta {
                name: name.to_string(),
                namespace: "default".to_string(),
                labels: HashMap::new(),
                generation: 1,
                finalizers: vec![],
            },
            spec: AncoraTenantSpec {
                cluster_ref: "my-cluster".to_string(),
                max_workers: 5,
                provider_allowlist: vec!["openai".to_string()],
                residency_region: Some("uae-north".to_string()),
                admin_role_binding: Some("ancora-admin".to_string()),
            },
            status: AncoraTenantStatus::default(),
        }
    }

    #[test]
    fn tenant_cr_creates_isolated_namespace_config() {
        let mut r = Reconciler::new();
        let mut tenant = make_tenant("acme");
        r.reconcile_tenant(&mut tenant).unwrap();
        assert!(r.k8s().exists("Namespace", "ancora-acme"));
        assert!(r.k8s().exists("RoleBinding", "acme-admin"));
    }

    #[test]
    fn tenant_status_set_after_reconcile() {
        let mut r = Reconciler::new();
        let mut tenant = make_tenant("acme");
        r.reconcile_tenant(&mut tenant).unwrap();
        assert!(tenant.status.provisioned);
        assert_eq!(tenant.status.namespace.as_deref(), Some("ancora-acme"));
    }

    #[test]
    fn tenant_finalizer_added() {
        let mut r = Reconciler::new();
        let mut tenant = make_tenant("acme");
        r.reconcile_tenant(&mut tenant).unwrap();
        assert!(tenant.metadata.finalizers.iter().any(|f| f == "ancora.io/cleanup"));
    }
}
