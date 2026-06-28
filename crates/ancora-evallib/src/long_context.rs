//! Long-context eval suite.
//!
//! Verifies that an agent can retrieve specific facts from documents that
//! would span many tokens (simulated here with repeated padding text).

/// A long document represented as a vector of sections.
#[derive(Debug, Clone)]
pub struct LongDocument {
    pub sections: Vec<String>,
}

impl LongDocument {
    pub fn new(sections: Vec<String>) -> Self {
        LongDocument { sections }
    }

    /// Concatenate all sections into a single string.
    pub fn full_text(&self) -> String {
        self.sections.join("\n\n")
    }
}

/// One long-context eval case.
#[derive(Debug, Clone)]
pub struct LongContextCase {
    pub id: String,
    pub document: LongDocument,
    pub question: String,
    /// Exact substring that must appear in the extracted answer.
    pub expected_fragment: String,
}

impl LongContextCase {
    pub fn new(
        id: impl Into<String>,
        document: LongDocument,
        question: impl Into<String>,
        expected_fragment: impl Into<String>,
    ) -> Self {
        LongContextCase {
            id: id.into(),
            document,
            question: question.into(),
            expected_fragment: expected_fragment.into(),
        }
    }
}

/// Outcome of a long-context eval.
#[derive(Debug, Clone, PartialEq)]
pub enum LongContextOutcome {
    Correct,
    FragmentMissing,
}

/// Offline long-context retriever: scans sections for the question keyword.
pub struct LocalContextRetriever;

impl LocalContextRetriever {
    /// Extract the first section that contains a keyword from the question.
    pub fn extract(&self, doc: &LongDocument, question: &str) -> Option<String> {
        let keywords: Vec<&str> = question.split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();
        for section in &doc.sections {
            let section_lower = section.to_lowercase();
            if keywords.iter().any(|kw| section_lower.contains(&kw.to_lowercase())) {
                return Some(section.clone());
            }
        }
        None
    }
}

/// The full long-context eval suite.
pub struct LongContextSuite {
    pub cases: Vec<LongContextCase>,
}

impl LongContextSuite {
    /// Build the default catalog with simulated long documents.
    pub fn default_catalog() -> Self {
        let filler: Vec<String> = (0..20)
            .map(|i| format!("Section {}: This is filler content to simulate a large document.", i))
            .collect();

        let mut sections_a = filler.clone();
        sections_a.push("Section target: The founder of Ancora is YASSERRMD.".into());
        sections_a.extend(filler.clone());

        let mut sections_b = filler.clone();
        sections_b.push("Section target: The license for Ancora is Apache-2.0.".into());
        sections_b.extend(filler);

        LongContextSuite {
            cases: vec![
                LongContextCase::new(
                    "lc-001",
                    LongDocument::new(sections_a),
                    "Who is the founder of Ancora?",
                    "YASSERRMD",
                ),
                LongContextCase::new(
                    "lc-002",
                    LongDocument::new(sections_b),
                    "What license does Ancora use?",
                    "Apache-2.0",
                ),
            ],
        }
    }

    pub fn evaluate(&self, case: &LongContextCase) -> LongContextOutcome {
        let retriever = LocalContextRetriever;
        match retriever.extract(&case.document, &case.question) {
            Some(section) if section.contains(&case.expected_fragment) => {
                LongContextOutcome::Correct
            }
            _ => LongContextOutcome::FragmentMissing,
        }
    }

    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == LongContextOutcome::Correct)
            .count();
        (passed, total)
    }
}
