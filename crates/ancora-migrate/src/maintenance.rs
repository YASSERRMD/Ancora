/// Maintenance mode gate. When active, normal run dispatch is blocked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceState {
    Active,
    Inactive,
}

pub struct MaintenanceWindow {
    state: MaintenanceState,
    pub entered_at: Option<u64>,
    pub reason: Option<String>,
}

impl MaintenanceWindow {
    pub fn new() -> Self {
        Self {
            state: MaintenanceState::Inactive,
            entered_at: None,
            reason: None,
        }
    }

    pub fn enter(&mut self, now: u64, reason: &str) {
        self.state = MaintenanceState::Active;
        self.entered_at = Some(now);
        self.reason = Some(reason.to_string());
    }

    pub fn exit(&mut self) {
        self.state = MaintenanceState::Inactive;
        self.entered_at = None;
        self.reason = None;
    }

    pub fn is_active(&self) -> bool {
        self.state == MaintenanceState::Active
    }

    pub fn duration_secs(&self, now: u64) -> Option<u64> {
        self.entered_at.map(|start| now.saturating_sub(start))
    }
}

impl Default for MaintenanceWindow {
    fn default() -> Self {
        Self::new()
    }
}
