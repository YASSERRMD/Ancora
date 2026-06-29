/// Replay support: reconstruct the model-version sequence from a journal.

use crate::journal::{SwapEvent, SwapJournal};
use crate::model::ModelVersion;

/// A step in a replayed swap sequence.
#[derive(Debug, Clone)]
pub struct ReplayStep {
    pub step: usize,
    pub from: ModelVersion,
    pub to: ModelVersion,
    pub is_rollback: bool,
}

/// Replay a journal into a sequence of `ReplayStep`s for audit or testing.
pub fn replay(journal: &SwapJournal) -> Vec<ReplayStep> {
    journal
        .entries()
        .iter()
        .enumerate()
        .map(|(i, entry)| match &entry.event {
            SwapEvent::Swap { from, to } => ReplayStep {
                step: i,
                from: *from,
                to: *to,
                is_rollback: false,
            },
            SwapEvent::Rollback { from, to } => ReplayStep {
                step: i,
                from: *from,
                to: *to,
                is_rollback: true,
            },
        })
        .collect()
}

/// Verify that the final active model version after replay matches `expected`.
pub fn verify_final_version(journal: &SwapJournal, expected: ModelVersion) -> bool {
    let steps = replay(journal);
    if let Some(last) = steps.last() {
        last.to == expected
    } else {
        false
    }
}
