/// palette - Node palette: catalog of available agent, tool, and verifier node types.

use std::collections::HashMap;

/// Category of a palette entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeCategory {
    Agent,
    Tool,
    Verifier,
    Control,
    Custom(String),
}

impl std::fmt::Display for NodeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeCategory::Agent => write!(f, "agent"),
            NodeCategory::Tool => write!(f, "tool"),
            NodeCategory::Verifier => write!(f, "verifier"),
            NodeCategory::Control => write!(f, "control"),
            NodeCategory::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

/// A single entry in the node palette.
#[derive(Debug, Clone)]
pub struct PaletteEntry {
    pub kind: String,
    pub label: String,
    pub category: NodeCategory,
    pub description: String,
    /// Default configuration key-value pairs for new nodes.
    pub defaults: HashMap<String, String>,
}

impl PaletteEntry {
    pub fn new(
        kind: impl Into<String>,
        label: impl Into<String>,
        category: NodeCategory,
        description: impl Into<String>,
    ) -> Self {
        PaletteEntry {
            kind: kind.into(),
            label: label.into(),
            category,
            description: description.into(),
            defaults: HashMap::new(),
        }
    }

    pub fn with_default(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.defaults.insert(key.into(), value.into());
        self
    }
}

/// The palette holds all registered node types.
#[derive(Debug, Default, Clone)]
pub struct Palette {
    entries: Vec<PaletteEntry>,
}

impl Palette {
    pub fn new() -> Self {
        Palette::default()
    }

    /// Build the default palette with built-in node types.
    pub fn default_palette() -> Self {
        let mut p = Palette::new();

        // Agents
        p.register(PaletteEntry::new(
            "agent.llm",
            "LLM Agent",
            NodeCategory::Agent,
            "A language-model agent that processes prompts.",
        ).with_default("model", "default").with_default("temperature", "0.7"));

        p.register(PaletteEntry::new(
            "agent.retrieval",
            "Retrieval Agent",
            NodeCategory::Agent,
            "An agent that retrieves documents from a store.",
        ).with_default("top_k", "5"));

        p.register(PaletteEntry::new(
            "agent.classifier",
            "Classifier Agent",
            NodeCategory::Agent,
            "Classifies input into one of several categories.",
        ));

        // Tools
        p.register(PaletteEntry::new(
            "tool.web_search",
            "Web Search",
            NodeCategory::Tool,
            "Executes a web search and returns snippets.",
        ).with_default("max_results", "10"));

        p.register(PaletteEntry::new(
            "tool.code_exec",
            "Code Executor",
            NodeCategory::Tool,
            "Runs a sandboxed code snippet.",
        ).with_default("language", "python"));

        p.register(PaletteEntry::new(
            "tool.file_reader",
            "File Reader",
            NodeCategory::Tool,
            "Reads a file from the local filesystem.",
        ));

        // Verifiers
        p.register(PaletteEntry::new(
            "verifier.json_schema",
            "JSON Schema Verifier",
            NodeCategory::Verifier,
            "Validates output against a JSON schema.",
        ));

        p.register(PaletteEntry::new(
            "verifier.hallucination",
            "Hallucination Detector",
            NodeCategory::Verifier,
            "Checks for unsupported factual claims.",
        ).with_default("threshold", "0.5"));

        p.register(PaletteEntry::new(
            "verifier.toxicity",
            "Toxicity Filter",
            NodeCategory::Verifier,
            "Filters out toxic or harmful content.",
        ));

        // Control flow
        p.register(PaletteEntry::new(
            "control.router",
            "Router",
            NodeCategory::Control,
            "Routes the message to one of several downstream nodes.",
        ));

        p.register(PaletteEntry::new(
            "control.merge",
            "Merge",
            NodeCategory::Control,
            "Waits for multiple upstream nodes and merges results.",
        ));

        p.register(PaletteEntry::new(
            "control.loop",
            "Loop",
            NodeCategory::Control,
            "Repeats a subgraph until a condition is met.",
        ).with_default("max_iterations", "10"));

        p
    }

    pub fn register(&mut self, entry: PaletteEntry) {
        self.entries.push(entry);
    }

    /// Look up an entry by kind.
    pub fn get(&self, kind: &str) -> Option<&PaletteEntry> {
        self.entries.iter().find(|e| e.kind == kind)
    }

    /// Return entries filtered by category.
    pub fn by_category(&self, cat: &NodeCategory) -> Vec<&PaletteEntry> {
        self.entries.iter().filter(|e| &e.category == cat).collect()
    }

    /// Search entries by label substring (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<&PaletteEntry> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.label.to_lowercase().contains(&q) || e.kind.to_lowercase().contains(&q))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn default_palette_non_empty() {
        let p = Palette::default_palette();
        assert!(!p.is_empty());
    }

    #[test]
    fn palette_lookup_by_kind() {
        let p = Palette::default_palette();
        let e = p.get("agent.llm").expect("llm agent missing");
        assert_eq!(e.category, NodeCategory::Agent);
    }

    #[test]
    fn palette_search() {
        let p = Palette::default_palette();
        let results = p.search("verif");
        assert!(!results.is_empty());
        for r in &results {
            assert!(r.label.to_lowercase().contains("verif") || r.kind.contains("verif"));
        }
    }

    #[test]
    fn palette_by_category() {
        let p = Palette::default_palette();
        let agents = p.by_category(&NodeCategory::Agent);
        assert!(!agents.is_empty());
    }
}
