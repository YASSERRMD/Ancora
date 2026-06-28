use crate::grader::{Grader, Score};

/// A single tool call recorded in a trajectory.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub tool: String,
    pub args: std::collections::HashMap<String, String>,
}

impl ToolCall {
    pub fn new(tool: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            args: std::collections::HashMap::new(),
        }
    }

    pub fn with_arg(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(key.into(), value.into());
        self
    }
}

/// A trajectory is an ordered sequence of tool calls.
#[derive(Debug, Clone, Default)]
pub struct Trajectory {
    pub calls: Vec<ToolCall>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, call: ToolCall) -> Self {
        self.calls.push(call);
        self
    }

    /// Parse a trajectory from a simple text format:
    /// One tool name per line, e.g. "search\nread_file\nwrite_file"
    pub fn from_lines(text: &str) -> Self {
        let calls = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|tool| ToolCall::new(tool))
            .collect();
        Self { calls }
    }
}

/// Grader that compares a candidate trajectory against an expected trajectory.
///
/// The score is the fraction of expected tool names that appear in the
/// candidate trajectory in the correct relative order.
#[derive(Debug, Clone, Default)]
pub struct TrajectoryGrader {
    /// When true, require exact position matching; otherwise require subsequence ordering.
    pub strict: bool,
}

impl TrajectoryGrader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }

    fn is_subsequence(needle: &[String], haystack: &[String]) -> bool {
        let mut ni = 0;
        for h in haystack {
            if ni < needle.len() && &needle[ni] == h {
                ni += 1;
            }
        }
        ni == needle.len()
    }
}

impl Grader for TrajectoryGrader {
    fn grade(&self, candidate: &str, expected: &str) -> Score {
        let cand_traj = Trajectory::from_lines(candidate);
        let exp_traj = Trajectory::from_lines(expected);

        if exp_traj.calls.is_empty() {
            return Score::new(1.0).with_rationale("No expected calls");
        }

        let cand_tools: Vec<String> = cand_traj.calls.iter().map(|c| c.tool.clone()).collect();
        let exp_tools: Vec<String> = exp_traj.calls.iter().map(|c| c.tool.clone()).collect();

        if self.strict {
            let matches: usize = cand_tools
                .iter()
                .zip(exp_tools.iter())
                .filter(|(c, e)| c == e)
                .count();
            let len = exp_tools.len().max(cand_tools.len());
            let value = matches as f64 / len as f64;
            Score::new(value.clamp(0.0, 1.0))
                .with_rationale(format!("Strict positional match: {}/{}", matches, len))
        } else {
            if Self::is_subsequence(&exp_tools, &cand_tools) {
                Score::new(1.0).with_rationale("Expected tools appear in order")
            } else {
                let present: usize = exp_tools
                    .iter()
                    .filter(|e| cand_tools.contains(e))
                    .count();
                let value = present as f64 / exp_tools.len() as f64;
                Score::new(value.clamp(0.0, 1.0))
                    .with_rationale(format!("Partial order match: {}/{}", present, exp_tools.len()))
            }
        }
    }

    fn name(&self) -> &str {
        "trajectory"
    }
}
