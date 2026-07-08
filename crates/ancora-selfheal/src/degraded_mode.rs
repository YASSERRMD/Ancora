/// Degraded mode: the system is partially operational.
/// In degraded mode new runs can still be accepted but capabilities are limited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemMode {
    Normal,
    Degraded { capabilities_lost: Vec<String> },
    Emergency,
}

pub struct DegradedController {
    pub mode: SystemMode,
}

impl DegradedController {
    pub fn new() -> Self {
        Self {
            mode: SystemMode::Normal,
        }
    }

    pub fn enter_degraded(&mut self, lost: Vec<String>) {
        self.mode = SystemMode::Degraded {
            capabilities_lost: lost,
        };
    }

    pub fn enter_emergency(&mut self) {
        self.mode = SystemMode::Emergency;
    }

    pub fn recover(&mut self) {
        self.mode = SystemMode::Normal;
    }

    pub fn is_accepting_runs(&self) -> bool {
        !matches!(self.mode, SystemMode::Emergency)
    }
}

impl Default for DegradedController {
    fn default() -> Self {
        Self::new()
    }
}
