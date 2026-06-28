/// trace_overlay - Trace overlay: renders run step results on top of the graph.

use crate::runner::{RunResult, RunStep, StepStatus};
use crate::scaffold::{Id, Position};
use std::collections::HashMap;

/// Visual state for a single node in the overlay.
#[derive(Debug, Clone)]
pub struct NodeOverlay {
    pub node_id: Id,
    pub status: OverlayStatus,
    /// Duration badge text (e.g. "10ms").
    pub duration_label: String,
    /// Short excerpt of the node output (truncated).
    pub output_excerpt: Option<String>,
    /// Error message if the step failed.
    pub error_message: Option<String>,
    /// Position of the progress indicator relative to the node's top-left corner.
    pub indicator_offset: Position,
}

/// Visual status mapped from StepStatus for rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayStatus {
    Idle,
    Pending,
    Running,
    Succeeded,
    Failed,
    Skipped,
}

impl From<&StepStatus> for OverlayStatus {
    fn from(s: &StepStatus) -> Self {
        match s {
            StepStatus::Pending => OverlayStatus::Pending,
            StepStatus::Running => OverlayStatus::Running,
            StepStatus::Succeeded => OverlayStatus::Succeeded,
            StepStatus::Failed => OverlayStatus::Failed,
            StepStatus::Skipped => OverlayStatus::Skipped,
        }
    }
}

impl OverlayStatus {
    /// Return a display colour name (for the UI renderer to map to actual colours).
    pub fn colour(&self) -> &'static str {
        match self {
            OverlayStatus::Idle => "grey",
            OverlayStatus::Pending => "blue",
            OverlayStatus::Running => "yellow",
            OverlayStatus::Succeeded => "green",
            OverlayStatus::Failed => "red",
            OverlayStatus::Skipped => "grey",
        }
    }
}

/// An edge overlay indicates data flowing along an edge.
#[derive(Debug, Clone)]
pub struct EdgeOverlay {
    pub edge_id: Id,
    pub active: bool,
    pub colour: String,
}

/// The full trace overlay for a completed (or in-progress) run.
#[derive(Debug, Clone, Default)]
pub struct TraceOverlay {
    pub node_overlays: HashMap<String, NodeOverlay>,
    pub edge_overlays: HashMap<String, EdgeOverlay>,
    pub highlighted_step: Option<usize>,
}

impl TraceOverlay {
    pub fn new() -> Self {
        TraceOverlay::default()
    }

    /// Build an overlay from a completed RunResult.
    pub fn from_run_result(result: &RunResult) -> Self {
        let mut overlay = TraceOverlay::new();

        for step in &result.steps {
            let node_overlay = node_overlay_from_step(step);
            overlay.node_overlays.insert(step.node_id.clone(), node_overlay);
        }

        overlay
    }

    /// Highlight a specific step (e.g. user clicked on it in the trace panel).
    pub fn highlight_step(&mut self, step_index: Option<usize>) {
        self.highlighted_step = step_index;
        // Update node overlays: dim all except highlighted.
        if let Some(idx) = step_index {
            // In a real renderer we would dim all overlays except the one at idx.
            let _ = idx;
        }
    }

    /// Return the overlay for a given node ID, if present.
    pub fn node_overlay(&self, node_id: &str) -> Option<&NodeOverlay> {
        self.node_overlays.get(node_id)
    }

    /// Clear all overlay data (e.g. when starting a new run).
    pub fn clear(&mut self) {
        self.node_overlays.clear();
        self.edge_overlays.clear();
        self.highlighted_step = None;
    }

    /// Return true if the overlay has any data.
    pub fn is_active(&self) -> bool {
        !self.node_overlays.is_empty()
    }

