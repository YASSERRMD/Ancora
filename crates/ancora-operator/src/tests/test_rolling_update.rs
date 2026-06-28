#[cfg(test)]
mod tests {
    use crate::tests::test_reconcile::tests::make_cluster;
    use crate::reconciler::Reconciler;

    #[test]
    fn update_propagates_to_deployments() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("upd-cluster");
        r.reconcile_cluster(&mut cluster).unwrap();

        r.rolling_update_cluster(&mut cluster, "ancora:v2".to_string());

        let cp = r.k8s().get("Deployment", "upd-cluster-control-plane").unwrap();
        assert_eq!(cp.get("image").and_then(|v| v.as_str()), Some("ancora:v2"));

        let worker = r.k8s().get("Deployment", "upd-cluster-worker").unwrap();
        assert_eq!(worker.get("image").and_then(|v| v.as_str()), Some("ancora:v2"));
    }

    #[test]
    fn status_reflects_health_after_update() {
        let mut r = Reconciler::new();
        let mut cluster = make_cluster("upd-cluster");
        r.rolling_update_cluster(&mut cluster, "ancora:v2".to_string());
        assert!(cluster.status.control_plane_ready);
        assert!(cluster.status.worker_ready);
    }
}
