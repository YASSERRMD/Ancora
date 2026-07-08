#[cfg(test)]
mod tests {
    use crate::run_store::{RunEntry, RunStatus, RunStore};

    fn entry(id: &str, status: RunStatus) -> RunEntry {
        RunEntry {
            run_id: id.into(),
            tenant_id: "t1".into(),
            status,
            worker_id: None,
            created_at_secs: 0,
        }
    }

    #[test]
    fn list_runs_output() {
        let mut s = RunStore::default();
        s.insert(entry("run-1", RunStatus::Running));
        s.insert(entry("run-2", RunStatus::Pending));
        let list = s.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn cancel_run_works() {
        let mut s = RunStore::default();
        s.insert(entry("run-1", RunStatus::Running));
        assert!(s.cancel("run-1"));
        assert_eq!(s.get("run-1").unwrap().status, RunStatus::Cancelled);
    }

    #[test]
    fn cancel_completed_run_fails() {
        let mut s = RunStore::default();
        s.insert(entry("run-1", RunStatus::Completed));
        assert!(!s.cancel("run-1"));
    }

    #[test]
    fn resume_cancelled_run() {
        let mut s = RunStore::default();
        s.insert(entry("run-1", RunStatus::Cancelled));
        assert!(s.resume("run-1"));
        assert_eq!(s.get("run-1").unwrap().status, RunStatus::Pending);
    }
}
