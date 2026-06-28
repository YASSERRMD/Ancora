/// Studio scaffold - top-level application state and routing.

#[derive(Debug, Clone, PartialEq)]
pub enum StudioView {
    RunList,
    Timeline { run_id: String },
    Inspector { run_id: String, step: usize },
    TraceTree { run_id: String },
    Replay { run_id: String },
    Diff { run_id_a: String, run_id_b: String },
    EvalView { run_id: String },
    CostView { run_id: String },
    DriftView,
    FeedbackView { run_id: String },
}

#[derive(Debug, Clone)]
pub struct StudioState {
    pub current_view: StudioView,
    pub status_message: Option<String>,
}

impl StudioState {
    pub fn new() -> Self {
        Self {
            current_view: StudioView::RunList,
            status_message: None,
        }
    }

    pub fn navigate(&mut self, view: StudioView) {
        self.current_view = view;
        self.status_message = None;
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}

impl Default for StudioState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate() {
        let mut state = StudioState::new();
        assert_eq!(state.current_view, StudioView::RunList);
        state.navigate(StudioView::Timeline { run_id: "r1".into() });
        assert!(matches!(state.current_view, StudioView::Timeline { .. }));
    }

    #[test]
    fn test_status() {
        let mut state = StudioState::new();
        state.set_status("loading");
        assert_eq!(state.status_message, Some("loading".to_string()));
        state.clear_status();
        assert!(state.status_message.is_none());
    }
}
