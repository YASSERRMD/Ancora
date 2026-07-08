#[cfg(test)]
mod tests {
    use crate::crd::cluster::ObjectMeta;
    use crate::crd::tenant::{AncoraTenant, AncoraTenantSpec, AncoraTenantStatus};
    use crate::reconciler::Reconciler;
    use crate::tests::test_reconcile::tests::make_cluster;
    use std::collections::HashMap;

    #[test]
    fn delete_cleans_up_via_finalizer() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("del-cluster");
        r.reconcile_cluster(&mut cluster).unwrap();
        assert!(r.k8s().exists("Deployment", "del-cluster-control-plane"));

        r.delete_cluster(&mut cluster).unwrap();
        assert!(!r.k8s().exists("Deployment", "del-cluster-control-plane"));
        assert!(
            cluster.metadata.finalizers.is_empty(),
            "finalizer should be removed"
        );
    }

    #[test]
    fn tenant_delete_removes_namespace() {
        let mut r = Reconciler::new();
        let mut tenant = AncoraTenant {
            metadata: ObjectMeta {
                name: "del-tenant".to_string(),
                namespace: "default".to_string(),
                labels: HashMap::new(),
                generation: 1,
                finalizers: vec![],
            },
            spec: AncoraTenantSpec {
                cluster_ref: "c".to_string(),
                max_workers: 2,
                provider_allowlist: vec![],
                residency_region: None,
                admin_role_binding: None,
            },
            status: AncoraTenantStatus::default(),
        };
        r.reconcile_tenant(&mut tenant).unwrap();
        assert!(r.k8s().exists("Namespace", "ancora-del-tenant"));
        r.delete_tenant(&mut tenant).unwrap();
        assert!(!r.k8s().exists("Namespace", "ancora-del-tenant"));
    }
}
