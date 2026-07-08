#[cfg(test)]
mod tests {
    use crate::worker_ops::{WorkerRegistry, WorkerState, WorkerStatus};

    fn worker(id: &str, active: u32) -> WorkerStatus {
        WorkerStatus {
            worker_id: id.into(),
            state: WorkerState::Active,
            active_runs: active,
            last_heartbeat_secs: 0,
        }
    }

    #[test]
    fn drain_worker_works() {
        let mut reg = WorkerRegistry::default();
        reg.register(worker("w-1", 0));
        assert!(reg.drain("w-1"));
        let w = reg.list().iter().find(|w| w.worker_id == "w-1").unwrap();
        assert_eq!(w.state, WorkerState::Draining);
    }

    #[test]
    fn drained_worker_detected() {
        let mut reg = WorkerRegistry::default();
        reg.register(worker("w-1", 0));
        reg.drain("w-1");
        assert!(reg.is_drained("w-1"));
    }

    #[test]
    fn non_drained_worker_has_active_runs() {
        let mut reg = WorkerRegistry::default();
        reg.register(worker("w-1", 5));
        reg.drain("w-1");
        assert!(!reg.is_drained("w-1"));
    }
}
