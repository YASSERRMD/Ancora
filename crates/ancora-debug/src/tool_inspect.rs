/// tool_inspect.rs - Focused helpers for inspecting tool I/O at a specific step.
///
/// This module provides thin wrappers around [`Inspector`] that surface only
/// tool call data (name, arguments, output), making it convenient for UIs
/// that display tool invocation details without needing the full inspector.

use crate::inspector::{Inspector, ToolSnapshot};
use crate::loader::{Journal, Seq};

/// Return the name of the tool invoked at or before `seq`.
///
/// Returns `None` if no tool call exists up to that point.
pub fn tool_name_at(journal: &Journal, seq: Seq) -> Option<String> {
    Inspector::new(journal).tool_at(seq).map(|s| s.tool_name)
}

/// Return the output produced by the tool at or before `seq`.
pub fn tool_output_at(journal: &Journal, seq: Seq) -> Option<String> {
    Inspector::new(journal).tool_at(seq).map(|s| s.output)
}

/// Return the full tool snapshot (name, args, output) at or before `seq`.
pub fn tool_snapshot_at(journal: &Journal, seq: Seq) -> Option<ToolSnapshot> {
    Inspector::new(journal).tool_at(seq)
}

/// Return all tool calls up to `seq` in sequence order.
pub fn all_tool_calls(journal: &Journal, seq: Seq) -> Vec<ToolSnapshot> {
    Inspector::new(journal).all_tool_calls(seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId};

    fn make_journal() -> Journal {
        let mut args1 = HashMap::new();
        args1.insert("q".into(), "rust".into());
        let mut args2 = HashMap::new();
        args2.insert("url".into(), "https://example.com".into());

        let entries = vec![
            JournalEntry::new(
                RunId::new("r-tool"),
                0,
                EntryKind::StateChange { from: "init".into(), to: "ready".into() },
            ),
            JournalEntry::new(
                RunId::new("r-tool"),
                1,
                EntryKind::ToolCall {
                    tool_name: "search".into(),
                    args: args1,
                    output: "10 results".into(),
                },
            ),
            JournalEntry::new(
                RunId::new("r-tool"),
                2,
                EntryKind::ToolCall {
                    tool_name: "fetch".into(),
                    args: args2,
                    output: "<html>...</html>".into(),
                },
            ),
        ];
        load_journal(entries).unwrap()
    }

    #[test]
    fn tool_name_at_returns_name() {
        let j = make_journal();
        assert_eq!(tool_name_at(&j, Seq(1)).as_deref(), Some("search"));
    }

    #[test]
    fn tool_output_at_returns_latest() {
        let j = make_journal();
        assert_eq!(
            tool_output_at(&j, Seq(2)).as_deref(),
            Some("<html>...</html>")
        );
    }

    #[test]
    fn tool_snapshot_contains_args() {
        let j = make_journal();
        let snap = tool_snapshot_at(&j, Seq(1)).unwrap();
        assert_eq!(snap.args.get("q").map(|s| s.as_str()), Some("rust"));
    }

    #[test]
    fn all_tool_calls_count() {
        let j = make_journal();
        assert_eq!(all_tool_calls(&j, Seq(2)).len(), 2);
    }

    #[test]
    fn tool_name_at_no_tool_is_none() {
        let j = make_journal();
        assert!(tool_name_at(&j, Seq(0)).is_none());
    }
}
