//! Index of runnable examples demonstrating observability and eval features.

/// A single example entry in the examples index.
#[derive(Debug, Clone)]
pub struct ExampleEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub path: String,
}

impl ExampleEntry {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            tags: Vec::new(),
            path: path.into(),
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// The full index of available examples.
#[derive(Debug, Default)]
pub struct ExamplesIndex {
    entries: Vec<ExampleEntry>,
}

impl ExamplesIndex {
    pub fn add(&mut self, entry: ExampleEntry) {
        self.entries.push(entry);
    }

    pub fn by_tag(&self, tag: &str) -> Vec<&ExampleEntry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    pub fn find_by_id(&self, id: &str) -> Option<&ExampleEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn total(&self) -> usize {
        self.entries.len()
    }

    /// Return all examples, sorted alphabetically by title.
    pub fn sorted_by_title(&self) -> Vec<&ExampleEntry> {
        let mut refs: Vec<&ExampleEntry> = self.entries.iter().collect();
        refs.sort_by(|a, b| a.title.cmp(&b.title));
        refs
    }
}

/// Builds the standard Ancora OE examples index.
pub fn build_default_index() -> ExamplesIndex {
    let mut index = ExamplesIndex::default();
    index.add(
        ExampleEntry::new(
            "basic_tracing",
            "Basic Tracing",
            "Demonstrates span creation and propagation.",
            "examples/basic_tracing",
        )
        .with_tag("tracing"),
    );
    index.add(
        ExampleEntry::new(
            "cost_report",
            "Cost Report",
            "Generates a cost summary from token usage.",
            "examples/cost_report",
        )
        .with_tag("cost"),
    );
    index.add(
        ExampleEntry::new(
            "drift_alert",
            "Drift Alert",
            "Detects quality drift and emits alerts.",
            "examples/drift_alert",
        )
        .with_tag("drift"),
    );
    index
}
