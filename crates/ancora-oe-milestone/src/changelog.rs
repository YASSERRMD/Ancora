/// Category of a changelog entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Added,
    Fixed,
    Changed,
    Removed,
    Performance,
    Security,
    Docs,
}

impl std::fmt::Display for EntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EntryKind::Added => "Added",
            EntryKind::Fixed => "Fixed",
            EntryKind::Changed => "Changed",
            EntryKind::Removed => "Removed",
            EntryKind::Performance => "Performance",
            EntryKind::Security => "Security",
            EntryKind::Docs => "Docs",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct ChangelogEntry {
    pub kind: EntryKind,
    pub description: String,
    pub pr: Option<u32>,
}

impl ChangelogEntry {
    pub fn new(kind: EntryKind, description: impl Into<String>) -> Self {
        Self {
            kind,
            description: description.into(),
            pr: None,
        }
    }

    pub fn with_pr(mut self, pr: u32) -> Self {
        self.pr = Some(pr);
        self
    }

    pub fn render(&self) -> String {
        let pr_part = self.pr.map(|n| format!(" (#{n})")).unwrap_or_default();
        format!("- [{}] {}{}", self.kind, self.description, pr_part)
    }
}

#[derive(Debug, Clone)]
pub struct ChangelogSection {
    pub version: String,
    pub date: String,
    pub entries: Vec<ChangelogEntry>,
}

impl ChangelogSection {
    pub fn new(version: impl Into<String>, date: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            date: date.into(),
            entries: Vec::new(),
        }
    }

    pub fn add(mut self, entry: ChangelogEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!("## {} ({})\n\n", self.version, self.date);
        for e in &self.entries {
            out.push_str(&format!("{}\n", e.render()));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_renders_with_pr() {
        let e = ChangelogEntry::new(EntryKind::Added, "Histogram bucket support").with_pr(212);
        let r = e.render();
        assert!(r.contains("[Added]"));
        assert!(r.contains("(#212)"));
    }

    #[test]
    fn section_renders_all_entries() {
        let s = ChangelogSection::new("0.6.0", "2026-06-29")
            .add(ChangelogEntry::new(
                EntryKind::Fixed,
                "Race in metric flush",
            ))
            .add(ChangelogEntry::new(
                EntryKind::Performance,
                "Reduced allocations in tracer",
            ));
        let r = s.render();
        assert!(r.contains("0.6.0"));
        assert!(r.contains("[Fixed]"));
        assert!(r.contains("[Performance]"));
    }
}
