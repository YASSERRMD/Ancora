/// A single upgrade note for transitioning between versions.
#[derive(Debug, Clone)]
pub struct UpgradeNote {
    pub from_version: String,
    pub to_version: String,
    pub breaking: bool,
    pub title: String,
    pub body: String,
    pub migration_steps: Vec<String>,
}

impl UpgradeNote {
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        breaking: bool,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            from_version: from.into(),
            to_version: to.into(),
            breaking,
            title: title.into(),
            body: body.into(),
            migration_steps: Vec::new(),
        }
    }

    pub fn add_step(mut self, step: impl Into<String>) -> Self {
        self.migration_steps.push(step.into());
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!(
            "{} {} -> {}: {}\n  {}\n",
            if self.breaking {
                "[BREAKING]"
            } else {
                "[non-breaking]"
            },
            self.from_version,
            self.to_version,
            self.title,
            self.body
        );
        for (i, step) in self.migration_steps.iter().enumerate() {
            out.push_str(&format!("  {}. {}\n", i + 1, step));
        }
        out
    }
}

/// Collect all breaking notes.
pub fn breaking_notes(notes: &[UpgradeNote]) -> Vec<&UpgradeNote> {
    notes.iter().filter(|n| n.breaking).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breaking_note_renders_with_tag() {
        let n = UpgradeNote::new(
            "0.5.0",
            "0.6.0",
            true,
            "Renamed trace exporter config key",
            "The key `otel_endpoint` was renamed to `exporter_endpoint`.",
        )
        .add_step("Update your config file.")
        .add_step("Restart the agent.");
        let r = n.render();
        assert!(r.contains("[BREAKING]"));
        assert!(r.contains("0.5.0"));
        assert!(r.contains("1. Update your config file."));
    }

    #[test]
    fn non_breaking_note() {
        let n = UpgradeNote::new(
            "0.5.0",
            "0.6.0",
            false,
            "Added histogram support",
            "New API.",
        );
        assert!(!n.breaking);
        assert!(n.render().contains("[non-breaking]"));
    }

    #[test]
    fn breaking_notes_filtered() {
        let notes = vec![
            UpgradeNote::new("0.5", "0.6", false, "a", "b"),
            UpgradeNote::new("0.5", "0.6", true, "c", "d"),
        ];
        let b = breaking_notes(&notes);
        assert_eq!(b.len(), 1);
        assert!(b[0].title.contains("c"));
    }
}
