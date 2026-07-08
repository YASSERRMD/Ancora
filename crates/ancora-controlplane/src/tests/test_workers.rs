#[cfg(test)]
mod tests {
    use crate::model::RunPriority;
    use crate::store::ControlPlaneStore;
    use chrono::{Duration, Utc};

    #[test]
    fn worker_heartbeat_and_lease_expiry() {
        let mut store = ControlPlaneStore::new();
        let worker = store.register_worker(5);
        store.heartbeat_worker(&worker.id).unwrap();
        let run = store.create_run("t1", RunPriority::Normal);
        let claimed = store.claim_run(&worker.id).unwrap().unwrap();
        assert_eq!(claimed.id, run.id);

        // Simulate lease expiry by manually expiring
        store.workers.get_mut(&worker.id).unwrap().lease_expires_at =
            Some(Utc::now() - Duration::seconds(1));
        store.expire_leases();

        let run_state = store.get_run(&run.id).unwrap().state.clone();
        assert_eq!(run_state, crate::model::RunState::Queued);
    }

    #[test]
    fn run_assignment_is_exclusive() {
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w1 = store.register_worker(5);
        let w2 = store.register_worker(5);

        let c1 = store.claim_run(&w1.id).unwrap();
        let c2 = store.claim_run(&w2.id).unwrap();

        assert!(c1.is_some());
        assert!(c2.is_none(), "Second worker should not claim the same run");
        drop(run);
    }

    #[test]
    fn graceful_release_completes_run() {
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(5);
        let claimed = store.claim_run(&w.id).unwrap().unwrap();
        store.release_lease(&w.id, &claimed.id, true);
        assert_eq!(
            store.get_run(&run.id).unwrap().state,
            crate::model::RunState::Completed
        );
    }

    #[test]
    fn failed_release_requeues_run() {
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let w = store.register_worker(5);
        let claimed = store.claim_run(&w.id).unwrap().unwrap();
        store.release_lease(&w.id, &claimed.id, false);
        assert_eq!(
            store.get_run(&run.id).unwrap().state,
            crate::model::RunState::Queued
        );
    }
}

#[cfg(test)]
mod assignment_tests {
    use crate::model::RunPriority;
    use crate::store::ControlPlaneStore;

    #[test]
    fn two_workers_do_not_double_claim_a_run() {
        let mut store = ControlPlaneStore::new();
        store.create_run("t1", RunPriority::Normal);
        let w1 = store.register_worker(5);
        let w2 = store.register_worker(5);
        let c1 = store.claim_run(&w1.id).unwrap();
        let c2 = store.claim_run(&w2.id).unwrap();
        let total_claimed = [c1.is_some(), c2.is_some()].iter().filter(|&&x| x).count();
        assert_eq!(total_claimed, 1, "exactly one worker should claim the run");
    }
}
