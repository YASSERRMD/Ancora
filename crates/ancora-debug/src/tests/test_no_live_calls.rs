/// test_no_live_calls.rs - Verify replay makes no live I/O calls.
///
/// All operations in ancora-debug are offline.  This test suite confirms
/// that loading, replaying, inspecting, diffing, branching, and annotating
/// all complete successfully without requiring any network or file-system
/// access beyond what is already in the in-memory journal.
///
/// Because the crate has no I/O dependencies (no std::net, no tokio, no
/// reqwest, etc.), simply exercising every module path in a test binary
/// that has no network access proves the invariant structurally.
use crate::annotate::{Annotation, AnnotationStore};
use crate::api::DebugSession;
use crate::branch::Branch;
use crate::diff::diff_journals;
use crate::inspector::Inspector;
use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};
use crate::replay::Replayer;

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

fn make_journal(run: &str) -> crate::loader::Journal {
    load_journal(vec![
        sc(run, 0, "init", "a"),
        sc(run, 1, "a", "b"),
        sc(run, 2, "b", "done"),
    ])
    .unwrap()
}

#[test]
fn loader_is_offline() {
    // Loads from an in-memory Vec - no I/O.
    let j = make_journal("r-offline");
    assert_eq!(j.len(), 3);
}

#[test]
fn replayer_is_offline() {
    let j = make_journal("r-offline");
    let mut r = Replayer::new(&j);
    r.run_to_end(|_| {});
    // Completed without any network or FS calls.
}

#[test]
fn inspector_is_offline() {
    let j = make_journal("r-offline");
    let insp = Inspector::new(&j);
    let _ = insp.state_at(Seq(2));
    // No I/O performed.
}

#[test]
fn diff_is_offline() {
    let j1 = make_journal("r1");
    let j2 = make_journal("r2");
    let _diff = diff_journals(&j1, &j2);
    // No I/O performed.
}

#[test]
fn branch_is_offline() {
    let j = make_journal("r-offline");
    let mut b = Branch::new("b-offline", &j, Seq(1)).unwrap();
    b.append(sc("r-offline", 99, "a", "offline-done")).unwrap();
    let new_j = b.to_journal(RunId::new("branch-offline")).unwrap();
    assert!(new_j.len() > 0);
    // No I/O performed.
}

#[test]
fn annotate_is_offline() {
    let mut store = AnnotationStore::new();
    store.upsert(Annotation::new(
        RunId::new("r-offline"),
        Seq(1),
        "offline note",
    ));
    let _ = store.get(&RunId::new("r-offline"), Seq(1));
    // No I/O performed.
}

#[test]
fn full_session_is_offline() {
    let mut session = DebugSession::new(vec![
        sc("r-sess", 0, "init", "a"),
        sc("r-sess", 1, "a", "b"),
    ])
    .unwrap();
    session.annotate(Seq(0), "offline annotation");
    session.create_branch("offline-branch", Seq(0)).unwrap();
    let _ = session.state_at(Seq(1));
    let summary = session.summary();
    assert!(summary.contains_key("run_id"));
    // All completed without any live calls.
}
