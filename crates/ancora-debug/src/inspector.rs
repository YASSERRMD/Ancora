/// inspector.rs - Inspect agent state, prompts, and tool I/O at any seq.
///
/// The [`Inspector`] provides a read-only view into a journal at a specific
/// sequence point.  It surfaces the current state string, the most recent
/// LLM exchange, and the most recent tool invocation without requiring
/// callers to walk the journal themselves.

use std::collections::HashMap;

use crate::loader::{EntryKind, Journal, JournalEntry, Seq};

/// A snapshot of observable agent state at a given sequence number.
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    /// The sequence number this snapshot describes.
    pub seq: Seq,
    /// The current state name (e.g. "planning", "executing").
    pub state: String,
}

/// The most recent LLM exchange visible up to a given seq.
#[derive(Debug, Clone)]
pub struct LlmSnapshot {
    pub seq: Seq,
    pub prompt: String,
    pub response: String,
}

/// The most recent tool invocation visible up to a given seq.
#[derive(Debug, Clone)]
pub struct ToolSnapshot {
    pub seq: Seq,
    pub tool_name: String,
    pub args: HashMap<String, String>,
    pub output: String,
}

/// Read-only inspector over a journal.
pub struct Inspector<'j> {
    journal: &'j Journal,
}

impl<'j> Inspector<'j> {
    pub fn new(journal: &'j Journal) -> Self {
        Self { journal }
    }

    /// Return the agent state at the given sequence number.
    ///
    /// Walks backward from `seq` to find the most recent state-change entry.
    /// Returns `None` if no state-change entry exists at or before `seq`.
    pub fn state_at(&self, seq: Seq) -> Option<StateSnapshot> {
        let limit = seq.0.min(self.journal.len() as u64 - 1);
        for i in (0..=limit).rev() {
            if let Some(entry) = self.journal.entry_at(Seq(i)) {
                if let EntryKind::StateChange { to, .. } = &entry.kind {
                    return Some(StateSnapshot { seq: Seq(i), state: to.clone() });
                }
            }
        }
        None
    }

    /// Return the most recent LLM exchange at or before `seq`.
    pub fn llm_at(&self, seq: Seq) -> Option<LlmSnapshot> {
        let limit = seq.0.min(self.journal.len() as u64 - 1);
        for i in (0..=limit).rev() {
            if let Some(entry) = self.journal.entry_at(Seq(i)) {
                if let EntryKind::LlmExchange { prompt, response } = &entry.kind {
                    return Some(LlmSnapshot {
                        seq: Seq(i),
                        prompt: prompt.clone(),
                        response: response.clone(),
                    });
                }
            }
        }
        None
    }

    /// Return the most recent tool call at or before `seq`.
    pub fn tool_at(&self, seq: Seq) -> Option<ToolSnapshot> {
        let limit = seq.0.min(self.journal.len() as u64 - 1);
        for i in (0..=limit).rev() {
            if let Some(entry) = self.journal.entry_at(Seq(i)) {
                if let EntryKind::ToolCall { tool_name, args, output } = &entry.kind {
                    return Some(ToolSnapshot {
                        seq: Seq(i),
                        tool_name: tool_name.clone(),
                        args: args.clone(),
                        output: output.clone(),
                    });
                }
            }
        }
        None
    }

    /// Return all entries at or before `seq` that match the given predicate.
    pub fn entries_matching<F>(&self, seq: Seq, predicate: F) -> Vec<&'j JournalEntry>
    where
        F: Fn(&JournalEntry) -> bool,
    {
        let limit = (seq.0 as usize + 1).min(self.journal.len());
        self.journal.entries()[..limit].iter().filter(|e| predicate(e)).collect()
    }

    /// Return all LLM exchanges in the journal up to `seq`.
    pub fn all_llm_exchanges(&self, seq: Seq) -> Vec<LlmSnapshot> {
        let limit = (seq.0 as usize + 1).min(self.journal.len());
        self.journal.entries()[..limit]
            .iter()
            .filter_map(|e| {
                if let EntryKind::LlmExchange { prompt, response } = &e.kind {
                    Some(LlmSnapshot {
                        seq: e.seq,
                        prompt: prompt.clone(),
                        response: response.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Return all tool calls in the journal up to `seq`.
    pub fn all_tool_calls(&self, seq: Seq) -> Vec<ToolSnapshot> {
        let limit = (seq.0 as usize + 1).min(self.journal.len());
        self.journal.entries()[..limit]
            .iter()
            .filter_map(|e| {
                if let EntryKind::ToolCall { tool_name, args, output } = &e.kind {
                    Some(ToolSnapshot {
                        seq: e.seq,
                        tool_name: tool_name.clone(),
                        args: args.clone(),
                        output: output.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId};

    fn build_journal() -> Journal {
        let mut args = HashMap::new();
        args.insert("input".into(), "hello".into());

        let entries = vec![
            JournalEntry::new(
                RunId::new("r1"),
                0,
                EntryKind::StateChange { from: "init".into(), to: "planning".into() },
            ),
            JournalEntry::new(
                RunId::new("r1"),
                1,
                EntryKind::LlmExchange {
                    prompt: "Plan the task".into(),
                    response: "Step 1: gather info".into(),
                },
            ),
            JournalEntry::new(
                RunId::new("r1"),
                2,
                EntryKind::ToolCall {
                    tool_name: "search".into(),
                    args,
                    output: "result: foo".into(),
                },
            ),
            JournalEntry::new(
                RunId::new("r1"),
                3,
                EntryKind::StateChange { from: "planning".into(), to: "executing".into() },
            ),
        ];
        load_journal(entries).unwrap()
    }

    #[test]
    fn state_at_returns_most_recent() {
        let j = build_journal();
        let insp = Inspector::new(&j);
        let snap = insp.state_at(Seq(3)).unwrap();
        assert_eq!(snap.state, "executing");
    }

    #[test]
    fn state_at_intermediate() {
        let j = build_journal();
        let insp = Inspector::new(&j);
        let snap = insp.state_at(Seq(1)).unwrap();
        assert_eq!(snap.state, "planning");
    }

    #[test]
    fn llm_at_returns_prompt() {
        let j = build_journal();
        let insp = Inspector::new(&j);
        let snap = insp.llm_at(Seq(3)).unwrap();
        assert_eq!(snap.prompt, "Plan the task");
    }

    #[test]
    fn tool_at_returns_tool_info() {
        let j = build_journal();
        let insp = Inspector::new(&j);
        let snap = insp.tool_at(Seq(3)).unwrap();
        assert_eq!(snap.tool_name, "search");
        assert_eq!(snap.output, "result: foo");
    }

    #[test]
    fn all_tool_calls_count() {
        let j = build_journal();
        let insp = Inspector::new(&j);
        let calls = insp.all_tool_calls(Seq(3));
        assert_eq!(calls.len(), 1);
    }
}
