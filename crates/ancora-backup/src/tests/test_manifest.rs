#[cfg(test)]
mod tests {
    use crate::manifest::sha256_hex;
    use crate::{BackupEngine, Journal, JournalEntry};

    #[test]
    fn checksum_mismatch_is_rejected() {
        let eng = BackupEngine::plaintext();
        let mut j = Journal::new();
        j.append(JournalEntry {
            seq: 1,
            run_id: "r".into(),
            tenant_id: "t".into(),
            kind: "x".into(),
            payload: "{}".into(),
        });
        let mut archive = eng.snapshot(&j, vec![], vec![], 0).unwrap();

        // Tamper with payload
        archive.payload[0] ^= 0xFF;

        let mut dst = Journal::new();
        let result = eng.restore_snapshot(&archive, &mut dst);
        assert!(matches!(result, Err(crate::BackupError::ChecksumMismatch)));
    }

    #[test]
    fn manifest_verify_returns_true_for_valid_data() {
        let data = b"test data for checksum";
        let manifest = crate::manifest::BackupManifest::new(data, 0, 0, false, 0);
        assert!(manifest.verify(data));
    }

    #[test]
    fn sha256_hex_is_deterministic() {
        let h1 = sha256_hex(b"hello");
        let h2 = sha256_hex(b"hello");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }
}
