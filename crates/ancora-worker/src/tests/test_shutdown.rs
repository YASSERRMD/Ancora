#[cfg(test)]
mod tests {
    use crate::lifecycle::run_with_shutdown;
    use crate::pool::WorkerPool;
    use crate::shutdown::ShutdownSignal;
    use ancora_controlplane::model::RunPriority;
    use ancora_controlplane::store::ControlPlaneStore;

    #[test]
    fn graceful_shutdown_drains_active_runs() {
        let mut store = ControlPlaneStore::new();
        store.create_run("t1", RunPriority::Normal);
        store.create_run("t1", RunPriority::Normal);
        let mut pool = WorkerPool::new(store, 2, 2);

        let signal = ShutdownSignal::new();

        // Run one tick to claim runs, then request shutdown
        pool.tick();
        signal.request();

        // Run with shutdown - drains remaining, completes in-flight
        run_with_shutdown(&mut pool, signal, 20);
        // Pool should be idle after drain
        assert!(pool.is_idle());
    }

    #[test]
    fn new_runs_not_claimed_after_drain() {
        let store = ControlPlaneStore::new();
        let mut pool = WorkerPool::new(store, 1, 1);
        pool.start_drain();

        // Queue a run after drain started
        {
            let s = pool.store();
            s.lock().unwrap().create_run("t1", RunPriority::Normal);
        }

        let results = pool.tick();
        let claimed = results.iter().filter(|r| matches!(r, Ok(Some(_)))).count();
        assert_eq!(claimed, 0, "draining pool should not claim new runs");
    }

    #[test]
    fn shutdown_signal_is_thread_safe() {
        let sig = ShutdownSignal::new();
        let sig2 = sig.clone();
        let handle = std::thread::spawn(move || {
            sig2.request();
        });
        handle.join().unwrap();
        assert!(sig.is_requested());
    }
}
