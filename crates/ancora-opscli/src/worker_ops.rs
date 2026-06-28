use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerState {
    Active,
    Draining,
    Down,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerStatus {
    pub worker_id: String,
    pub state: WorkerState,
    pub active_runs: u32,
    pub last_heartbeat_secs: u64,
}

#[derive(Default)]
pub struct WorkerRegistry {
    workers: Vec<WorkerStatus>,
}

impl WorkerRegistry {
    pub fn register(&mut self, ws: WorkerStatus) {
        self.workers.push(ws);
    }

    pub fn list(&self) -> &[WorkerStatus] {
        &self.workers
    }

    pub fn drain(&mut self, worker_id: &str) -> bool {
        if let Some(w) = self.workers.iter_mut().find(|w| w.worker_id == worker_id) {
            w.state = WorkerState::Draining;
            return true;
        }
        false
    }

    pub fn is_drained(&self, worker_id: &str) -> bool {
        self.workers
            .iter()
            .find(|w| w.worker_id == worker_id)
            .map(|w| w.active_runs == 0 && w.state == WorkerState::Draining)
            .unwrap_or(false)
    }
}
