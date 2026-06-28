#[cfg(test)]
mod tests {
    use crate::RunTracker;

    #[test]
    fn zero_duplicate_side_effects_tracked() {
        let mut t = RunTracker::new();
        t.start("run-a");
        t.complete("run-a");
        // run-a should not appear in resume list
        assert!(t.runs_to_resume().is_empty());
        assert!(t.is_completed("run-a"));
    }

    #[test]
    fn in_flight_runs_resume_after_failover() {
        let mut t = RunTracker::new();
        t.start("run-a");
        t.start("run-b");
        let to_resume = t.runs_to_resume();
        assert_eq!(to_resume.len(), 2);
    }
}
