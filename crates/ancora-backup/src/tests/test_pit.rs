#[cfg(test)]
mod tests {
    use crate::{BackupEngine, Journal, JournalEntry};

    fn journal_with_entries(n: u64) -> Journal {
        let mut j = Journal::new();
        for i in 1..=n {
            j.append(JournalEntry {
                seq: i,
                run_id: "r".into(),
                tenant_id: "t".into(),
                kind: "step".into(),
                payload: "{}".into(),
            });
        }
        j
    }

    #[test]
    fn point_in_time_restore_stops_at_seq() {
        let eng = BackupEngine::plaintext();
        let src = journal_with_entries(10);
        let archive = eng.snapshot(&src, vec![], vec![], 0).unwrap();

        let mut dst = Journal::new();
        eng.restore_pit(&archive, &mut dst, 5).unwrap();

        assert_eq!(dst.entries().len(), 5);
        assert_eq!(dst.max_seq(), 5);
    }

    #[test]
    fn pit_restore_includes_boundary_seq() {
        let eng = BackupEngine::plaintext();
        let src = journal_with_entries(3);
        let archive = eng.snapshot(&src, vec![], vec![], 0).unwrap();
        let mut dst = Journal::new();
        eng.restore_pit(&archive, &mut dst, 3).unwrap();
        assert_eq!(dst.entries().len(), 3);
    }
}
