/// branch.rs - Branch from a seq to explore alternative run paths.
///
/// A branch is a new journal that shares a prefix with an existing journal
/// up to and including a given sequence number, then diverges with new
/// entries.  This enables "what if" analysis: keep the same initial
/// conditions but simulate a different continuation.
use crate::loader::{Journal, JournalEntry, RunId, Seq};

/// Errors that can occur when creating or manipulating branches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchError {
    /// The branch point sequence number is out of range for the source journal.
    SeqOutOfRange { seq: u64, journal_len: usize },
    /// Attempted to merge two branches that share no common prefix.
    NoPrefixShared,
}

impl std::fmt::Display for BranchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BranchError::SeqOutOfRange { seq, journal_len } => {
                write!(
                    f,
                    "seq {} is out of range for journal of length {}",
                    seq, journal_len
                )
            }
            BranchError::NoPrefixShared => {
                write!(f, "branches share no common prefix")
            }
        }
    }
}

impl std::error::Error for BranchError {}

/// A branch derived from a parent journal.
#[derive(Debug, Clone)]
pub struct Branch {
    /// Identifier for this branch.
    pub branch_id: String,
    /// The run id of the parent journal.
    pub parent_run_id: RunId,
    /// The sequence number at which this branch was created.
    pub branch_point: Seq,
    /// The prefix entries copied verbatim from the parent.
    prefix: Vec<JournalEntry>,
    /// The new entries added after the branch point.
    extension: Vec<JournalEntry>,
}

impl Branch {
    /// Create a new branch from `journal` at `branch_point`.
    ///
    /// The branch inherits all entries up to and including `branch_point`.
    pub fn new(
        branch_id: impl Into<String>,
        journal: &Journal,
        branch_point: Seq,
    ) -> Result<Self, BranchError> {
        if branch_point.0 as usize >= journal.len() {
            return Err(BranchError::SeqOutOfRange {
                seq: branch_point.0,
                journal_len: journal.len(),
            });
        }
        let prefix = journal.entries()[..=branch_point.0 as usize].to_vec();
        Ok(Branch {
            branch_id: branch_id.into(),
            parent_run_id: journal.run_id.clone(),
            branch_point,
            prefix,
            extension: Vec::new(),
        })
    }

    /// Append a new entry to the branch extension.
    ///
    /// The sequence number of the new entry must be exactly one more than
    /// the last entry in the branch.
    pub fn append(&mut self, entry: JournalEntry) -> Result<(), BranchError> {
        let expected_seq = self.len() as u64;
        // Silently re-sequence the entry so callers don't have to track it.
        let mut entry = entry;
        entry.seq = crate::loader::Seq(expected_seq);
        self.extension.push(entry);
        Ok(())
    }

    /// Total number of entries in the branch (prefix + extension).
    pub fn len(&self) -> usize {
        self.prefix.len() + self.extension.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return all entries in order (prefix then extension).
    pub fn entries(&self) -> impl Iterator<Item = &JournalEntry> {
        self.prefix.iter().chain(self.extension.iter())
    }

    /// Return only the extension entries.
    pub fn extension_entries(&self) -> &[JournalEntry] {
        &self.extension
    }

    /// Return only the prefix entries.
    pub fn prefix_entries(&self) -> &[JournalEntry] {
        &self.prefix
    }

    /// Materialise the branch as a new [`Journal`] for use with other
    /// debug tools.
    pub fn to_journal(&self, new_run_id: RunId) -> Result<Journal, crate::loader::LoadError> {
        let mut entries: Vec<JournalEntry> = self.entries().cloned().collect();
        // Re-assign run ids so the journal validator is satisfied.
        for (i, e) in entries.iter_mut().enumerate() {
            e.run_id = new_run_id.clone();
            e.seq = crate::loader::Seq(i as u64);
        }
        crate::loader::load_journal(entries)
    }
}

/// Registry of all branches in a debugging session.
#[derive(Default)]
pub struct BranchRegistry {
    branches: Vec<Branch>,
}

impl BranchRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, branch: Branch) {
        self.branches.push(branch);
    }

    pub fn get(&self, branch_id: &str) -> Option<&Branch> {
        self.branches.iter().find(|b| b.branch_id == branch_id)
    }

    pub fn list(&self) -> Vec<&str> {
        self.branches.iter().map(|b| b.branch_id.as_str()).collect()
    }

    pub fn count(&self) -> usize {
        self.branches.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId};

    fn sc(run: &str, seq: u64, from: &str, to: &str) -> JournalEntry {
        JournalEntry::new(
            RunId::new(run),
            seq,
            EntryKind::StateChange {
                from: from.into(),
                to: to.into(),
            },
        )
    }

    fn sample_journal() -> Journal {
        load_journal(vec![
            sc("r1", 0, "init", "planning"),
            sc("r1", 1, "planning", "executing"),
            sc("r1", 2, "executing", "done"),
        ])
        .unwrap()
    }

    #[test]
    fn branch_copies_prefix() {
        let j = sample_journal();
        let b = Branch::new("b1", &j, Seq(1)).unwrap();
        assert_eq!(b.prefix_entries().len(), 2); // entries 0 and 1
    }

    #[test]
    fn branch_out_of_range_errors() {
        let j = sample_journal();
        assert!(matches!(
            Branch::new("b1", &j, Seq(10)),
            Err(BranchError::SeqOutOfRange { .. })
        ));
    }

    #[test]
    fn branch_append_extends() {
        let j = sample_journal();
        let mut b = Branch::new("b1", &j, Seq(1)).unwrap();
        b.append(sc("r1", 999, "planning", "alternative")).unwrap();
        assert_eq!(b.extension_entries().len(), 1);
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn branch_to_journal_is_valid() {
        let j = sample_journal();
        let mut b = Branch::new("b1", &j, Seq(1)).unwrap();
        b.append(sc("r1", 999, "planning", "alt-done")).unwrap();
        let new_j = b.to_journal(RunId::new("branch-run")).unwrap();
        assert_eq!(new_j.len(), 3);
    }

    #[test]
    fn registry_stores_and_retrieves() {
        let j = sample_journal();
        let b = Branch::new("my-branch", &j, Seq(0)).unwrap();
        let mut reg = BranchRegistry::new();
        reg.insert(b);
        assert!(reg.get("my-branch").is_some());
        assert_eq!(reg.count(), 1);
    }
}
