use crate::agent_spec::AgentTask;

/// Records a subagent spawn event.
#[derive(Debug, Clone)]
pub struct SpawnRecord {
    pub parent_agent_id: String,
    pub child_task: AgentTask,
    pub spawned_at: u64,
}

pub struct SpawnTracker {
    records: Vec<SpawnRecord>,
}

impl SpawnTracker {
    pub fn new() -> Self {
        Self { records: vec![] }
    }

    pub fn spawn(&mut self, parent_agent_id: &str, task: AgentTask, now: u64) {
        self.records.push(SpawnRecord {
            parent_agent_id: parent_agent_id.to_string(),
            child_task: task,
            spawned_at: now,
        });
    }

    pub fn by_parent(&self, parent_agent_id: &str) -> Vec<&SpawnRecord> {
        self.records
            .iter()
            .filter(|r| r.parent_agent_id == parent_agent_id)
            .collect()
    }

    pub fn total_spawned(&self) -> usize {
        self.records.len()
    }
}

impl Default for SpawnTracker {
    fn default() -> Self {
        Self::new()
    }
}
