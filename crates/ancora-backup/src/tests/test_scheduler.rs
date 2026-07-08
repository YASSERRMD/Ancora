#[cfg(test)]
mod tests {
    use crate::{BackupEngine, BackupSchedule, Journal, JournalEntry};

    #[test]
    fn scheduled_job_produces_valid_backup() {
        let mut sched = BackupSchedule::new(3600);
        let eng = BackupEngine::plaintext();
        let mut j = Journal::new();
        j.append(JournalEntry {
            seq: 1,
            run_id: "r".into(),
            tenant_id: "t".into(),
            kind: "step".into(),
            payload: "{}".into(),
        });

        // Due once the interval has elapsed from the epoch start
        assert!(sched.is_due(3600), "due after one interval");
        let archive = eng.snapshot(&j, vec![], vec![], 3600).unwrap();
        sched.record_run(3600);

        assert!(!sched.is_due(5000), "not due before next interval");
        assert!(sched.is_due(7200), "due after second interval");

        // Verify the archive produced is valid
        let mut dst = Journal::new();
        eng.restore_snapshot(&archive, &mut dst).unwrap();
        assert_eq!(dst.entries().len(), 1);
    }

    #[test]
    fn schedule_interval_enforced() {
        let mut sched = BackupSchedule::new(60);
        sched.record_run(1000);
        assert!(!sched.is_due(1059));
        assert!(sched.is_due(1060));
    }
}
