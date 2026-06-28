//! Multi-agent coordination eval suite.
//!
//! Evaluates whether a set of agents correctly partitions a task and assembles
//! the sub-results into a coherent final answer.

/// A sub-task assigned to a single agent.
#[derive(Debug, Clone)]
pub struct SubTask {
    pub id: String,
    pub description: String,
    pub assigned_agent: String,
}

impl SubTask {
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        assigned_agent: impl Into<String>,
    ) -> Self {
        SubTask {
            id: id.into(),
            description: description.into(),
            assigned_agent: assigned_agent.into(),
        }
    }
}

/// The result produced by an agent for its sub-task.
#[derive(Debug, Clone)]
pub struct AgentResult {
    pub agent: String,
    pub output: String,
}

impl AgentResult {
    pub fn new(agent: impl Into<String>, output: impl Into<String>) -> Self {
        AgentResult {
            agent: agent.into(),
            output: output.into(),
        }
    }
}

/// One coordination eval case.
#[derive(Debug, Clone)]
pub struct CoordCase {
    pub id: String,
    pub master_task: String,
    pub sub_tasks: Vec<SubTask>,
    /// Simulated outputs from each agent.
    pub agent_results: Vec<AgentResult>,
    /// Keywords that must appear in the assembled final answer.
    pub required_keywords: Vec<String>,
}

impl CoordCase {
    pub fn new(
        id: impl Into<String>,
        master_task: impl Into<String>,
        sub_tasks: Vec<SubTask>,
        agent_results: Vec<AgentResult>,
        required_keywords: Vec<String>,
    ) -> Self {
        CoordCase {
            id: id.into(),
            master_task: master_task.into(),
            sub_tasks,
            agent_results,
            required_keywords,
        }
    }
}

/// Offline coordinator: concatenates agent results.
pub struct LocalCoordinator;

impl LocalCoordinator {
    pub fn assemble(&self, results: &[AgentResult]) -> String {
        results
            .iter()
            .map(|r| r.output.as_str())
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

/// Outcome of a coordination eval.
#[derive(Debug, Clone, PartialEq)]
pub enum CoordOutcome {
    Correct,
    MissingKeywords(Vec<String>),
}

/// The full coordination eval suite.
pub struct CoordinationSuite {
    pub cases: Vec<CoordCase>,
}

impl CoordinationSuite {
    pub fn default_catalog() -> Self {
        CoordinationSuite {
            cases: vec![CoordCase::new(
                "co-001",
                "Summarize the weather in Paris and London.",
                vec![
                    SubTask::new("s1", "Get Paris weather", "weather-agent-paris"),
                    SubTask::new("s2", "Get London weather", "weather-agent-london"),
                ],
                vec![
                    AgentResult::new("weather-agent-paris", "Paris: sunny 25C"),
                    AgentResult::new("weather-agent-london", "London: cloudy 18C"),
                ],
                vec!["Paris".into(), "London".into()],
            )],
        }
    }

    pub fn evaluate(&self, case: &CoordCase) -> CoordOutcome {
        let coord = LocalCoordinator;
        let assembled = coord.assemble(&case.agent_results);
        let assembled_lower = assembled.to_lowercase();
        let missing: Vec<String> = case
            .required_keywords
            .iter()
            .filter(|kw| !assembled_lower.contains(&kw.to_lowercase()))
            .cloned()
            .collect();
        if missing.is_empty() {
            CoordOutcome::Correct
        } else {
            CoordOutcome::MissingKeywords(missing)
        }
    }

    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == CoordOutcome::Correct)
            .count();
        (passed, total)
    }
}
