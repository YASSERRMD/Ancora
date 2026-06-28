use crate::quota::{ResourceQuota, ResourceUsage};

#[derive(Debug)]
pub struct TenantSummary {
    pub tenant_id: String,
    pub tenant_name: String,
    pub status: String,
    pub quota: QuotaSummary,
}

#[derive(Debug)]
pub struct QuotaSummary {
    pub agents_used: u64,
    pub agents_max: u64,
    pub tasks_used: u64,
    pub tasks_max: u64,
    pub memory_used_mb: u64,
    pub memory_max_mb: u64,
    pub secrets_used: u64,
    pub secrets_max: u64,
    pub agent_utilization_pct: f64,
    pub task_utilization_pct: f64,
}

impl QuotaSummary {
    pub fn from(usage: &ResourceUsage, quota: &ResourceQuota) -> Self {
        let agent_utilization_pct = if quota.max_agents == 0 || quota.max_agents == u64::MAX {
            0.0
        } else {
            usage.agents as f64 / quota.max_agents as f64 * 100.0
        };
        let task_utilization_pct = if quota.max_tasks == 0 || quota.max_tasks == u64::MAX {
            0.0
        } else {
            usage.tasks as f64 / quota.max_tasks as f64 * 100.0
        };
        Self {
            agents_used: usage.agents,
            agents_max: quota.max_agents,
            tasks_used: usage.tasks,
            tasks_max: quota.max_tasks,
            memory_used_mb: usage.memory_mb,
            memory_max_mb: quota.max_memory_mb,
            secrets_used: usage.secrets,
            secrets_max: quota.max_secrets,
            agent_utilization_pct,
            task_utilization_pct,
        }
    }
}
