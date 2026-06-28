use crate::worker_ops::WorkerState;

pub struct WorkerAuditEntry {
    pub worker_id: String,
    pub from_state: WorkerState,
    pub to_state: WorkerState,
    pub at_secs: u64,
    pub reason: String,
}

#[derive(Default)]
pub struct WorkerAuditLog {
    entries: Vec<WorkerAuditEntry>,
}

impl WorkerAuditLog {
    pub fn record(&mut self, worker_id: &str, from: WorkerState, to: WorkerState, at: u64, reason: &str) {
        self.entries.push(WorkerAuditEntry {
            worker_id: worker_id.to_string(),
            from_state: from,
            to_state: to,
            at_secs: at,
            reason: reason.to_string(),
        });
    }

    pub fn for_worker(&self, worker_id: &str) -> Vec<&WorkerAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.worker_id == worker_id)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }
}
