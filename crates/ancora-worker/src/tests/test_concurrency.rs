#[cfg(test)]
mod tests {
    use crate::concurrency::{total_capacity, worker_has_capacity};
    use ancora_controlplane::model::RunPriority;
    use ancora_controlplane::store::ControlPlaneStore;

    #[test]
    fn worker_has_capacity_when_under_limit() {
        let mut store = ControlPlaneStore::new();
        let w = store.register_worker(3);
        assert!(worker_has_capacity(&store, &w.id));
    }

    #[test]
    fn worker_at_limit_has_no_capacity() {
        let mut store = ControlPlaneStore::new();
        let w = store.register_worker(1);
        store.create_run("t1", RunPriority::Normal);
        store.claim_run(&w.id).unwrap();
        assert!(!worker_has_capacity(&store, &w.id));
    }

    #[test]
    fn total_capacity_sums_across_workers() {
        let mut store = ControlPlaneStore::new();
        store.register_worker(3);
        store.register_worker(2);
        assert_eq!(total_capacity(&store), 5);
    }

    #[test]
    fn crashed_worker_run_is_requeued() {
        use chrono::{Duration, Utc};
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(1);
        store.claim_run(&w.id).unwrap();
        // Simulate crash: expire lease without releasing
        store.workers.get_mut(&w.id).unwrap().lease_expires_at =
            Some(Utc::now() - Duration::seconds(1));
        store.expire_leases();
        assert_eq!(store.queue_depth(), 1);
        drop(run);
    }

    #[test]
    fn lease_renewal_prevents_premature_requeue() {
        use chrono::Utc;
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(1);
        store.claim_run(&w.id).unwrap();
        // Heartbeat renews lease
        store.heartbeat_worker(&w.id).unwrap();
        // Lease should still be valid
        let w_ref = store.workers.get(&w.id).unwrap();
        assert!(w_ref.lease_expires_at.unwrap() > Utc::now());
        drop(run);
    }
}
