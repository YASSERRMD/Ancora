//! RAG faithfulness eval suite.
//!
//! Checks that generated answers are grounded in the retrieved context and do
//! not introduce hallucinated facts.

/// A retrieved context passage.
#[derive(Debug, Clone)]
pub struct ContextPassage {
    pub id: String,
    pub text: String,
}

impl ContextPassage {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        ContextPassage {
            id: id.into(),
            text: text.into(),
        }
    }
}

/// One RAG faithfulness eval case.
#[derive(Debug, Clone)]
pub struct RagCase {
    pub id: String,
    pub question: String,
    pub context: Vec<ContextPassage>,
    /// The candidate answer to judge.
    pub answer: String,
    /// True if the answer should be considered faithful.
    pub expected_faithful: bool,
}

impl RagCase {
    pub fn new(
        id: impl Into<String>,
        question: impl Into<String>,
        context: Vec<ContextPassage>,
        answer: impl Into<String>,
        expected_faithful: bool,
    ) -> Self {
        RagCase {
            id: id.into(),
            question: question.into(),
            context,
            answer: answer.into(),
            expected_faithful,
        }
    }
}

/// Simple offline faithfulness judge.
///
/// A claim is considered faithful if at least one key term from the answer
/// appears verbatim in the context. For production use this would be replaced
/// by a semantic entailment model.
pub struct LocalFaithfulnessJudge;

impl LocalFaithfulnessJudge {
    /// Returns true when the answer is judged faithful to the context.
    pub fn judge(&self, answer: &str, context: &[ContextPassage]) -> bool {
        let answer_lower = answer.to_lowercase();
        // Extract words longer than 4 chars as candidate key terms.
        let key_terms: Vec<&str> = answer_lower
            .split_whitespace()
            .filter(|w| w.len() > 4)
            .collect();

        if key_terms.is_empty() {
            return false;
        }

        let context_blob: String = context
            .iter()
            .map(|p| p.text.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");

        key_terms.iter().any(|term| context_blob.contains(term))
    }
}

/// Outcome of a RAG faithfulness eval.
#[derive(Debug, Clone, PartialEq)]
pub enum RagOutcome {
    /// Judge and expectation agree.
    Correct,
    /// Judge produced a different verdict than expected.
    Mismatch { judged_faithful: bool },
}

/// The full RAG faithfulness eval suite.
pub struct RagFaithfulnessSuite {
    pub cases: Vec<RagCase>,
    judge: LocalFaithfulnessJudge,
}

impl RagFaithfulnessSuite {
    pub fn new(cases: Vec<RagCase>) -> Self {
        RagFaithfulnessSuite {
            cases,
            judge: LocalFaithfulnessJudge,
        }
    }

    /// Build the default catalog.
    pub fn default_catalog() -> Self {
        let ctx_a = vec![
            ContextPassage::new("p1", "Rust was created by Mozilla Research."),
            ContextPassage::new("p2", "The first stable release of Rust was in 2015."),
        ];
        let ctx_b = vec![ContextPassage::new(
            "p3",
            "Python is a dynamically typed language.",
        )];

        Self::new(vec![
            RagCase::new(
                "rf-001",
                "Who created Rust?",
                ctx_a.clone(),
                "Rust was created by Mozilla Research.",
                true,
            ),
            RagCase::new(
                "rf-002",
                "Who created Rust?",
                ctx_a,
                "Rust was invented by Google in 2010.",
                false,
            ),
            RagCase::new(
                "rf-003",
                "What kind of language is Python?",
                ctx_b,
                "Python is dynamically typed.",
                true,
            ),
        ])
    }

    /// Evaluate a single case.
    pub fn evaluate(&self, case: &RagCase) -> RagOutcome {
        let judged = self.judge.judge(&case.answer, &case.context);
        if judged == case.expected_faithful {
            RagOutcome::Correct
        } else {
            RagOutcome::Mismatch {
                judged_faithful: judged,
            }
        }
    }

    /// Run all cases and return (passed, total).
    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == RagOutcome::Correct)
            .count();
        (passed, total)
    }
}
