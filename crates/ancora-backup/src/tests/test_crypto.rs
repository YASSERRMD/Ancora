#[cfg(test)]
mod tests {
    use crate::{BackupEngine, Journal, JournalEntry};

    fn entry() -> JournalEntry {
        JournalEntry { seq: 1, run_id: "r1".into(), tenant_id: "t".into(), kind: "step".into(), payload: "secret".into() }
    }

    #[test]
    fn encrypted_backup_round_trips() {
        let key = b"test-key-32-bytes-padded-xxxxxyyy".to_vec();
        let eng = BackupEngine::new(key);
        let mut src = Journal::new();
        src.append(entry());

        let archive = eng.snapshot(&src, vec![], vec![], 0).unwrap();
        assert!(archive.manifest.encrypted);

        let mut dst = Journal::new();
        eng.restore_snapshot(&archive, &mut dst).unwrap();
        assert_eq!(dst.entries(), src.entries());
    }

    #[test]
    fn encrypted_payload_differs_from_plaintext() {
        let key = b"mykey".to_vec();
        let mut j = Journal::new();
        j.append(entry());

        let enc_eng = BackupEngine::new(key);
        let plain_eng = BackupEngine::plaintext();

        let enc = enc_eng.snapshot(&j, vec![], vec![], 0).unwrap();
        let plain = plain_eng.snapshot(&j, vec![], vec![], 0).unwrap();

        assert_ne!(enc.payload, plain.payload);
    }

    #[test]
    fn wrong_key_fails_checksum() {
        let key1 = b"key1".to_vec();
        let key2 = b"key2".to_vec();

        let mut j = Journal::new();
        j.append(entry());

        let archive = BackupEngine::new(key1).snapshot(&j, vec![], vec![], 0).unwrap();
        let mut dst = Journal::new();
        let result = BackupEngine::new(key2).restore_snapshot(&archive, &mut dst);
        // Decrypted bytes won't match the stored checksum
        assert!(result.is_err());
    }
}
