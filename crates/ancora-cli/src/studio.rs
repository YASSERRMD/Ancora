use std::sync::Arc;

use ancora_core::journal::MemoryStore;

/// Timeline for a single run, returned by GET /runs/:id/timeline.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RunTimeline {
    pub run_id: String,
    pub events: Vec<String>,
}
