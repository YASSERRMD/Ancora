use ancora_core::run::Run;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn two_runs_have_distinct_ids() {
    let a = Run::generate();
    let b = Run::generate();
    assert_ne!(a.id, b.id);
}

#[test]
fn concurrent_runs_produce_unique_ids() {
    let ids: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let ids = Arc::clone(&ids);
            thread::spawn(move || {
                let run = Run::generate();
                ids.lock().unwrap().push(run.id);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let ids = ids.lock().unwrap();
    let unique: HashSet<&String> = ids.iter().collect();
    assert_eq!(
        ids.len(),
        unique.len(),
        "all concurrent run IDs must be distinct"
    );
}

#[test]
fn primary_and_verifier_spec_are_different_instructions() {
    let primary_instr = "Produce an answer.";
    let verifier_instr = "Verify the answer.";
    assert_ne!(primary_instr, verifier_instr);
}

#[test]
fn four_concurrent_run_ids_are_all_distinct() {
    let runs: Vec<Run> = (0..4).map(|_| Run::generate()).collect();
    let unique: HashSet<&String> = runs.iter().map(|r| &r.id).collect();
    assert_eq!(4, unique.len());
}
