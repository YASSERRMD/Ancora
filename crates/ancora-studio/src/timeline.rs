//! Timeline view - visualises each step of a run as a horizontal swimlane.

#[derive(Debug, Clone, PartialEq)]
pub enum StepKind {
    LlmCall,
    ToolCall,
    Checkpoint,
    UserMessage,
    AssistantMessage,
    Error,
}

#[derive(Debug, Clone)]
pub struct TimelineStep {
    pub index: usize,
    pub kind: StepKind,
    pub label: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub tokens_in: Option<u32>,
    pub tokens_out: Option<u32>,
    pub cost_usd: Option<f64>,
    pub redacted: bool,
}

impl TimelineStep {
    pub fn duration_ms(&self) -> u64 {
        self.end_ms.saturating_sub(self.start_ms)
    }

    pub fn is_visible(&self) -> bool {
        !self.redacted
    }
}

pub struct Timeline {
    pub run_id: String,
    steps: Vec<TimelineStep>,
}

impl Timeline {
    pub fn new(run_id: impl Into<String>, steps: Vec<TimelineStep>) -> Self {
        Self {
            run_id: run_id.into(),
            steps,
        }
    }

    pub fn steps(&self) -> &[TimelineStep] {
        &self.steps
    }

    pub fn visible_steps(&self) -> Vec<&TimelineStep> {
        self.steps.iter().filter(|s| s.is_visible()).collect()
    }

    pub fn total_duration_ms(&self) -> u64 {
        let start = self.steps.iter().map(|s| s.start_ms).min().unwrap_or(0);
        let end = self.steps.iter().map(|s| s.end_ms).max().unwrap_or(0);
        end.saturating_sub(start)
    }

    pub fn total_cost_usd(&self) -> f64 {
        self.steps.iter().filter_map(|s| s.cost_usd).sum()
    }

    pub fn step_at(&self, index: usize) -> Option<&TimelineStep> {
        self.steps.iter().find(|s| s.index == index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_timeline() -> Timeline {
        Timeline::new(
            "r1",
            vec![
                TimelineStep {
                    index: 0,
                    kind: StepKind::LlmCall,
                    label: "call gpt".into(),
                    start_ms: 0,
                    end_ms: 100,
                    tokens_in: Some(50),
                    tokens_out: Some(30),
                    cost_usd: Some(0.002),
                    redacted: false,
                },
                TimelineStep {
                    index: 1,
                    kind: StepKind::ToolCall,
                    label: "search".into(),
                    start_ms: 100,
                    end_ms: 200,
                    tokens_in: None,
                    tokens_out: None,
                    cost_usd: None,
                    redacted: true,
                },
            ],
        )
    }

    #[test]
    fn test_total_duration() {
        let tl = sample_timeline();
        assert_eq!(tl.total_duration_ms(), 200);
    }

    #[test]
    fn test_visible_steps() {
        let tl = sample_timeline();
        let vis = tl.visible_steps();
        assert_eq!(vis.len(), 1);
        assert_eq!(vis[0].index, 0);
    }

    #[test]
    fn test_total_cost() {
        let tl = sample_timeline();
        assert!((tl.total_cost_usd() - 0.002).abs() < 1e-9);
    }
}
