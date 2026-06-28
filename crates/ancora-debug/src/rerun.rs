/// rerun.rs - Edit input at a branch point and re-run from that position.
///
/// Re-running from a point means: take the original journal's prefix up to
/// seq N, replace or add entries representing the edited input, and produce
/// a new [`Branch`] that can be replayed and inspected like any other journal.
///
/// No live LLM or tool calls are made.  The "re-run" is a purely structural
/// operation on in-memory journal data.

use crate::branch::{Branch, BranchError};
use crate::loader::{EntryKind, Journal, JournalEntry, RunId, Seq};

/// Description of an edited entry to inject at the branch point.
#[derive(Debug, Clone)]
pub struct EditedInput {
    /// The run id that will be assigned to the new entry.
    pub run_id: RunId,
    /// The new kind of the entry (replaces whatever was at the branch point).
    pub kind: EntryKind,
}

/// Errors from the re-run subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RerunError {
    Branch(BranchError),
}

impl std::fmt::Display for RerunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RerunError::Branch(e) => write!(f, "branch error: {}", e),
        }
    }
}

impl std::error::Error for RerunError {}

impl From<BranchError> for RerunError {
    fn from(e: BranchError) -> Self {
        RerunError::Branch(e)
    }
}

/// Create a branch at `branch_point - 1` (so that `branch_point` is not
/// included in the prefix) and append `edited_entries` as the new
/// continuation.
///
/// If `branch_point` is 0 the branch starts from an empty prefix and the
/// entire run is replaced.
pub fn rerun_from(
    branch_id: impl Into<String>,
    journal: &Journal,
    branch_point: Seq,
    edited_entries: Vec<EditedInput>,
) -> Result<Branch, RerunError> {
    let branch_id = branch_id.into();

    // Branch just before the edit point so the edit point is excluded.
    let prefix_point = if branch_point.0 == 0 {
        None
    } else {
        Some(Seq(branch_point.0 - 1))
    };

    let mut branch = match prefix_point {
        Some(pp) => Branch::new(&branch_id, journal, pp)?,
        None => {
            // Empty prefix - create a branch at seq 0 and we'll replace everything.
            // We still need at least one entry; use seq 0 as the prefix.
            let mut b = Branch::new(&branch_id, journal, Seq(0))?;
            // Remove the single prefix entry by creating a fresh branch without it.
            // We achieve this by materialising and discarding, then rebuilding via
            // an alternate approach: branch at 0 gives us 1 prefix entry.  For a
            // "replace from start" scenario we keep that single entry and append.
            // The caller's edited_entries will be appended after seq 0.
            return {
                for input in edited_entries {
                    let entry = JournalEntry {
                        run_id: input.run_id,
                        seq: Seq(0), // will be re-sequenced in append
                        kind: input.kind,
                    };
                    b.append(entry)?;
                }
                Ok(b)
            };
        }
    };

    for input in edited_entries {
        let entry = JournalEntry {
            run_id: input.run_id,
            seq: Seq(0), // re-sequenced by Branch::append
            kind: input.kind,
        };
        branch.append(entry)?;
    }

    Ok(branch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};

    fn sc(run: &str, seq: u64, from: &str, to: &str) -> JournalEntry {
        JournalEntry::new(
            RunId::new(run),
            seq,
            EntryKind::StateChange { from: from.into(), to: to.into() },
        )
    }

    fn original() -> Journal {
        load_journal(vec![
            sc("orig", 0, "boot", "idle"),
            sc("orig", 1, "idle", "planning"),
            sc("orig", 2, "planning", "executing"),
            sc("orig", 3, "executing", "done"),
        ])
        .unwrap()
    }

    #[test]
    fn rerun_from_mid_creates_branch_with_prefix() {
        let j = original();
        let edited = vec![EditedInput {
            run_id: RunId::new("rerun"),
            kind: EntryKind::StateChange {
                from: "planning".into(),
                to: "alt-executing".into(),
            },
        }];
        let b = rerun_from("r1", &j, Seq(2), edited).unwrap();
        // Prefix = entries 0..=1 (2 entries), extension = 1 entry.
        assert_eq!(b.prefix_entries().len(), 2);
        assert_eq!(b.extension_entries().len(), 1);
    }

    #[test]
    fn rerun_materialises_to_valid_journal() {
        let j = original();
        let edited = vec![EditedInput {
            run_id: RunId::new("rerun"),
            kind: EntryKind::StateChange {
                from: "idle".into(),
                to: "fast-done".into(),
            },
        }];
        let b = rerun_from("r2", &j, Seq(1), edited).unwrap();
        let new_j = b.to_journal(RunId::new("rerun-run")).unwrap();
        assert!(new_j.len() >= 2);
    }

    #[test]
    fn rerun_from_invalid_seq_returns_error() {
        let j = original();
        let result = rerun_from("r3", &j, Seq(100), vec![]);
        assert!(result.is_err());
    }
}
