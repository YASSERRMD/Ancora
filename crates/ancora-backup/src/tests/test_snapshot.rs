#[cfg(test)]
mod tests {
    use crate::{BackupEngine, Journal, JournalEntry};

    fn sample_journal() -> Journal {
        let mut j = Journal::new();
        j.append(JournalEntry { seq: 1, run_id: "r1".into(), tenant_id: "t".into(), kind: "start".into(), payload: "{}".into() });
        j.append(JournalEntry { seq: 2, run_id: "r1".into(), tenant_id: "t".into(), kind: "step".into(), payload: "{\"n\":1}".into() });
        j.append(JournalEntry { seq: 3, run_id: "r1".into(), tenant_id: "t".into(), kind: "complete".into(), payload: "{}".into() });
        j
    }

    #[test]
    fn snapshot_then_restore_reproduces_state() {
        let eng = BackupEngine::plaintext();
        let src = sample_journal();
        let archive = eng.snapshot(&src, vec![], vec![], 1000).unwrap();

        let mut dst = Journal::new();
        eng.restore_snapshot(&archive, &mut dst).unwrap();

        assert_eq!(dst.entries(), src.entries());
    }

    #[test]
    fn snapshot_manifest_entry_count_matches() {
        let eng = BackupEngine::plaintext();
        let src = sample_journal();
        let archive = eng.snapshot(&src, vec![], vec![], 0).unwrap();
        assert_eq!(archive.manifest.entry_count, 3);
        assert_eq!(archive.manifest.max_seq, 3);
    }

    #[test]
    fn restored_run_replays_identically() {
        let eng = BackupEngine::plaintext();
        let src = sample_journal();
        let archive = eng.snapshot(&src, vec![], vec![], 0).unwrap();

        let mut dst = Journal::new();
        eng.restore_snapshot(&archive, &mut dst).unwrap();

        for (a, b) in src.entries().iter().zip(dst.entries()) {
            assert_eq!(a, b);
        }
    }
}
