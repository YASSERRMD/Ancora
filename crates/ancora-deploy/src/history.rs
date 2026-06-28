use serde::{Deserialize, Serialize};
use crate::worker::Version;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeployEvent {
    BlueGreenSwitch { from: Version, to: Version, duration_ms: u64 },
    BlueGreenRollback { to: Version },
    CanaryStarted { version: Version, pct: f64 },
    CanaryPromoted { version: Version },
    CanaryRolledBack { reason: String },
}

#[derive(Default, Debug)]
pub struct DeployHistory {
    events: Vec<DeployEvent>,
}

impl DeployHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, event: DeployEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[DeployEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
