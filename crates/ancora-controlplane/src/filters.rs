use crate::model::{Run, RunPriority, RunState};

#[derive(Debug, Default)]
pub struct RunFilter {
    pub tenant_id: Option<String>,
    pub state: Option<RunState>,
    pub priority_min: Option<RunPriority>,
}

impl RunFilter {
    pub fn matches(&self, run: &Run) -> bool {
        self.tenant_id.as_deref().map_or(true, |t| run.tenant_id == t)
            && self.state.as_ref().map_or(true, |s| &run.state == s)
            && self
                .priority_min
                .map_or(true, |p| run.priority >= p)
    }
}
