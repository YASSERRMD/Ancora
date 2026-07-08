/// test_debug_api.rs - Verify the high-level debug API returns correct inspection data.
use std::collections::HashMap;

use crate::api::DebugSession;
use crate::loader::{EntryKind, JournalEntry, RunId, Seq};

fn sc(seq: u64, from: &str, to: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new("api-run"),
        seq,
        EntryKind::StateChange {
            from: from.into(),
            to: to.into(),
        },
    )
}

fn llm(seq: u64, prompt: &str, response: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new("api-run"),
        seq,
        EntryKind::LlmExchange {
            prompt: prompt.into(),
            response: response.into(),
        },
    )
}

fn tool(seq: u64, name: &str, output: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new("api-run"),
        seq,
        EntryKind::ToolCall {
            tool_name: name.into(),
            args: HashMap::new(),
            output: output.into(),
        },
    )
}

fn make_session() -> DebugSession {
    DebugSession::new(vec![
        sc(0, "boot", "idle"),
        llm(1, "Generate a plan", "Plan: A then B"),
        tool(2, "lookup", "result-foo"),
        sc(3, "idle", "executing"),
        llm(4, "Summarise", "Summary: done"),
        sc(5, "executing", "done"),
    ])
    .unwrap()
}

#[test]
fn api_state_at_returns_correct_state() {
    let s = make_session();
    assert_eq!(s.state_at(Seq(5)).unwrap().state, "done");
    assert_eq!(s.state_at(Seq(3)).unwrap().state, "executing");
    assert_eq!(s.state_at(Seq(0)).unwrap().state, "idle");
}

#[test]
fn api_llm_at_returns_most_recent() {
    let s = make_session();
    let snap = s.llm_at(Seq(5)).unwrap();
    assert_eq!(snap.prompt, "Summarise");
}

#[test]
fn api_tool_at_returns_tool() {
    let s = make_session();
    let snap = s.tool_at(Seq(5)).unwrap();
    assert_eq!(snap.tool_name, "lookup");
    assert_eq!(snap.output, "result-foo");
}

#[test]
fn api_replay_all_visits_six_entries() {
    let s = make_session();
    let mut count = 0usize;
    s.replay_all(|_| count += 1);
    assert_eq!(count, 6);
}

#[test]
fn api_create_branch_appears_in_list() {
    let mut s = make_session();
    s.create_branch("what-if", Seq(2)).unwrap();
    assert!(s.branch_ids().contains(&"what-if"));
}

#[test]
fn api_branch_journal_is_valid() {
    let mut s = make_session();
    s.create_branch("alt", Seq(3)).unwrap();
    let new_j = s.branch_journal("alt", RunId::new("alt-run")).unwrap();
    // 4 entries: seqs 0-3 from original.
    assert_eq!(new_j.len(), 4);
}

#[test]
fn api_annotate_and_retrieve() {
    let mut s = make_session();
    s.annotate(Seq(2), "tool call looks suspect");
    let ann = s.get_annotation(Seq(2)).unwrap();
    assert_eq!(ann.text, "tool call looks suspect");
}

#[test]
fn api_annotate_tagged() {
    let mut s = make_session();
    s.annotate_tagged(Seq(1), "slow response", "performance");
    let ann = s.get_annotation(Seq(1)).unwrap();
    assert_eq!(ann.tag.as_deref(), Some("performance"));
}

#[test]
fn api_all_annotations_returns_all() {
    let mut s = make_session();
    s.annotate(Seq(0), "first");
    s.annotate(Seq(3), "fourth");
    assert_eq!(s.all_annotations().len(), 2);
}

#[test]
fn api_diff_with_secondary() {
    let mut s = make_session();
    s.load_secondary(vec![sc(0, "boot", "idle"), sc(1, "idle", "DIFFERENT")])
        .unwrap();
    let diff = s.diff().unwrap();
    assert!(diff.first_divergence.is_some());
}

#[test]
fn api_summary_fields() {
    let s = make_session();
    let summary = s.summary();
    assert_eq!(summary["run_id"], "api-run");
    assert_eq!(summary["entry_count"], "6");
}
