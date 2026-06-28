#[cfg(test)]
mod tests {
    use crate::poison::PoisonTracker;

    #[test]
    fn no_quarantine_before_threshold() {
        let mut t = PoisonTracker::new();
        for _ in 0..4 {
            assert!(!t.record_failure("run-1"));
        }
        assert!(!t.is_quarantined("run-1"));
    }

    #[test]
    fn quarantine_at_threshold() {
        let mut t = PoisonTracker::new();
        for _ in 0..4 {
            t.record_failure("run-1");
        }
        let poisoned = t.record_failure("run-1");
        assert!(poisoned);
        assert!(t.is_quarantined("run-1"));
    }

    #[test]
    fn reset_clears_quarantine() {
        let mut t = PoisonTracker::new();
        for _ in 0..5 {
            t.record_failure("run-1");
        }
        t.reset("run-1");
        assert!(!t.is_quarantined("run-1"));
    }

    #[test]
    fn quarantine_list_accurate() {
        let mut t = PoisonTracker::new();
        for _ in 0..5 {
            t.record_failure("run-a");
        }
        let list = t.quarantined_set();
        assert!(list.contains(&"run-a".to_string()));
    }
}
