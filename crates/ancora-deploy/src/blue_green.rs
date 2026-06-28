use crate::error::DeployError;
use crate::worker::VersionedWorker;

pub struct BlueGreenController {
    pub blue: Vec<VersionedWorker>,
    pub green: Vec<VersionedWorker>,
    /// `true` if green is currently live
    pub green_live: bool,
}

impl BlueGreenController {
    pub fn new(blue: Vec<VersionedWorker>, green: Vec<VersionedWorker>) -> Self {
        Self { blue, green, green_live: false }
    }

    /// Drain the current live pool, then switch.
    pub fn switch(&mut self) -> Result<(), DeployError> {
        let active = if self.green_live {
            self.green.iter().map(|w| w.active_runs).sum::<u32>()
        } else {
            self.blue.iter().map(|w| w.active_runs).sum::<u32>()
        };
        if active > 0 {
            return Err(DeployError::DrainIncomplete { active_runs: active });
        }
        self.green_live = !self.green_live;
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<(), DeployError> {
        if !self.green_live {
            return Err(DeployError::RollbackFailed("already on blue".to_string()));
        }
        self.green_live = false;
        Ok(())
    }

    pub fn live_workers(&self) -> &[VersionedWorker] {
        if self.green_live { &self.green } else { &self.blue }
    }

    pub fn live_version(&self) -> Option<&crate::worker::Version> {
        self.live_workers().first().map(|w| &w.version)
    }
}
