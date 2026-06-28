#[cfg(test)]
mod tests {
    use crate::{FailoverController, JournalEntry, JournalStore, Role, replicate};

    fn do_failover() -> (JournalStore, JournalStore, FailoverController) {
        let mut p = JournalStore::new();
        p.append(JournalEntry { seq: 1, data: "d".into() }).unwrap();
        let mut s = JournalStore::new();
        replicate(&p, &mut s);
        let mut ctrl = FailoverController::new();
        ctrl.failover(&mut p, &mut s, 0).unwrap();
        (p, s, ctrl)
    }

    #[test]
    fn failback_restores_primary_cleanly() {
        let (mut old_p, mut new_p, mut ctrl) = do_failover();
        // Write to new primary after failover
        new_p.append(JournalEntry { seq: 2, data: "after".into() }).unwrap();

        ctrl.failback(&mut old_p, &mut new_p).unwrap();

        assert_eq!(ctrl.primary_role, Role::Primary);
        assert_eq!(ctrl.secondary_role, Role::Secondary);
        // Old primary synced with new data
        assert!(old_p.entries.iter().any(|e| e.seq == 2));
    }

    #[test]
    fn failback_fails_without_prior_failover() {
        let mut ctrl = FailoverController::new();
        let mut p = JournalStore::new();
        let mut s = JournalStore::new();
        assert!(ctrl.failback(&mut p, &mut s).is_err());
    }
}
