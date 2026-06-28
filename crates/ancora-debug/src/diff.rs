/// diff.rs - Diff two run journals to find where they diverge.
///
/// Two runs may share a common prefix and then diverge.  [`diff_journals`]
/// walks both journals in lockstep and returns a [`RunDiff`] describing
/// the point of divergence and per-position differences.

use crate::loader::{EntryKind, Journal, Seq};

/// Describes how two journal entries differ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryDiff {
    /// Both entries have the same kind and content.
    Equal,
    /// The entry kinds are the same but the content differs.
    ContentChanged { left: String, right: String },
    /// The entry kinds differ entirely.
    KindChanged { left: String, right: String },
    /// The left journal has an entry but the right does not.
    OnlyInLeft,
    /// The right journal has an entry but the left does not.
    OnlyInRight,
}

/// A single position in a pairwise journal diff.
#[derive(Debug, Clone)]
pub struct DiffLine {
    pub seq: Seq,
    pub diff: EntryDiff,
}

/// The result of diffing two journals.
#[derive(Debug, Clone)]
pub struct RunDiff {
    /// The sequence number at which the two runs first diverge.
    /// `None` if the journals are identical up to the length of the shorter one.
    pub first_divergence: Option<Seq>,
    /// Per-position diff lines.
    pub lines: Vec<DiffLine>,
}

fn entry_kind_label(kind: &EntryKind) -> &'static str {
    match kind {
        EntryKind::StateChange { .. } => "StateChange",
        EntryKind::LlmExchange { .. } => "LlmExchange",
        EntryKind::ToolCall { .. } => "ToolCall",
        EntryKind::Annotation { .. } => "Annotation",
    }
}

fn entry_summary(kind: &EntryKind) -> String {
    match kind {
        EntryKind::StateChange { from, to } => format!("{}->{}", from, to),
        EntryKind::LlmExchange { prompt, response } => {
            format!("prompt={:?} response={:?}", &prompt[..prompt.len().min(40)], &response[..response.len().min(40)])
        }
        EntryKind::ToolCall { tool_name, output, .. } => {
            format!("tool={} output={:?}", tool_name, &output[..output.len().min(40)])
        }
        EntryKind::Annotation { text } => format!("annotation={:?}", text),
    }
}

fn diff_entry(left: &EntryKind, right: &EntryKind) -> EntryDiff {
    let lk = entry_kind_label(left);
    let rk = entry_kind_label(right);
    if lk != rk {
        return EntryDiff::KindChanged {
            left: format!("{}: {}", lk, entry_summary(left)),
            right: format!("{}: {}", rk, entry_summary(right)),
        };
    }
    let ls = entry_summary(left);
    let rs = entry_summary(right);
    if ls == rs {
        EntryDiff::Equal
    } else {
        EntryDiff::ContentChanged { left: ls, right: rs }
    }
}

/// Diff two journals and return the result.
pub fn diff_journals(left: &Journal, right: &Journal) -> RunDiff {
    let max_len = left.len().max(right.len());
    let mut lines = Vec::with_capacity(max_len);
    let mut first_divergence = None;

    for i in 0..max_len as u64 {
        let seq = Seq(i);
        let le = left.entry_at(seq);
        let re = right.entry_at(seq);

        let d = match (le, re) {
            (Some(l), Some(r)) => diff_entry(&l.kind, &r.kind),
            (Some(_), None) => EntryDiff::OnlyInLeft,
            (None, Some(_)) => EntryDiff::OnlyInRight,
            (None, None) => break,
        };

        if d != EntryDiff::Equal && first_divergence.is_none() {
            first_divergence = Some(seq);
        }

        lines.push(DiffLine { seq, diff: d });
    }

    RunDiff { first_divergence, lines }
}

impl RunDiff {
    /// Return true if the two journals are identical (no divergence).
    pub fn is_identical(&self) -> bool {
        self.first_divergence.is_none()
            && self.lines.iter().all(|l| l.diff == EntryDiff::Equal)
    }

    /// Return only the lines that differ.
    pub fn changed_lines(&self) -> Vec<&DiffLine> {
        self.lines.iter().filter(|l| l.diff != EntryDiff::Equal).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::{load_journal, EntryKind, JournalEntry, RunId};

    fn sc(run: &str, seq: u64, from: &str, to: &str) -> JournalEntry {
        JournalEntry::new(
            RunId::new(run),
            seq,
            EntryKind::StateChange { from: from.into(), to: to.into() },
        )
    }

    #[test]
    fn identical_journals_report_no_divergence() {
        let j1 = load_journal(vec![sc("r1", 0, "a", "b"), sc("r1", 1, "b", "c")]).unwrap();
        let j2 = load_journal(vec![sc("r2", 0, "a", "b"), sc("r2", 1, "b", "c")]).unwrap();
        let diff = diff_journals(&j1, &j2);
        assert!(diff.is_identical());
    }

    #[test]
    fn diff_detects_content_change() {
        let j1 = load_journal(vec![sc("r1", 0, "a", "b")]).unwrap();
        let j2 = load_journal(vec![sc("r2", 0, "a", "X")]).unwrap();
        let diff = diff_journals(&j1, &j2);
        assert_eq!(diff.first_divergence, Some(Seq(0)));
        assert!(!diff.is_identical());
    }

    #[test]
    fn diff_detects_length_mismatch() {
        let j1 = load_journal(vec![sc("r1", 0, "a", "b"), sc("r1", 1, "b", "c")]).unwrap();
        let j2 = load_journal(vec![sc("r2", 0, "a", "b")]).unwrap();
        let diff = diff_journals(&j1, &j2);
        assert!(diff.changed_lines().iter().any(|l| l.diff == EntryDiff::OnlyInLeft));
    }

    #[test]
    fn changed_lines_filters_equal() {
        let j1 = load_journal(vec![sc("r1", 0, "a", "b"), sc("r1", 1, "b", "c")]).unwrap();
        let j2 = load_journal(vec![sc("r2", 0, "a", "b"), sc("r2", 1, "b", "X")]).unwrap();
        let diff = diff_journals(&j1, &j2);
        let changed = diff.changed_lines();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].seq, Seq(1));
    }
}
