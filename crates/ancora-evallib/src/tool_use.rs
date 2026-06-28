//! Tool-use eval suite.
//!
//! Tests an agent's ability to select and invoke the correct tool given a task
//! description, and to forward the tool output back to a response.

/// A single tool available to the agent during an eval.
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

impl Tool {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Tool {
            name: name.into(),
            description: description.into(),
        }
    }
}

/// One case in the tool-use suite.
#[derive(Debug, Clone)]
pub struct ToolUseCase {
    pub id: String,
    pub prompt: String,
    pub available_tools: Vec<Tool>,
    pub expected_tool: String,
}

impl ToolUseCase {
    pub fn new(
        id: impl Into<String>,
        prompt: impl Into<String>,
        available_tools: Vec<Tool>,
        expected_tool: impl Into<String>,
    ) -> Self {
        ToolUseCase {
            id: id.into(),
            prompt: prompt.into(),
            available_tools,
            expected_tool: expected_tool.into(),
        }
    }
}

/// Outcome of running a tool-use eval case.
#[derive(Debug, Clone, PartialEq)]
pub enum ToolUseOutcome {
    Correct,
    WrongTool { selected: String },
    NoToolSelected,
}

/// Simulated agent response used for offline evaluation.
pub struct SimulatedToolUseAgent;

impl SimulatedToolUseAgent {
    /// Returns the name of the tool the simulated agent would select.
    /// The logic is a simple keyword match used for deterministic offline testing.
    pub fn select_tool<'a>(&self, prompt: &str, tools: &'a [Tool]) -> Option<&'a Tool> {
        let prompt_lower = prompt.to_lowercase();
        tools.iter().find(|t| {
            let name_lower = t.name.to_lowercase();
            prompt_lower.contains(&name_lower)
                || prompt_lower.contains(&t.description.to_lowercase())
        })
    }
}

/// The full tool-use eval suite.
pub struct ToolUseSuite {
    pub cases: Vec<ToolUseCase>,
}

impl ToolUseSuite {
    /// Build the default catalog of tool-use eval cases.
    pub fn default_catalog() -> Self {
        let search_tool = Tool::new("search", "search the web for information");
        let calc_tool = Tool::new("calculator", "perform arithmetic calculations");
        let file_tool = Tool::new("file_read", "read the contents of a file");

        ToolUseSuite {
            cases: vec![
                ToolUseCase::new(
                    "tu-001",
                    "Please search for the latest news on Rust.",
                    vec![search_tool.clone(), calc_tool.clone()],
                    "search",
                ),
                ToolUseCase::new(
                    "tu-002",
                    "What is 42 * 17? Use the calculator.",
                    vec![search_tool.clone(), calc_tool.clone()],
                    "calculator",
                ),
                ToolUseCase::new(
                    "tu-003",
                    "Read the contents of config.toml using file_read.",
                    vec![file_tool.clone(), search_tool.clone()],
                    "file_read",
                ),
            ],
        }
    }

    /// Evaluate a single case and return the outcome.
    pub fn evaluate(&self, case: &ToolUseCase) -> ToolUseOutcome {
        let agent = SimulatedToolUseAgent;
        match agent.select_tool(&case.prompt, &case.available_tools) {
            None => ToolUseOutcome::NoToolSelected,
            Some(tool) if tool.name == case.expected_tool => ToolUseOutcome::Correct,
            Some(tool) => ToolUseOutcome::WrongTool {
                selected: tool.name.clone(),
            },
        }
    }

    /// Run all cases and return (passed, total).
    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == ToolUseOutcome::Correct)
            .count();
        (passed, total)
    }
}
