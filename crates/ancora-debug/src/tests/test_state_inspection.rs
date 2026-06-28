/// test_state_inspection.rs - Verify that the inspector returns accurate state.

use std::collections::HashMap;

use crate::inspector::Inspector;
use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};

fn make_journal() -> crate::loader::Journal {
    let mut args = HashMap::new();
    args.insert("q".into(), "test query".into());

    let entries = vec![
        JournalEntry::new(
            RunId::new("r-insp"),
            0,
            EntryKind::StateChange { from: "boot".into(), to: "idle".into() },
        ),
        JournalEntry::new(
            RunId::new("r-insp"),
            1,
            EntryKind::LlmExchange {
                prompt: "What is the plan?".into(),
                response: "Step A then step B.".into(),
            },
        ),
        JournalEntry::new(
            RunId::new("r-insp"),
            2,
            EntryKind::ToolCall {
                tool_name: "calculator".into(),
                args: args.clone(),
                output: "42".into(),
            },
        ),
        JournalEntry::new(
            RunId::new("r-insp"),
            3,
            EntryKind::StateChange { from: "idle".into(), to: "working".into() },
        ),
        JournalEntry::new(
            RunId::new("r-insp"),
            4,
            EntryKind::LlmExchange {
                prompt: "Summarise result.".into(),
                response: "The answer is 42.".into(),
            },
        ),
        JournalEntry::new(
            RunId::new("r-insp"),
            5,
            EntryKind::StateChange { from: "working".into(), to: "done".into() },
        ),
    ];
    load_journal(entries).unwrap()
}

#[test]
fn state_at_first_entry() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    let snap = insp.state_at(Seq(0)).unwrap();
    assert_eq!(snap.state, "idle");
}

#[test]
fn state_at_final_entry() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    let snap = insp.state_at(Seq(5)).unwrap();
    assert_eq!(snap.state, "done");
}

#[test]
fn state_at_non_state_entry_walks_back() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    // seq 4 is an LLM exchange; state should still be "working" from seq 3.
    let snap = insp.state_at(Seq(4)).unwrap();
    assert_eq!(snap.state, "working");
}

#[test]
fn llm_at_returns_correct_prompt() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    let snap = insp.llm_at(Seq(1)).unwrap();
    assert_eq!(snap.prompt, "What is the plan?");
}

#[test]
fn llm_at_later_seq_returns_latest_llm() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    let snap = insp.llm_at(Seq(5)).unwrap();
    assert_eq!(snap.prompt, "Summarise result.");
}

#[test]
fn tool_at_returns_tool_name_and_output() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    let snap = insp.tool_at(Seq(5)).unwrap();
    assert_eq!(snap.tool_name, "calculator");
    assert_eq!(snap.output, "42");
}

#[test]
fn all_llm_exchanges_count() {
    let j = make_journal();
    let insp = Inspector::new(&j);
    let exchanges = insp.all_llm_exchanges(Seq(5));
    assert_eq!(exchanges.len(), 2);
}
