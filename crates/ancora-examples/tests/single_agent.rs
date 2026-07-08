use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_core::run::{Run, RunStatus};

#[test]
fn run_generates_unique_id() {
    let a = Run::generate();
    let b = Run::generate();
    assert_ne!(a.id, b.id);
    assert!(!a.id.is_empty());
}

#[test]
fn run_starts_in_pending_state() {
    let run = Run::new("example-run-1");
    assert_eq!(RunStatus::Pending, run.status);
}

#[test]
fn run_transitions_pending_to_running() {
    let mut run = Run::new("example-run-2");
    run.transition(RunStatus::Running).unwrap();
    assert_eq!(RunStatus::Running, run.status);
}

#[test]
fn run_transitions_running_to_completed() {
    let mut run = Run::new("example-run-3");
    run.transition(RunStatus::Running).unwrap();
    run.transition(RunStatus::Completed).unwrap();
    assert!(run.status.is_terminal());
}

#[test]
fn memory_store_starts_empty() {
    let store = MemoryStore::new();
    let events = store.read("run-1").unwrap();
    assert!(events.is_empty());
}

#[test]
fn run_id_is_uuid_format() {
    let run = Run::generate();
    let parts: Vec<&str> = run.id.split('-').collect();
    assert_eq!(5, parts.len(), "UUID v4 has 5 hyphen-separated groups");
}
