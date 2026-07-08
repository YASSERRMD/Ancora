#[cfg(test)]
pub mod tests {
    use crate::crd::cluster::{
        AncoraCluster, AncoraClusterSpec, AncoraClusterStatus, JournalStoreConfig, ObjectMeta,
    };
    use crate::reconciler::Reconciler;
    use std::collections::HashMap;

    pub(crate) fn make_cluster(name: &str) -> AncoraCluster {
        AncoraCluster {
            metadata: ObjectMeta {
                name: name.to_string(),
                namespace: "default".to_string(),
                labels: HashMap::new(),
                generation: 1,
                finalizers: vec![],
            },
            spec: AncoraClusterSpec {
                control_plane_replicas: 2,
                worker_replicas: 4,
                worker_concurrency: 8,
                journal_store: JournalStoreConfig {
                    backend: "sqlite".to_string(),
                    connection_secret_ref: None,
                },
                autoscaler_enabled: true,
                min_workers: 2,
                max_workers: 10,
                image: "ancora:latest".to_string(),
                secret_refs: HashMap::new(),
            },
            status: AncoraClusterStatus::default(),
        }
    }

    #[test]
    fn reconcile_creates_expected_resources() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("test-cluster");
        r.reconcile_cluster(&mut cluster).unwrap();

        assert!(r.k8s().exists("Deployment", "test-cluster-control-plane"));
        assert!(r.k8s().exists("Deployment", "test-cluster-worker"));
        assert!(r
            .k8s()
            .exists("HorizontalPodAutoscaler", "test-cluster-worker-hpa"));
        assert!(r.k8s().exists("ConfigMap", "test-cluster-journal"));
    }

    #[test]
    fn reconcile_sets_status_ready() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("test-cluster");
        r.reconcile_cluster(&mut cluster).unwrap();
        assert!(cluster.status.control_plane_ready);
        assert!(cluster.status.worker_ready);
        assert!(!cluster.status.conditions.is_empty());
    }

    #[test]
    fn finalizer_added_on_reconcile() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("test-cluster");
        r.reconcile_cluster(&mut cluster).unwrap();
        assert!(cluster
            .metadata
            .finalizers
            .iter()
            .any(|f| f == "ancora.io/cleanup"));
    }

    #[test]
    fn status_reflects_observed_generation() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("test-cluster");
        cluster.metadata.generation = 3;
        r.reconcile_cluster(&mut cluster).unwrap();
        assert_eq!(cluster.status.observed_generation, 3);
    }
}