    /// Return all failed node IDs.
    pub fn failed_nodes(&self) -> Vec<&str> {
        self.node_overlays
            .iter()
            .filter(|(_, ov)| ov.status == OverlayStatus::Failed)
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Return all succeeded node IDs.
    pub fn succeeded_nodes(&self) -> Vec<&str> {
        self.node_overlays
            .iter()
            .filter(|(_, ov)| ov.status == OverlayStatus::Succeeded)
            .map(|(id, _)| id.as_str())
            .collect()
    }
}

fn node_overlay_from_step(step: &RunStep) -> NodeOverlay {
    let duration_label = format!("{}ms", step.duration_ms);
    let output_excerpt = step.output.as_ref().map(|o| truncate(o, 80));

    NodeOverlay {
        node_id: Id::new(step.node_id.clone()),
        status: OverlayStatus::from(&step.status),
        duration_label,
        output_excerpt,
        error_message: step.error.clone(),
        indicator_offset: Position::new(0.0, -16.0),
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars - 3).collect();
        format!("{}...", truncated)
    }
}

/// A streaming overlay that updates as steps complete (e.g. live runs).
#[derive(Debug, Default, Clone)]
pub struct LiveTraceOverlay {
    pub inner: TraceOverlay,
    pub current_step_index: Option<usize>,
}

impl LiveTraceOverlay {
    pub fn new() -> Self {
        LiveTraceOverlay::default()
    }

    /// Apply a single completed step to the overlay.
    pub fn apply_step(&mut self, step: &RunStep) {
        let overlay = node_overlay_from_step(step);
        self.inner.node_overlays.insert(step.node_id.clone(), overlay);
        self.current_step_index = Some(step.step_index);
    }

    /// Mark the current running node (before a step completes).
    pub fn mark_running(&mut self, node_id: &str) {
        let overlay = NodeOverlay {
            node_id: Id::new(node_id),
            status: OverlayStatus::Running,
            duration_label: "-".into(),
            output_excerpt: None,
            error_message: None,
            indicator_offset: Position::new(0.0, -16.0),
        };
        self.inner.node_overlays.insert(node_id.to_string(), overlay);
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::runner::{RunResult, RunStatus, RunStep, StepStatus};
    use std::collections::HashMap;

    fn make_run_result() -> RunResult {
        RunResult {
            run_id: "run_0001".into(),
            spec_name: "test".into(),
            status: RunStatus::Completed,
            steps: vec![
                RunStep {
                    step_index: 0,
                    node_id: "n1".into(),
                    node_kind: "agent.llm".into(),
                    status: StepStatus::Succeeded,
                    output: Some("hello".into()),
                    error: None,
                    duration_ms: 10,
                },
                RunStep {
                    step_index: 1,
                    node_id: "n2".into(),
                    node_kind: "verifier.json_schema".into(),
                    status: StepStatus::Failed,
                    output: None,
                    error: Some("schema mismatch".into()),
                    duration_ms: 5,
                },
            ],
            total_duration_ms: 15,
            outputs: HashMap::new(),
        }
    }

    #[test]
    fn overlay_from_run_result() {
        let result = make_run_result();
        let overlay = TraceOverlay::from_run_result(&result);
        assert!(overlay.is_active());
        assert_eq!(overlay.node_overlays.len(), 2);
    }

    #[test]
    fn overlay_status_colours() {
        assert_eq!(OverlayStatus::Succeeded.colour(), "green");
        assert_eq!(OverlayStatus::Failed.colour(), "red");
        assert_eq!(OverlayStatus::Running.colour(), "yellow");
    }

    #[test]
    fn overlay_failed_nodes() {
        let result = make_run_result();
        let overlay = TraceOverlay::from_run_result(&result);
        let failed = overlay.failed_nodes();
        assert_eq!(failed.len(), 1);
        assert!(failed.contains(&"n2"));
    }

    #[test]
    fn overlay_succeeded_nodes() {
        let result = make_run_result();
        let overlay = TraceOverlay::from_run_result(&result);
        let succeeded = overlay.succeeded_nodes();
        assert_eq!(succeeded.len(), 1);
        assert!(succeeded.contains(&"n1"));
    }

    #[test]
    fn live_overlay_mark_running() {
        let mut live = LiveTraceOverlay::new();
        live.mark_running("n1");
        assert_eq!(
            live.inner.node_overlay("n1").unwrap().status,
            OverlayStatus::Running
        );
    }

    #[test]
    fn truncate_long_string() {
        let s: String = "x".repeat(200);
        let t = truncate(&s, 80);
        assert!(t.ends_with("..."));
        assert!(t.len() <= 80);
    }
}
