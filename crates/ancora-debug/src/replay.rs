/// replay.rs - Step-through replay of a loaded journal.
///
/// [`Replayer`] wraps a [`Journal`] and maintains a cursor.  Callers can
/// advance the cursor one step at a time, jump to an arbitrary sequence
/// number, or reset to the beginning.  No live LLM or tool calls are ever
/// made during replay.
use crate::loader::{Journal, JournalEntry, Seq};

/// Direction of replay stepping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// Outcome of a single replay step.
#[derive(Debug, Clone)]
pub enum StepResult<'j> {
    /// Successfully moved to the given entry.
    Stepped(&'j JournalEntry),
    /// Already at the beginning (backward) or end (forward) of the journal.
    AtBoundary,
}

/// Step-through replayer over a journal.
pub struct Replayer<'j> {
    journal: &'j Journal,
    /// Current position; `None` means before the first entry.
    cursor: Option<Seq>,
}

impl<'j> Replayer<'j> {
    /// Create a new replayer positioned before the first entry.
    pub fn new(journal: &'j Journal) -> Self {
        Self {
            journal,
            cursor: None,
        }
    }

    /// Return the current cursor position.
    pub fn cursor(&self) -> Option<Seq> {
        self.cursor
    }

    /// Return the entry at the current cursor, if the cursor is positioned.
    pub fn current(&self) -> Option<&'j JournalEntry> {
        self.cursor.and_then(|s| self.journal.entry_at(s))
    }

    /// Advance one step forward.
    pub fn step_forward(&mut self) -> StepResult<'j> {
        let next_seq = match self.cursor {
            None => Seq(0),
            Some(Seq(n)) => {
                let next = n + 1;
                if next >= self.journal.len() as u64 {
                    return StepResult::AtBoundary;
                }
                Seq(next)
            }
        };
        self.cursor = Some(next_seq);
        StepResult::Stepped(self.journal.entry_at(next_seq).unwrap())
    }

    /// Step one position backward.
    pub fn step_backward(&mut self) -> StepResult<'j> {
        match self.cursor {
            None | Some(Seq(0)) => {
                self.cursor = None;
                StepResult::AtBoundary
            }
            Some(Seq(n)) => {
                let prev = Seq(n - 1);
                self.cursor = Some(prev);
                StepResult::Stepped(self.journal.entry_at(prev).unwrap())
            }
        }
    }

    /// Jump to an arbitrary sequence number.
    ///
    /// Returns `None` if the sequence number is out of range.
    pub fn seek(&mut self, seq: Seq) -> Option<&'j JournalEntry> {
        let entry = self.journal.entry_at(seq)?;
        self.cursor = Some(seq);
        Some(entry)
    }

    /// Reset the replayer to before the first entry.
    pub fn reset(&mut self) {
        self.cursor = None;
    }

    /// Collect all entries from the current cursor to the end and return
    /// them in order without advancing the cursor.
    pub fn remaining(&self) -> Vec<&'j JournalEntry> {
        let start = self.cursor.map(|s| s.0 + 1).unwrap_or(0) as usize;
        self.journal.entries()[start..].iter().collect()
    }

    /// Drain all entries from the start of the journal to the cursor
    /// (inclusive) and return them, without changing the cursor.
    pub fn visited(&self) -> Vec<&'j JournalEntry> {
        let end = self.cursor.map(|s| s.0 as usize + 1).unwrap_or(0);
        self.journal.entries()[..end].iter().collect()
    }

    /// Step through the entire journal from the current position,
    /// calling `f` for each entry.  The cursor ends at the last entry.
    pub fn run_to_end<F>(&mut self, mut f: F)
    where
        F: FnMut(&'j JournalEntry),
    {
        while let StepResult::Stepped(entry) = self.step_forward() {
            f(entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId};

    fn sample_journal() -> Journal {
        let entries: Vec<JournalEntry> = (0..5)
            .map(|i| {
                JournalEntry::new(
                    RunId::new("run-replay"),
                    i,
                    EntryKind::StateChange {
                        from: format!("s{}", i),
                        to: format!("s{}", i + 1),
                    },
                )
            })
            .collect();
        load_journal(entries).unwrap()
    }

    #[test]
    fn step_forward_advances_cursor() {
        let j = sample_journal();
        let mut r = Replayer::new(&j);
        assert!(r.cursor().is_none());
        r.step_forward();
        assert_eq!(r.cursor(), Some(Seq(0)));
    }

    #[test]
    fn step_backward_at_start_returns_boundary() {
        let j = sample_journal();
        let mut r = Replayer::new(&j);
        assert!(matches!(r.step_backward(), StepResult::AtBoundary));
    }

    #[test]
    fn seek_positions_cursor() {
        let j = sample_journal();
        let mut r = Replayer::new(&j);
        let entry = r.seek(Seq(3)).unwrap();
        assert_eq!(entry.seq, Seq(3));
        assert_eq!(r.cursor(), Some(Seq(3)));
    }

    #[test]
    fn run_to_end_visits_all() {
        let j = sample_journal();
        let mut r = Replayer::new(&j);
        let mut visited = 0usize;
        r.run_to_end(|_| visited += 1);
        assert_eq!(visited, 5);
    }

    #[test]
    fn reset_clears_cursor() {
        let j = sample_journal();
        let mut r = Replayer::new(&j);
        r.step_forward();
        r.reset();
        assert!(r.cursor().is_none());
    }
}
