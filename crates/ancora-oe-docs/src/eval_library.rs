//! Eval library catalog: pre-built evaluation suites for common agent tasks.

/// Category of a pre-built eval suite.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalCategory {
    /// Tests factual accuracy.
    Factuality,
    /// Tests instruction following.
    InstructionFollowing,
    /// Tests safety and harm avoidance.
    Safety,
    /// Tests code generation quality.
    CodeGen,
    /// Tests summarisation quality.
    Summarisation,
    /// Custom domain-specific eval.
    Custom(String),
}

impl std::fmt::Display for EvalCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalCategory::Factuality => write!(f, "factuality"),
            EvalCategory::InstructionFollowing => write!(f, "instruction_following"),
            EvalCategory::Safety => write!(f, "safety"),
            EvalCategory::CodeGen => write!(f, "code_gen"),
            EvalCategory::Summarisation => write!(f, "summarisation"),
            EvalCategory::Custom(s) => write!(f, "custom:{s}"),
        }
    }
}

/// A catalog entry for a pre-built eval suite.
#[derive(Debug, Clone)]
pub struct EvalCatalogEntry {
    pub id: String,
    pub name: String,
    pub category: EvalCategory,
    pub sample_count: usize,
    pub grader_id: String,
}

impl EvalCatalogEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        category: EvalCategory,
        sample_count: usize,
        grader_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category,
            sample_count,
            grader_id: grader_id.into(),
        }
    }
}

/// The full eval library catalog.
#[derive(Debug, Default)]
pub struct EvalCatalog {
    entries: Vec<EvalCatalogEntry>,
}

impl EvalCatalog {
    pub fn add(&mut self, entry: EvalCatalogEntry) {
        self.entries.push(entry);
    }

    pub fn by_category(&self, category: &EvalCategory) -> Vec<&EvalCatalogEntry> {
        self.entries
            .iter()
            .filter(|e| &e.category == category)
            .collect()
    }

    pub fn total(&self) -> usize {
        self.entries.len()
    }

    /// Returns all entries sorted by name.
    pub fn sorted_by_name(&self) -> Vec<&EvalCatalogEntry> {
        let mut refs: Vec<&EvalCatalogEntry> = self.entries.iter().collect();
        refs.sort_by(|a, b| a.name.cmp(&b.name));
        refs
    }
}
