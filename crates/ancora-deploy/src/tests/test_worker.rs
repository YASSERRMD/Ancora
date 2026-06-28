#[cfg(test)]
mod tests {
    use crate::{Version, VersionedWorker};

    #[test]
    fn version_tagged_worker_reports_idle() {
        let w = VersionedWorker::new("w1", Version::new(1, 0, 0));
        assert!(w.is_idle());
    }

    #[test]
    fn worker_with_active_runs_is_not_idle() {
        let mut w = VersionedWorker::new("w1", Version::new(1, 0, 0));
        w.active_runs = 2;
        assert!(!w.is_idle());
    }

    #[test]
    fn version_display_format_correct() {
        let v = Version::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }
}
