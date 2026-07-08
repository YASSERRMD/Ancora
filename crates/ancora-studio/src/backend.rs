use crate::cost_view::{CostBreakdown, StepCost};
use crate::eval_view::{EvalOutcome, EvalResult, EvalView};
use crate::feedback_view::{FeedbackEntry, FeedbackRating, FeedbackView};
/// Local backend - offline data access layer for the studio.
use crate::run_list::{RunList, RunStatus, RunSummary};
use crate::timeline::{StepKind, Timeline, TimelineStep};

#[derive(Debug)]
pub enum BackendError {
    RunNotFound(String),
    StorageError(String),
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::RunNotFound(id) => write!(f, "run not found: {}", id),
            BackendError::StorageError(msg) => write!(f, "storage error: {}", msg),
        }
    }
}

/// Trait for backends to implement. Enables in-memory or file-backed variants.
pub trait StudioBackend {
    fn list_runs(&self) -> Result<RunList, BackendError>;
    fn get_timeline(&self, run_id: &str) -> Result<Timeline, BackendError>;
    fn get_evals(&self, run_id: &str) -> Result<EvalView, BackendError>;
    fn get_cost_breakdown(&self, run_id: &str) -> Result<CostBreakdown, BackendError>;
    fn get_feedback(&self, run_id: &str) -> Result<FeedbackView, BackendError>;
}

/// In-memory backend for offline use and testing.
pub struct InMemoryBackend {
    runs: Vec<RunSummary>,
    timelines: std::collections::HashMap<String, Vec<TimelineStep>>,
    evals: std::collections::HashMap<String, Vec<EvalResult>>,
    costs: std::collections::HashMap<String, Vec<StepCost>>,
    feedback: std::collections::HashMap<String, Vec<FeedbackEntry>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            timelines: Default::default(),
            evals: Default::default(),
            costs: Default::default(),
            feedback: Default::default(),
        }
    }

    pub fn add_run(&mut self, run: RunSummary) {
        self.runs.push(run);
    }

    pub fn add_timeline_steps(&mut self, run_id: impl Into<String>, steps: Vec<TimelineStep>) {
        self.timelines.insert(run_id.into(), steps);
    }

    pub fn add_eval_results(&mut self, run_id: impl Into<String>, results: Vec<EvalResult>) {
        self.evals.insert(run_id.into(), results);
    }

    pub fn add_cost_steps(&mut self, run_id: impl Into<String>, steps: Vec<StepCost>) {
        self.costs.insert(run_id.into(), steps);
    }

    pub fn add_feedback_entries(&mut self, run_id: impl Into<String>, entries: Vec<FeedbackEntry>) {
        self.feedback.insert(run_id.into(), entries);
    }
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl StudioBackend for InMemoryBackend {
    fn list_runs(&self) -> Result<RunList, BackendError> {
        Ok(RunList::new(self.runs.clone()))
    }

    fn get_timeline(&self, run_id: &str) -> Result<Timeline, BackendError> {
        let steps = self.timelines.get(run_id).cloned().unwrap_or_default();
        Ok(Timeline::new(run_id, steps))
    }

    fn get_evals(&self, run_id: &str) -> Result<EvalView, BackendError> {
        let results = self.evals.get(run_id).cloned().unwrap_or_default();
        Ok(EvalView::new(results))
    }

    fn get_cost_breakdown(&self, run_id: &str) -> Result<CostBreakdown, BackendError> {
        let steps = self.costs.get(run_id).cloned().unwrap_or_default();
        Ok(CostBreakdown::new(run_id, steps))
    }

    fn get_feedback(&self, run_id: &str) -> Result<FeedbackView, BackendError> {
        let entries = self.feedback.get(run_id).cloned().unwrap_or_default();
        Ok(FeedbackView::new(entries))
    }
}

/// Build a demo backend populated with sample data.
pub fn demo_backend() -> InMemoryBackend {
    let mut backend = InMemoryBackend::new();

    backend.add_run(RunSummary {
        id: "demo-r1".into(),
        label: "Demo run 1".into(),
        status: RunStatus::Completed,
        started_at: 1_700_000_000,
        duration_ms: Some(1200),
        total_cost_usd: Some(0.005),
        step_count: 2,
        tags: vec!["demo".into()],
    });

    backend.add_timeline_steps(
        "demo-r1",
        vec![
            TimelineStep {
                index: 0,
                kind: StepKind::LlmCall,
                label: "initial call".into(),
                start_ms: 0,
                end_ms: 800,
                tokens_in: Some(100),
                tokens_out: Some(60),
                cost_usd: Some(0.003),
                redacted: false,
            },
            TimelineStep {
                index: 1,
                kind: StepKind::ToolCall,
                label: "fetch data".into(),
                start_ms: 800,
                end_ms: 1200,
                tokens_in: None,
                tokens_out: None,
                cost_usd: None,
                redacted: false,
            },
        ],
    );

    backend.add_eval_results(
        "demo-r1",
        vec![EvalResult {
            eval_name: "coherence".into(),
            run_id: "demo-r1".into(),
            outcome: EvalOutcome::Pass,
            reason: None,
            latency_ms: 50,
        }],
    );

    backend.add_cost_steps(
        "demo-r1",
        vec![StepCost {
            step_index: 0,
            model: "gpt-4".into(),
            tokens_in: 100,
            tokens_out: 60,
            cost_usd: 0.003,
        }],
    );

    backend.add_feedback_entries(
        "demo-r1",
        vec![FeedbackEntry {
            id: "fb1".into(),
            run_id: "demo-r1".into(),
            step_index: None,
            rating: FeedbackRating::ThumbsUp,
            comment: None,
            reviewer: "demo-user".into(),
            created_at: 1_700_000_100,
            tags: vec![],
        }],
    );

    backend
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_backend_list_runs() {
        let backend = demo_backend();
        let runs = backend.list_runs().unwrap();
        assert!(!runs.all().is_empty());
    }

    #[test]
    fn test_demo_backend_timeline() {
        let backend = demo_backend();
        let tl = backend.get_timeline("demo-r1").unwrap();
        assert_eq!(tl.steps().len(), 2);
    }

    #[test]
    fn test_demo_backend_evals() {
        let backend = demo_backend();
        let ev = backend.get_evals("demo-r1").unwrap();
        assert_eq!(ev.pass_rate(), 1.0);
    }

    #[test]
    fn test_demo_backend_costs() {
        let backend = demo_backend();
        let costs = backend.get_cost_breakdown("demo-r1").unwrap();
        assert!((costs.total_cost_usd() - 0.003).abs() < 1e-9);
    }
}
