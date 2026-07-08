#[cfg(test)]
mod tests {
    use crate::release::{release_failure, release_success};
    use ancora_controlplane::model::{RunPriority, RunState};
    use ancora_controlplane::store::ControlPlaneStore;

    #[test]
    fn release_success_marks_completed() {
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(1);
        let claimed = store.claim_run(&w.id).unwrap().unwrap();
        release_success(&mut store, &w.id, &claimed.id);
        assert_eq!(store.get_run(&run.id).unwrap().state, RunState::Completed);
    }

    #[test]
    fn release_failure_requeues_run() {
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(1);
        let claimed = store.claim_run(&w.id).unwrap().unwrap();
        release_failure(&mut store, &w.id, &claimed.id);
        assert_eq!(store.get_run(&run.id).unwrap().state, RunState::Queued);
    }

    #[test]
    fn test_lease_renewal_prevents_premature_requeue() {
        use chrono::Utc;
        let mut store = ControlPlaneStore::new();
        store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(1);
        store.claim_run(&w.id).unwrap().unwrap();
        store.heartbeat_worker(&w.id).unwrap();
        let worker = store.workers.get(&w.id).unwrap();
        assert!(worker.lease_expires_at.unwrap() > Utc::now());
    }
}
