//! Smoke test verifying that all example helpers compile and basic invariants hold.

use ancora_core::run::{Run, RunStatus};
use ancora_examples::{keyword_retrieve, Passage, RunJournal, Span, TokenEstimator};
use std::collections::HashSet;

#[test]
fn all_helpers_are_importable_and_functional() {
    // RunJournal
    let mut j = RunJournal::new();
    j.record_run("smoke");
    j.append_event("smoke", r#"{"kind":"started"}"#);
    assert_eq!(1, j.run_count());
    assert_eq!(1, j.events_for_run("smoke").len());

    // Span + TokenEstimator
    let mut s = Span::new("smoke.span");
    s.set_attribute(
        "tokens",
        TokenEstimator::estimate_tokens("hello world").to_string(),
    );
    let ms = s.end_ms();
    assert!(ms < 10_000);

    // keyword_retrieve
    let corpus = vec![
        Passage::new("a.md", "hello world"),
        Passage::new("b.md", "foo bar"),
    ];
    let hits = keyword_retrieve(&corpus, "hello", 1);
    assert_eq!("a.md", hits[0].key);

    // Run lifecycle
    let mut run = Run::generate();
    run.transition(RunStatus::Running).unwrap();
    run.transition(RunStatus::Completed).unwrap();
    assert!(run.status.is_terminal());
}

#[test]
fn example_run_ids_are_globally_unique_across_all_examples() {
    let ids: Vec<String> = (0..10).map(|_| Run::generate().id).collect();
    let unique: HashSet<&String> = ids.iter().collect();
    assert_eq!(10, unique.len());
}
