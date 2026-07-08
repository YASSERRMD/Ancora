use crate::archive::{BackupArchive, BackupPayload};
use crate::crypto::{xor_decrypt, xor_encrypt};
use crate::error::BackupError;
use crate::journal::{Journal, JournalEntry};
use crate::manifest::BackupManifest;

pub struct BackupEngine {
    /// Optional encryption key. Empty = no encryption.
    key: Vec<u8>,
}

impl BackupEngine {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    pub fn plaintext() -> Self {
        Self::new(vec![])
    }

    fn serialize_payload(&self, payload: &BackupPayload) -> Result<Vec<u8>, BackupError> {
        serde_json::to_vec(payload).map_err(|e| BackupError::Serialization(e.to_string()))
    }

    fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        if self.key.is_empty() {
            data
        } else {
            xor_encrypt(&data, &self.key)
        }
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        if self.key.is_empty() {
            data.to_vec()
        } else {
            xor_decrypt(data, &self.key)
        }
    }

    /// Full snapshot backup.
    pub fn snapshot(
        &self,
        journal: &Journal,
        memory: Vec<(String, String)>,
        config: Vec<(String, String)>,
        now: u64,
    ) -> Result<BackupArchive, BackupError> {
        let payload = BackupPayload {
            journal: journal.snapshot(),
            memory,
            config,
        };
        self.build_archive(payload, journal.max_seq(), now)
    }

    /// Incremental backup (entries since `since_seq`).
    pub fn incremental(
        &self,
        journal: &Journal,
        since_seq: u64,
        now: u64,
    ) -> Result<BackupArchive, BackupError> {
        let entries = journal.incremental(since_seq);
        let payload = BackupPayload {
            journal: entries,
            memory: vec![],
            config: vec![],
        };
        self.build_archive(payload, journal.max_seq(), now)
    }

    fn build_archive(
        &self,
        payload: BackupPayload,
        max_seq: u64,
        now: u64,
    ) -> Result<BackupArchive, BackupError> {
        let raw = self.serialize_payload(&payload)?;
        let encrypted = self.encrypt(raw);
        let manifest = BackupManifest::new(
            &encrypted,
            payload.journal.len(),
            max_seq,
            !self.key.is_empty(),
            now,
        );
        Ok(BackupArchive {
            manifest,
            payload: encrypted,
        })
    }

    /// Verify then restore a snapshot backup into `journal`.
    pub fn restore_snapshot(
        &self,
        archive: &BackupArchive,
        journal: &mut Journal,
    ) -> Result<(), BackupError> {
        self.verify_and_decode(archive)
            .map(|p| journal.restore(p.journal))
    }

    /// Restore incremental backup by appending entries.
    pub fn restore_incremental(
        &self,
        archive: &BackupArchive,
        journal: &mut Journal,
    ) -> Result<(), BackupError> {
        let payload = self.verify_and_decode(archive)?;
        for entry in payload.journal {
            journal.append(entry);
        }
        Ok(())
    }

    /// Point-in-time restore: restore only entries with seq <= `up_to_seq`.
    pub fn restore_pit(
        &self,
        archive: &BackupArchive,
        journal: &mut Journal,
        up_to_seq: u64,
    ) -> Result<(), BackupError> {
        let payload = self.verify_and_decode(archive)?;
        let entries: Vec<JournalEntry> = payload
            .journal
            .into_iter()
            .filter(|e| e.seq <= up_to_seq)
            .collect();
        journal.restore(entries);
        Ok(())
    }

    fn verify_and_decode(&self, archive: &BackupArchive) -> Result<BackupPayload, BackupError> {
        if !archive.manifest.verify(&archive.payload) {
            return Err(BackupError::ChecksumMismatch);
        }
        let decrypted = self.decrypt(&archive.payload);
        serde_json::from_slice(&decrypted).map_err(|e| BackupError::Deserialization(e.to_string()))
    }
}
