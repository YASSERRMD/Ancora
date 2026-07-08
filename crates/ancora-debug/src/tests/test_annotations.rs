/// test_annotations.rs - Verify that annotations persist and are retrievable.
use crate::annotate::{AnnotateError, Annotation, AnnotationStore};
use crate::loader::{RunId, Seq};

#[test]
fn annotation_persists_after_upsert() {
    let mut store = AnnotationStore::new();
    let ann = Annotation::new(RunId::new("r1"), Seq(3), "something odd here");
    store.upsert(ann);
    let got = store.get(&RunId::new("r1"), Seq(3)).unwrap();
    assert_eq!(got.text, "something odd here");
    assert_eq!(got.seq.0, 3);
}

#[test]
fn annotation_tag_preserved() {
    let mut store = AnnotationStore::new();
    let ann = Annotation::new(RunId::new("r1"), Seq(0), "suspicious").with_tag("bug");
    store.upsert(ann);
    let got = store.get(&RunId::new("r1"), Seq(0)).unwrap();
    assert_eq!(got.tag.as_deref(), Some("bug"));
}

#[test]
fn annotation_update_replaces_text() {
    let mut store = AnnotationStore::new();
    store.upsert(Annotation::new(RunId::new("r1"), Seq(1), "first note"));
    store.upsert(Annotation::new(RunId::new("r1"), Seq(1), "revised note"));
    let got = store.get(&RunId::new("r1"), Seq(1)).unwrap();
    assert_eq!(got.text, "revised note");
    assert_eq!(store.count(), 1); // still only one annotation
}

#[test]
fn annotation_deletion_removes_it() {
    let mut store = AnnotationStore::new();
    store.upsert(Annotation::new(RunId::new("r1"), Seq(2), "to be removed"));
    store.delete(&RunId::new("r1"), Seq(2)).unwrap();
    assert!(store.get(&RunId::new("r1"), Seq(2)).is_none());
}

#[test]
fn deletion_nonexistent_returns_not_found() {
    let mut store = AnnotationStore::new();
    let err = store.delete(&RunId::new("r1"), Seq(42)).unwrap_err();
    assert!(matches!(err, AnnotateError::NotFound { seq: 42, .. }));
}

#[test]
fn all_for_run_sorted_by_seq() {
    let mut store = AnnotationStore::new();
    for seq in [7u64, 2, 5, 0, 9] {
        store.upsert(Annotation::new(
            RunId::new("run-a"),
            Seq(seq),
            format!("note-{}", seq),
        ));
    }
    let all = store.all_for_run(&RunId::new("run-a"));
    let seqs: Vec<u64> = all.iter().map(|a| a.seq.0).collect();
    assert_eq!(seqs, vec![0, 2, 5, 7, 9]);
}

#[test]
fn by_tag_only_returns_tagged_annotations() {
    let mut store = AnnotationStore::new();
    store.upsert(Annotation::new(RunId::new("r1"), Seq(0), "bug here").with_tag("bug"));
    store.upsert(Annotation::new(RunId::new("r1"), Seq(1), "a note").with_tag("note"));
    store.upsert(Annotation::new(RunId::new("r1"), Seq(2), "another bug").with_tag("bug"));
    let bugs = store.by_tag("bug");
    assert_eq!(bugs.len(), 2);
}

#[test]
fn annotations_isolated_by_run_id() {
    let mut store = AnnotationStore::new();
    store.upsert(Annotation::new(RunId::new("r1"), Seq(0), "run-1 note"));
    store.upsert(Annotation::new(RunId::new("r2"), Seq(0), "run-2 note"));
    let r1_ann = store.all_for_run(&RunId::new("r1"));
    assert_eq!(r1_ann.len(), 1);
    assert_eq!(r1_ann[0].text, "run-1 note");
}
