use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type RunId = String;
pub type WorkerId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RunPriority {
    Low = 0,
    #[default]
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Queued,
    Assigned,
    Running,
    Paused,
    Completed,
    Cancelled,
    Failed,
    Quarantined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: RunId,
    pub tenant_id: String,
    pub priority: RunPriority,
    pub state: RunState,
    pub assigned_worker: Option<WorkerId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub cost_tokens: u64,
    pub cost_usd_micro: u64,
    pub journal_seq: u64,
}

impl Run {
    pub fn new(tenant_id: impl Into<String>, priority: RunPriority) -> Self {
        let now = Utc::now();
        Run {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.into(),
            priority,
            state: RunState::Queued,
            assigned_worker: None,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
            cost_tokens: 0,
            cost_usd_micro: 0,
            journal_seq: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: WorkerId,
    pub registered_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub current_run: Option<RunId>,
    pub concurrency_limit: usize,
    pub active_count: usize,
    pub tags: Vec<String>,
}

impl Worker {
    pub fn new(concurrency_limit: usize) -> Self {
        let now = Utc::now();
        Worker {
            id: uuid::Uuid::new_v4().to_string(),
            registered_at: now,
            last_heartbeat: now,
            lease_expires_at: None,
            current_run: None,
            concurrency_limit,
            active_count: 0,
            tags: vec![],
        }
    }

    pub fn is_lease_valid(&self) -> bool {
        match self.lease_expires_at {
            Some(exp) => Utc::now() < exp,
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub run_id: RunId,
    pub seq: u64,
    pub payload: String,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSummary {
    pub run_id: Option<RunId>,
    pub tenant_id: Option<String>,
    pub total_tokens: u64,
    pub total_usd_micro: u64,
    pub run_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeDecision {
    pub approved: bool,
    pub reason: Option<String>,
}
