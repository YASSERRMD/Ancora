use crate::worker::Version;

/// Current deployment status snapshot.
#[derive(Debug, Clone)]
pub struct DeployStatus {
    pub active_version: Option<Version>,
    pub total_switches: u32,
    pub canary_active: bool,
    pub canary_pct: f64,
}

impl Default for DeployStatus {
    fn default() -> Self {
        Self {
            active_version: None,
            total_switches: 0,
            canary_active: false,
            canary_pct: 0.0,
        }
    }
}

impl DeployStatus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_switch(&mut self, to: Version) {
        self.active_version = Some(to);
        self.total_switches += 1;
    }

    pub fn set_canary(&mut self, pct: f64) {
        self.canary_active = pct > 0.0;
        self.canary_pct = pct;
    }
}
