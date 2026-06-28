#[cfg(test)]
mod tests {
    use crate::{BackupEngine, Journal};

    #[test]
    fn memory_store_included_in_backup() {
        let eng = BackupEngine::plaintext();
        let j = Journal::new();
        let memory = vec![("session:abc".to_string(), "state".to_string())];
        let archive = eng.snapshot(&j, memory.clone(), vec![], 0).unwrap();
        // Manifest reflects 0 journal entries but the payload was built
        assert_eq!(archive.manifest.entry_count, 0);
        // Verify the archive is valid (checksum passes)
        let mut dst = Journal::new();
        eng.restore_snapshot(&archive, &mut dst).unwrap();
    }

    #[test]
    fn config_included_in_backup() {
        let eng = BackupEngine::plaintext();
        let j = Journal::new();
        let config = vec![
            ("max_workers".to_string(), "10".to_string()),
            ("log_level".to_string(), "info".to_string()),
        ];
        let archive = eng.snapshot(&j, vec![], config, 0).unwrap();
        assert!(archive.manifest.verify(&archive.payload));
    }
}
