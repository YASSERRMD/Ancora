#[cfg(test)]
mod tests {
    use crate::{replicate, FailoverController, JournalEntry, JournalStore};

    #[test]
    fn fencing_prevents_double_primary() {
        let mut p = JournalStore::new();
        p.append(JournalEntry {
            seq: 1,
            data: "d".into(),
        })
        .unwrap();
        let mut s = JournalStore::new();
        replicate(&p, &mut s);

        let mut ctrl = FailoverController::new();
        ctrl.failover(&mut p, &mut s, 0).unwrap();

        // Old primary is fenced: writes must fail
        let err = p.append(JournalEntry {
            seq: 2,
            data: "split".into(),
        });
        assert!(err.is_err(), "fenced primary must reject writes");
    }

    #[test]
    fn secondary_unfenced_on_promotion() {
        let mut p = JournalStore::new();
        p.append(JournalEntry {
            seq: 1,
            data: "d".into(),
        })
        .unwrap();
        let mut s = JournalStore::new();
        replicate(&p, &mut s);

        let mut ctrl = FailoverController::new();
        ctrl.failover(&mut p, &mut s, 0).unwrap();

        // New primary (old secondary) must accept writes
        assert!(s
            .append(JournalEntry {
                seq: 2,
                data: "new".into()
            })
            .is_ok());
    }
}
