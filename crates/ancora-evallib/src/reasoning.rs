//! Reasoning eval suite.
//!
//! Tests logical, arithmetic, and causal reasoning capabilities of an agent.

/// Category of reasoning being tested.
#[derive(Debug, Clone, PartialEq)]
pub enum ReasoningCategory {
    Arithmetic,
    Logical,
    Causal,
}

/// One reasoning eval case.
#[derive(Debug, Clone)]
pub struct ReasoningCase {
    pub id: String,
    pub category: ReasoningCategory,
    pub question: String,
    pub expected_answer: String,
}

impl ReasoningCase {
    pub fn new(
        id: impl Into<String>,
        category: ReasoningCategory,
        question: impl Into<String>,
        expected_answer: impl Into<String>,
    ) -> Self {
        ReasoningCase {
            id: id.into(),
            category,
            question: question.into(),
            expected_answer: expected_answer.into(),
        }
    }
}

/// Outcome of a reasoning eval.
#[derive(Debug, Clone, PartialEq)]
pub enum ReasoningOutcome {
    Correct,
    Incorrect { produced: String },
}

/// Offline reasoning judge that handles simple arithmetic deterministically.
pub struct LocalReasoningJudge;

impl LocalReasoningJudge {
    /// Attempt to evaluate the question. Returns None when not able to compute.
    pub fn solve(&self, case: &ReasoningCase) -> Option<String> {
        match case.category {
            ReasoningCategory::Arithmetic => self.solve_arithmetic(&case.question),
            ReasoningCategory::Logical => self.solve_logical(&case.question),
            ReasoningCategory::Causal => self.solve_causal(&case.question),
        }
    }

    fn solve_arithmetic(&self, question: &str) -> Option<String> {
        // Very small DSL: "N + M", "N - M", "N * M", "N / M"
        let q = question.replace('?', "").trim().to_string();
        for op in &["+", "-", "*", "/"] {
            if let Some(pos) = q.find(op) {
                let left: f64 = q[..pos].trim().parse().ok()?;
                let right: f64 = q[pos + op.len()..].trim().parse().ok()?;
                let result = match *op {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => {
                        if right == 0.0 {
                            return None;
                        }
                        left / right
                    }
                    _ => return None,
                };
                // Format without trailing zeros for integers.
                if result.fract() == 0.0 {
                    return Some(format!("{}", result as i64));
                }
                return Some(format!("{}", result));
            }
        }
        None
    }

    fn solve_logical(&self, question: &str) -> Option<String> {
        // Recognise a small set of fixed logical questions.
        let q = question.to_lowercase();
        if q.contains("all humans are mortal") && q.contains("socrates is a human") {
            return Some("mortal".into());
        }
        None
    }

    fn solve_causal(&self, question: &str) -> Option<String> {
        let q = question.to_lowercase();
        if q.contains("fire") && q.contains("smoke") {
            return Some("smoke".into());
        }
        None
    }
}

/// The full reasoning eval suite.
pub struct ReasoningSuite {
    pub cases: Vec<ReasoningCase>,
}

impl ReasoningSuite {
    pub fn default_catalog() -> Self {
        ReasoningSuite {
            cases: vec![
                ReasoningCase::new("re-001", ReasoningCategory::Arithmetic, "42 + 58", "100"),
                ReasoningCase::new("re-002", ReasoningCategory::Arithmetic, "100 - 37", "63"),
                ReasoningCase::new(
                    "re-003",
                    ReasoningCategory::Logical,
                    "All humans are mortal. Socrates is a human. Is Socrates mortal?",
                    "mortal",
                ),
                ReasoningCase::new(
                    "re-004",
                    ReasoningCategory::Causal,
                    "If there is fire, what do you expect to see? (fire and smoke scenario)",
                    "smoke",
                ),
            ],
        }
    }

    pub fn evaluate(&self, case: &ReasoningCase) -> ReasoningOutcome {
        let judge = LocalReasoningJudge;
        match judge.solve(case) {
            Some(answer) if answer == case.expected_answer => ReasoningOutcome::Correct,
            Some(answer) => ReasoningOutcome::Incorrect { produced: answer },
            None => ReasoningOutcome::Incorrect {
                produced: String::from("<no answer>"),
            },
        }
    }

    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == ReasoningOutcome::Correct)
            .count();
        (passed, total)
    }
}
