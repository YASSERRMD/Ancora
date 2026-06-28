use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunEntry {
    pub run_id: String,
    pub tenant_id: String,
    pub status: RunStatus,
    pub worker_id: Option<String>,
    pub created_at_secs: u64,
}

/// In-memory run store for CLI operations.
#[derive(Default)]
pub struct RunStore {
    runs: HashMap<String, RunEntry>,
}

impl RunStore {
    pub fn insert(&mut self, entry: RunEntry) {
        self.runs.insert(entry.run_id.clone(), entry);
    }

    pub fn list(&self) -> Vec<&RunEntry> {
        let mut v: Vec<&RunEntry> = self.runs.values().collect();
        v.sort_by_key(|r| &r.run_id);
        v
    }

    pub fn get(&self, run_id: &str) -> Option<&RunEntry> {
        self.runs.get(run_id)
    }

    pub fn cancel(&mut self, run_id: &str) -> bool {
        if let Some(e) = self.runs.get_mut(run_id) {
            if e.status == RunStatus::Pending || e.status == RunStatus::Running {
                e.status = RunStatus::Cancelled;
                return true;
            }
        }
        false
    }

    pub fn resume(&mut self, run_id: &str) -> bool {
        if let Some(e) = self.runs.get_mut(run_id) {
            if e.status == RunStatus::Failed || e.status == RunStatus::Cancelled {
                e.status = RunStatus::Pending;
                return true;
            }
        }
        false
    }
}
