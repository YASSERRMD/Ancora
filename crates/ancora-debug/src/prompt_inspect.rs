/// prompt_inspect.rs - Focused helpers for inspecting LLM prompt and response
/// data at a specific step in a journal.
///
/// This module provides thin wrappers around [`Inspector`] that surface only
/// prompt/response data, making it convenient for UIs that display LLM
/// exchange details without needing access to the full inspector.

use crate::inspector::{Inspector, LlmSnapshot};
use crate::loader::{Journal, Seq};

/// Return the prompt text sent to the LLM at or before `seq`.
///
/// Returns `None` if no LLM exchange exists up to that point.
pub fn prompt_at(journal: &Journal, seq: Seq) -> Option<String> {
    Inspector::new(journal).llm_at(seq).map(|s| s.prompt)
}

/// Return the response text from the LLM at or before `seq`.
///
/// Returns `None` if no LLM exchange exists up to that point.
pub fn response_at(journal: &Journal, seq: Seq) -> Option<String> {
    Inspector::new(journal).llm_at(seq).map(|s| s.response)
}

/// Return both prompt and response at or before `seq` as a pair.
pub fn exchange_at(journal: &Journal, seq: Seq) -> Option<(String, String)> {
    Inspector::new(journal).llm_at(seq).map(|s| (s.prompt, s.response))
}

/// Return all LLM exchanges up to `seq` in sequence order.
pub fn all_exchanges(journal: &Journal, seq: Seq) -> Vec<LlmSnapshot> {
    Inspector::new(journal).all_llm_exchanges(seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId};

    fn make_journal() -> Journal {
        let entries = vec![
            JournalEntry::new(
                RunId::new("r-prompt"),
                0,
                EntryKind::StateChange { from: "init".into(), to: "ready".into() },
            ),
            JournalEntry::new(
                RunId::new("r-prompt"),
                1,
                EntryKind::LlmExchange {
                    prompt: "Describe the problem.".into(),
                    response: "The problem is X.".into(),
                },
            ),
            JournalEntry::new(
                RunId::new("r-prompt"),
                2,
                EntryKind::LlmExchange {
                    prompt: "Suggest a solution.".into(),
                    response: "Do Y.".into(),
                },
            ),
        ];
        load_journal(entries).unwrap()
    }

    #[test]
    fn prompt_at_returns_first_prompt() {
        let j = make_journal();
        assert_eq!(
            prompt_at(&j, Seq(1)).as_deref(),
            Some("Describe the problem.")
        );
    }

    #[test]
    fn response_at_returns_latest_response() {
        let j = make_journal();
        assert_eq!(response_at(&j, Seq(2)).as_deref(), Some("Do Y."));
    }

    #[test]
    fn exchange_at_returns_pair() {
        let j = make_journal();
        let (p, r) = exchange_at(&j, Seq(1)).unwrap();
        assert_eq!(p, "Describe the problem.");
        assert_eq!(r, "The problem is X.");
    }

    #[test]
    fn all_exchanges_count() {
        let j = make_journal();
        assert_eq!(all_exchanges(&j, Seq(2)).len(), 2);
    }

    #[test]
    fn prompt_at_before_any_exchange_is_none() {
        let j = make_journal();
        assert!(prompt_at(&j, Seq(0)).is_none());
    }
}
