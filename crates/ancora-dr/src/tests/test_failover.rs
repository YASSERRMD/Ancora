#[cfg(test)]
mod tests {
    use crate::{replicate, FailoverController, JournalEntry, JournalStore, Role};

    fn synced_pair(n: u64) -> (JournalStore, JournalStore) {
        let mut p = JournalStore::new();
        for i in 1..=n {
            p.append(JournalEntry {
                seq: i,
                data: format!("d{}", i),
            })
            .unwrap();
        }
        let mut s = JournalStore::new();
        replicate(&p, &mut s);
        (p, s)
    }

    #[test]
    fn failover_promotes_secondary() {
        let (mut p, mut s) = synced_pair(3);
        let mut ctrl = FailoverController::new();
        ctrl.failover(&mut p, &mut s, 0).unwrap();
        assert_eq!(ctrl.secondary_role, Role::Primary);
        assert_eq!(ctrl.primary_role, Role::Standby);
    }

    #[test]
    fn failover_rejected_when_lag_too_high() {
        let mut p = JournalStore::new();
        p.append(JournalEntry {
            seq: 1,
            data: "d".into(),
        })
        .unwrap();
        p.append(JournalEntry {
            seq: 2,
            data: "d".into(),
        })
        .unwrap();
        let mut s = JournalStore::new(); // 2 entries behind
        let mut ctrl = FailoverController::new();
        let err = ctrl.failover(&mut p, &mut s, 0);
        assert!(err.is_err());
    }

    #[test]
    fn in_flight_runs_listed_for_resume_after_failover() {
        let mut tracker = crate::RunTracker::new();
        tracker.start("run-1");
        tracker.start("run-2");
        tracker.complete("run-1");
        let to_resume = tracker.runs_to_resume();
        assert_eq!(to_resume.len(), 1);
        assert!(to_resume.contains(&"run-2".to_string()));
    }
}
