//! Cost-efficiency eval suite.
//!
//! Evaluates whether an agent meets quality requirements while staying within
//! a token or cost budget.

/// Token usage of an agent response.
#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
}

impl TokenUsage {
    pub fn new(prompt_tokens: u64, completion_tokens: u64) -> Self {
        TokenUsage {
            prompt_tokens,
            completion_tokens,
        }
    }

    pub fn total(&self) -> u64 {
        self.prompt_tokens + self.completion_tokens
    }
}

/// One cost-efficiency eval case.
#[derive(Debug, Clone)]
pub struct CostCase {
    pub id: String,
    pub prompt: String,
    /// Simulated token usage produced by the agent.
    pub usage: TokenUsage,
    /// Maximum total tokens allowed for the task.
    pub token_budget: u64,
    /// Whether the agent's response is considered high quality.
    pub quality_pass: bool,
}

impl CostCase {
    pub fn new(
        id: impl Into<String>,
        prompt: impl Into<String>,
        usage: TokenUsage,
        token_budget: u64,
        quality_pass: bool,
    ) -> Self {
        CostCase {
            id: id.into(),
            prompt: prompt.into(),
            usage,
            token_budget,
            quality_pass,
        }
    }
}

/// Outcome of a cost-efficiency eval.
#[derive(Debug, Clone, PartialEq)]
pub enum CostOutcome {
    /// Within budget and quality passes.
    Efficient,
    /// Over budget.
    OverBudget { total: u64, budget: u64 },
    /// Within budget but quality failed.
    LowQuality,
}

/// The full cost-efficiency eval suite.
pub struct CostEfficiencySuite {
    pub cases: Vec<CostCase>,
}

impl CostEfficiencySuite {
    pub fn default_catalog() -> Self {
        CostEfficiencySuite {
            cases: vec![
                CostCase::new(
                    "ce-001",
                    "Translate 'hello' to French.",
                    TokenUsage::new(10, 5),
                    100,
                    true,
                ),
                CostCase::new(
                    "ce-002",
                    "Write a 500-word essay on Rust.",
                    TokenUsage::new(50, 400),
                    500,
                    true,
                ),
                CostCase::new("ce-003", "What is 2+2?", TokenUsage::new(5, 3), 50, true),
            ],
        }
    }

    pub fn evaluate(&self, case: &CostCase) -> CostOutcome {
        let total = case.usage.total();
        if total > case.token_budget {
            CostOutcome::OverBudget {
                total,
                budget: case.token_budget,
            }
        } else if !case.quality_pass {
            CostOutcome::LowQuality
        } else {
            CostOutcome::Efficient
        }
    }

    /// Returns (passed, total) where passed = Efficient outcomes.
    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == CostOutcome::Efficient)
            .count();
        (passed, total)
    }
}
