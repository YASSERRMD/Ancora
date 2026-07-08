use crate::journal::JournalEntry;
use crate::manifest::BackupManifest;
use serde::{Deserialize, Serialize};

/// A complete backup archive (in memory / file bytes).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupArchive {
    pub manifest: BackupManifest,
    /// Raw payload bytes (may be encrypted).
    pub payload: Vec<u8>,
}

/// Payload structure stored inside the archive.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupPayload {
    pub journal: Vec<JournalEntry>,
    pub memory: Vec<(String, String)>,
    pub config: Vec<(String, String)>,
}
