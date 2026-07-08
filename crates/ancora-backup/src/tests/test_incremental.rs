#[cfg(test)]
mod tests {
    use crate::{BackupEngine, Journal, JournalEntry};

    fn journal_with_entries(n: u64) -> Journal {
        let mut j = Journal::new();
        for i in 1..=n {
            j.append(JournalEntry {
                seq: i,
                run_id: format!("r{}", i),
                tenant_id: "t".into(),
                kind: "step".into(),
                payload: "{}".into(),
            });
        }
        j
    }

    #[test]
    fn incremental_restore_reproduces_state() {
        let eng = BackupEngine::plaintext();
        let src = journal_with_entries(5);
        // Snapshot first 3
        let snap = eng
            .snapshot(&journal_with_entries(3), vec![], vec![], 0)
            .unwrap();
        // Incremental for entries 4..5
        let incr = eng.incremental(&src, 3, 0).unwrap();

        let mut dst = Journal::new();
        eng.restore_snapshot(&snap, &mut dst).unwrap();
        eng.restore_incremental(&incr, &mut dst).unwrap();

        assert_eq!(dst.entries().len(), 5);
        assert_eq!(dst.max_seq(), 5);
    }

    #[test]
    fn incremental_export_only_has_new_entries() {
        let eng = BackupEngine::plaintext();
        let src = journal_with_entries(5);
        let incr = eng.incremental(&src, 3, 0).unwrap();
        assert_eq!(incr.manifest.entry_count, 2); // entries 4 and 5
    }
}
