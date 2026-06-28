#[cfg(test)]
mod tests {
    use ancora_controlplane::model::RunPriority;
    use ancora_controlplane::store::ControlPlaneStore;
    use crate::pool::WorkerPool;
    use crate::lifecycle::run_to_idle;

    fn make_pool(worker_count: usize) -> WorkerPool {
        let mut store = ControlPlaneStore::new();
        // queue a few runs
        store.create_run("t1", RunPriority::Normal);
        store.create_run("t1", RunPriority::High);
        WorkerPool::new(store, worker_count, 4)
    }

    #[test]
    fn pool_executes_runs_to_completion() {
        let mut pool = make_pool(2);
        let completed = run_to_idle(&mut pool, 20);
        assert_eq!(completed, 2);
    }

    #[test]
    fn two_workers_do_not_double_execute_same_run() {
        let store = ControlPlaneStore::new();
        let mut pool = WorkerPool::new(store, 3, 2);
        {
            let s = pool.store();
            let mut s = s.lock().unwrap();
            s.create_run("t1", RunPriority::Normal);
        }
        let results = pool.tick();
        let claimed: usize = results.iter().filter(|r| matches!(r, Ok(Some(_)))).count();
        assert!(claimed <= 1, "at most one worker should execute a single run");
    }

    #[test]
    fn lease_expiry_requeues_run() {
        use chrono::{Duration, Utc};
        // Directly use the store to verify lease expiry requeues a run
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);
        let worker = store.register_worker(1);
        // Claim without releasing (simulates a crash)
        let _claimed = store.claim_run(&worker.id).unwrap().unwrap();
        // Force lease expiry
        store.workers.get_mut(&worker.id).unwrap().lease_expires_at =
            Some(Utc::now() - Duration::seconds(1));
        store.expire_leases();
        assert_eq!(store.queue_depth(), 1, "expired lease should requeue the run");
        drop(run);
    }
}
