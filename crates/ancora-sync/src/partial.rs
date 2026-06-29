//! Partial sync and resume support.
//!
//! Large batches of journal entries may not fit in a single request/response
//! round-trip, or connectivity may drop mid-sync.  [`PartialSyncSession`]
//! tracks progress so the sync can resume from where it left off.

use crate::model::{JournalEntry, SeqNo};
use crate::protocol::{Hub, SyncRequest, SyncResponse};

/// State carried across multiple partial sync round-trips.
pub struct PartialSyncSession {
    device_id: String,
    /// Entries yet to be uploaded.
    pending: Vec<JournalEntry>,
    /// Entries successfully acked by the hub.
    acked: Vec<SeqNo>,
    /// Maximum number of entries per request.
    batch_size: usize,
    /// Resume token from the hub (opaque string).
    resume_token: Option<String>,
}

impl PartialSyncSession {
    /// Create a new session with the given batch size.
    pub fn new(device_id: impl Into<String>, entries: Vec<JournalEntry>, batch_size: usize) -> Self {
        Self {
            device_id: device_id.into(),
            pending: entries,
            acked: Vec::new(),
            batch_size: batch_size.max(1),
            resume_token: None,
        }
    }

    /// Return `true` if all entries have been uploaded.
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty()
    }

    /// Return the sequence numbers acked so far.
    pub fn acked_seqs(&self) -> &[SeqNo] {
        &self.acked
    }

    /// Advance the session by one round-trip: build a request, send it to the
    /// hub, and process the response.
    pub fn step(&mut self, hub: &mut Hub) -> SyncResponse {
        let batch: Vec<JournalEntry> = self
            .pending
            .drain(..self.batch_size.min(self.pending.len()))
            .collect();

        let request = SyncRequest {
            device_id: self.device_id.clone(),
            entries: batch,
            resume_token: self.resume_token.clone(),
        };

        let response = hub.process(request);
        self.acked.extend_from_slice(&response.acked_seqs);
        self.resume_token = response.resume_token.clone();
        response
    }

    /// Run all steps until the session is complete.
    pub fn run_to_completion(&mut self, hub: &mut Hub) -> Vec<SeqNo> {
        while !self.is_complete() {
            self.step(hub);
        }
        self.acked.clone()
    }

    /// Remaining count of entries not yet uploaded.
    pub fn remaining(&self) -> usize {
        self.pending.len()
    }
}
