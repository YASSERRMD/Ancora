use crate::quota::{ResourceQuota, ResourceUsage};
use crate::tenant::{Tenant, TenantStatus};

#[derive(Debug, Clone)]
pub struct TenantSnapshot {
    pub tick: u64,
    pub id: String,
    pub name: String,
    pub status: TenantStatus,
    pub agents: u64,
    pub max_agents: u64,
    pub tasks: u64,
    pub max_tasks: u64,
    pub memory_mb: u64,
    pub max_memory_mb: u64,
}

impl TenantSnapshot {
    pub fn capture(
        tick: u64,
        tenant: &Tenant,
        usage: &ResourceUsage,
        quota: &ResourceQuota,
    ) -> Self {
        Self {
            tick,
            id: tenant.id.clone(),
            name: tenant.name.clone(),
            status: tenant.status.clone(),
            agents: usage.agents,
            max_agents: quota.max_agents,
            tasks: usage.tasks,
            max_tasks: quota.max_tasks,
            memory_mb: usage.memory_mb,
            max_memory_mb: quota.max_memory_mb,
        }
    }

    pub fn agent_headroom(&self) -> u64 {
        self.max_agents.saturating_sub(self.agents)
    }

    pub fn task_headroom(&self) -> u64 {
        self.max_tasks.saturating_sub(self.tasks)
    }

    pub fn is_near_agent_limit(&self, threshold_pct: f64) -> bool {
        if self.max_agents == u64::MAX {
            return false;
        }
        (self.agents as f64 / self.max_agents as f64) >= threshold_pct
    }
}
