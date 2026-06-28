use crate::lifecycle::{BackgroundRun, RunState};
use crate::progress::RunProgress;

/// Snapshot of dashboard data for a background run.
#[derive(Debug, Clone)]
pub struct RunDashboard {
    pub run_id: String,
    pub state_label: String,
    pub pct_complete: f64,
    pub effects_count: usize,
}

impl RunDashboard {
    pub fn from(run: &BackgroundRun, progress: Option<&RunProgress>) -> Self {
        let state_label = match &run.state {
            RunState::Created => "created",
            RunState::Running => "running",
            RunState::Sleeping { .. } => "sleeping",
            RunState::Woken => "woken",
            RunState::Completed => "completed",
            RunState::Failed(_) => "failed",
        };
        RunDashboard {
            run_id: run.run_id.clone(),
            state_label: state_label.to_string(),
            pct_complete: progress.map(|p| p.pct_complete()).unwrap_or(0.0),
            effects_count: run.effects_applied.len(),
        }
    }
}
