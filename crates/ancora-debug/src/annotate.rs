/// annotate.rs - Attach developer annotations to specific journal entries.
///
/// Annotations are stored separately from the journal so that the original
/// run data is never modified.  They are keyed by (run_id, seq) and can be
/// retrieved, updated, or deleted independently.

use std::collections::HashMap;

use crate::loader::{RunId, Seq};

/// A developer annotation attached to a specific sequence number.
#[derive(Debug, Clone)]
pub struct Annotation {
    pub run_id: RunId,
    pub seq: Seq,
    /// Free-form text written by the developer.
    pub text: String,
    /// An optional severity tag (e.g. "bug", "note", "warning").
    pub tag: Option<String>,
}

impl Annotation {
    pub fn new(run_id: RunId, seq: Seq, text: impl Into<String>) -> Self {
        Self { run_id, seq, text: text.into(), tag: None }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

/// Errors that can occur when managing annotations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotateError {
    /// No annotation exists at the given position.
    NotFound { run_id: String, seq: u64 },
}

impl std::fmt::Display for AnnotateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnotateError::NotFound { run_id, seq } => {
                write!(f, "no annotation for run {} at seq {}", run_id, seq)
            }
        }
    }
}

impl std::error::Error for AnnotateError {}

type AnnotationKey = (String, u64); // (run_id, seq)

fn key(run_id: &RunId, seq: Seq) -> AnnotationKey {
    (run_id.0.clone(), seq.0)
}

/// Store for all annotations in a debugging session.
#[derive(Default)]
pub struct AnnotationStore {
    inner: HashMap<AnnotationKey, Annotation>,
}

impl AnnotationStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace the annotation at (run_id, seq).
    pub fn upsert(&mut self, annotation: Annotation) {
        let k = key(&annotation.run_id, annotation.seq);
        self.inner.insert(k, annotation);
    }

    /// Return the annotation at (run_id, seq), if any.
    pub fn get(&self, run_id: &RunId, seq: Seq) -> Option<&Annotation> {
        self.inner.get(&key(run_id, seq))
    }

    /// Delete the annotation at (run_id, seq).
    pub fn delete(&mut self, run_id: &RunId, seq: Seq) -> Result<(), AnnotateError> {
        self.inner
            .remove(&key(run_id, seq))
            .map(|_| ())
            .ok_or_else(|| AnnotateError::NotFound {
                run_id: run_id.0.clone(),
                seq: seq.0,
            })
    }

    /// Return all annotations for the given run, sorted by seq.
    pub fn all_for_run(&self, run_id: &RunId) -> Vec<&Annotation> {
        let mut annotations: Vec<&Annotation> = self
            .inner
            .iter()
            .filter(|((rid, _), _)| rid == &run_id.0)
            .map(|(_, a)| a)
            .collect();
        annotations.sort_by_key(|a| a.seq);
        annotations
    }

    /// Return all annotations with the given tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&Annotation> {
        self.inner
            .values()
            .filter(|a| a.tag.as_deref() == Some(tag))
            .collect()
    }

    /// Total number of annotations.
    pub fn count(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::RunId;

    #[test]
    fn upsert_and_retrieve() {
        let mut store = AnnotationStore::new();
        let a = Annotation::new(RunId::new("r1"), Seq(2), "interesting step");
        store.upsert(a);
        let got = store.get(&RunId::new("r1"), Seq(2)).unwrap();
        assert_eq!(got.text, "interesting step");
    }

    #[test]
    fn delete_removes_annotation() {
        let mut store = AnnotationStore::new();
        store.upsert(Annotation::new(RunId::new("r1"), Seq(0), "temp note"));
        store.delete(&RunId::new("r1"), Seq(0)).unwrap();
        assert!(store.get(&RunId::new("r1"), Seq(0)).is_none());
    }

    #[test]
    fn delete_nonexistent_returns_error() {
        let mut store = AnnotationStore::new();
        let err = store.delete(&RunId::new("r1"), Seq(99));
        assert!(matches!(err, Err(AnnotateError::NotFound { .. })));
    }

    #[test]
    fn all_for_run_sorted() {
        let mut store = AnnotationStore::new();
        store.upsert(Annotation::new(RunId::new("r1"), Seq(5), "e"));
        store.upsert(Annotation::new(RunId::new("r1"), Seq(1), "a"));
        store.upsert(Annotation::new(RunId::new("r1"), Seq(3), "c"));
        let all = store.all_for_run(&RunId::new("r1"));
        let seqs: Vec<u64> = all.iter().map(|a| a.seq.0).collect();
        assert_eq!(seqs, vec![1, 3, 5]);
    }

    #[test]
    fn by_tag_filters_correctly() {
        let mut store = AnnotationStore::new();
        store.upsert(Annotation::new(RunId::new("r1"), Seq(0), "a bug").with_tag("bug"));
        store.upsert(Annotation::new(RunId::new("r1"), Seq(1), "a note").with_tag("note"));
        let bugs = store.by_tag("bug");
        assert_eq!(bugs.len(), 1);
    }

    #[test]
    fn upsert_replaces_existing() {
        let mut store = AnnotationStore::new();
        store.upsert(Annotation::new(RunId::new("r1"), Seq(0), "first"));
        store.upsert(Annotation::new(RunId::new("r1"), Seq(0), "updated"));
        assert_eq!(store.get(&RunId::new("r1"), Seq(0)).unwrap().text, "updated");
        assert_eq!(store.count(), 1);
    }
}
