/// api.rs - Debug API surface for the Ancora Studio.
///
/// Provides a high-level, session-scoped interface that bundles the loader,
/// replayer, inspector, diff engine, branch registry, and annotation store
/// behind a single type.  All operations are offline and require no live
/// LLM or tool calls.
use std::collections::HashMap;

use crate::annotate::{AnnotateError, Annotation, AnnotationStore};
use crate::branch::{Branch, BranchError, BranchRegistry};
use crate::diff::{diff_journals, RunDiff};
use crate::inspector::{Inspector, LlmSnapshot, StateSnapshot, ToolSnapshot};
use crate::loader::{load_journal, Journal, JournalEntry, LoadError, RunId, Seq};
use crate::replay::{Replayer, StepResult};

/// A debug session wraps one primary journal and manages replayers, branches,
/// and annotations.
pub struct DebugSession {
    primary: Journal,
    secondary: Option<Journal>,
    branch_registry: BranchRegistry,
    annotation_store: AnnotationStore,
}

/// Errors surfaced by the debug API.
#[derive(Debug)]
pub enum ApiError {
    Load(LoadError),
    Branch(BranchError),
    Annotate(AnnotateError),
    NoSecondaryJournal,
    BranchNotFound(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Load(e) => write!(f, "load error: {}", e),
            ApiError::Branch(e) => write!(f, "branch error: {}", e),
            ApiError::Annotate(e) => write!(f, "annotation error: {}", e),
            ApiError::NoSecondaryJournal => write!(f, "no secondary journal loaded for diff"),
            ApiError::BranchNotFound(id) => write!(f, "branch not found: {}", id),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<LoadError> for ApiError {
    fn from(e: LoadError) -> Self {
        ApiError::Load(e)
    }
}
impl From<BranchError> for ApiError {
    fn from(e: BranchError) -> Self {
        ApiError::Branch(e)
    }
}
impl From<AnnotateError> for ApiError {
    fn from(e: AnnotateError) -> Self {
        ApiError::Annotate(e)
    }
}

impl DebugSession {
    /// Create a new session from a primary journal.
    pub fn new(primary_entries: Vec<JournalEntry>) -> Result<Self, ApiError> {
        let primary = load_journal(primary_entries)?;
        Ok(Self {
            primary,
            secondary: None,
            branch_registry: BranchRegistry::new(),
            annotation_store: AnnotationStore::new(),
        })
    }

    /// Load a secondary journal to enable cross-run diffing.
    pub fn load_secondary(&mut self, entries: Vec<JournalEntry>) -> Result<(), ApiError> {
        self.secondary = Some(load_journal(entries)?);
        Ok(())
    }

    // ---- Inspection --------------------------------------------------------

    /// Return the agent state at the given sequence number.
    pub fn state_at(&self, seq: Seq) -> Option<StateSnapshot> {
        Inspector::new(&self.primary).state_at(seq)
    }

    /// Return the most recent LLM exchange at or before `seq`.
    pub fn llm_at(&self, seq: Seq) -> Option<LlmSnapshot> {
        Inspector::new(&self.primary).llm_at(seq)
    }

    /// Return the most recent tool call at or before `seq`.
    pub fn tool_at(&self, seq: Seq) -> Option<ToolSnapshot> {
        Inspector::new(&self.primary).tool_at(seq)
    }

    // ---- Replay ------------------------------------------------------------

    /// Run the primary journal from the beginning, calling `f` for each entry.
    pub fn replay_all<F>(&self, f: F)
    where
        F: FnMut(&JournalEntry),
    {
        let mut r = Replayer::new(&self.primary);
        r.run_to_end(f);
    }

    /// Replay up to and including `seq`, returning all visited entries.
    pub fn replay_to(&self, seq: Seq) -> Vec<&JournalEntry> {
        let mut r = Replayer::new(&self.primary);
        while let StepResult::Stepped(e) = r.step_forward() {
            if e.seq >= seq {
                break;
            }
        }
        r.visited()
    }

    // ---- Diff --------------------------------------------------------------

    /// Diff the primary and secondary journals.
    pub fn diff(&self) -> Result<RunDiff, ApiError> {
        let secondary = self
            .secondary
            .as_ref()
            .ok_or(ApiError::NoSecondaryJournal)?;
        Ok(diff_journals(&self.primary, secondary))
    }

    // ---- Branching ---------------------------------------------------------

    /// Create a branch from the primary journal at `branch_point`.
    pub fn create_branch(
        &mut self,
        branch_id: impl Into<String>,
        branch_point: Seq,
    ) -> Result<(), ApiError> {
        let b = Branch::new(branch_id, &self.primary, branch_point)?;
        self.branch_registry.insert(b);
        Ok(())
    }

    /// Append an entry to an existing branch.
    pub fn extend_branch(&mut self, branch_id: &str, entry: JournalEntry) -> Result<(), ApiError> {
        // We need to retrieve the branch mutably.  Re-create with modified registry.
        // Since BranchRegistry stores Vec<Branch> we locate by position.
        let branch = self
            .branch_registry
            .get(branch_id)
            .ok_or_else(|| ApiError::BranchNotFound(branch_id.to_string()))?;
        // Clone to avoid borrow conflict, then re-insert.
        let mut b = branch.clone();
        b.append(entry)?;
        // Replace in registry (insert overwrites via position).
        self.branch_registry.insert(b);
        Ok(())
    }

    /// Materialise a branch into a new journal for inspection.
    pub fn branch_journal(&self, branch_id: &str, new_run_id: RunId) -> Result<Journal, ApiError> {
        let b = self
            .branch_registry
            .get(branch_id)
            .ok_or_else(|| ApiError::BranchNotFound(branch_id.to_string()))?;
        Ok(b.to_journal(new_run_id)?)
    }

    /// List all branch ids.
    pub fn branch_ids(&self) -> Vec<&str> {
        self.branch_registry.list()
    }

    // ---- Annotations -------------------------------------------------------

    /// Add or update an annotation on the primary journal.
    pub fn annotate(&mut self, seq: Seq, text: impl Into<String>) {
        let a = Annotation::new(self.primary.run_id.clone(), seq, text);
        self.annotation_store.upsert(a);
    }

    /// Add or update a tagged annotation.
    pub fn annotate_tagged(&mut self, seq: Seq, text: impl Into<String>, tag: impl Into<String>) {
        let a = Annotation::new(self.primary.run_id.clone(), seq, text).with_tag(tag);
        self.annotation_store.upsert(a);
    }

    /// Return the annotation at the given seq, if any.
    pub fn get_annotation(&self, seq: Seq) -> Option<&Annotation> {
        self.annotation_store.get(&self.primary.run_id, seq)
    }

    /// Delete the annotation at the given seq.
    pub fn delete_annotation(&mut self, seq: Seq) -> Result<(), ApiError> {
        Ok(self.annotation_store.delete(&self.primary.run_id, seq)?)
    }

    /// Return all annotations for the primary run.
    pub fn all_annotations(&self) -> Vec<&Annotation> {
        self.annotation_store.all_for_run(&self.primary.run_id)
    }

    // ---- Summary -----------------------------------------------------------

    /// Return a summary map suitable for serialisation.
    pub fn summary(&self) -> HashMap<&'static str, String> {
        let mut m = HashMap::new();
        m.insert("run_id", self.primary.run_id.0.clone());
        m.insert("entry_count", self.primary.len().to_string());
        m.insert("branch_count", self.branch_registry.count().to_string());
        m.insert(
            "annotation_count",
            self.annotation_store.count().to_string(),
        );
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{EntryKind, JournalEntry, RunId};

    fn sc(seq: u64, from: &str, to: &str) -> JournalEntry {
        JournalEntry::new(
            RunId::new("r1"),
            seq,
            EntryKind::StateChange {
                from: from.into(),
                to: to.into(),
            },
        )
    }

    fn sample_session() -> DebugSession {
        DebugSession::new(vec![
            sc(0, "init", "planning"),
            sc(1, "planning", "executing"),
            sc(2, "executing", "done"),
        ])
        .unwrap()
    }

    #[test]
    fn session_state_at_works() {
        let session = sample_session();
        let snap = session.state_at(Seq(2)).unwrap();
        assert_eq!(snap.state, "done");
    }

    #[test]
    fn session_replay_all_visits_all() {
        let session = sample_session();
        let mut count = 0usize;
        session.replay_all(|_| count += 1);
        assert_eq!(count, 3);
    }

    #[test]
    fn session_diff_no_secondary_errors() {
        let session = sample_session();
        assert!(matches!(session.diff(), Err(ApiError::NoSecondaryJournal)));
    }

    #[test]
    fn session_diff_with_secondary() {
        let mut session = sample_session();
        session
            .load_secondary(vec![JournalEntry::new(
                RunId::new("r2"),
                0,
                EntryKind::StateChange {
                    from: "init".into(),
                    to: "planning".into(),
                },
            )])
            .unwrap();
        let diff = session.diff().unwrap();
        // r1 has 3 entries, r2 has 1: diverge at seq 1.
        assert!(diff.first_divergence.is_some());
    }

    #[test]
    fn session_create_and_retrieve_branch() {
        let mut session = sample_session();
        session.create_branch("alt", Seq(1)).unwrap();
        assert_eq!(session.branch_ids(), vec!["alt"]);
    }

    #[test]
    fn session_annotations_persist() {
        let mut session = sample_session();
        session.annotate(Seq(1), "suspect state transition");
        let got = session.get_annotation(Seq(1)).unwrap();
        assert_eq!(got.text, "suspect state transition");
    }

    #[test]
    fn session_summary_has_expected_keys() {
        let session = sample_session();
        let s = session.summary();
        assert!(s.contains_key("run_id"));
        assert!(s.contains_key("entry_count"));
    }
}
