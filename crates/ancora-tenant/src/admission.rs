use crate::quota::{ResourceQuota, ResourceUsage};

#[derive(Debug, PartialEq, Eq)]
pub enum AdmissionDecision {
    Allow,
    Deny(String),
}

pub struct AdmissionController;

impl AdmissionController {
    pub fn check_agents(
        quota: &ResourceQuota,
        usage: &ResourceUsage,
        delta: u64,
    ) -> AdmissionDecision {
        if usage.agents + delta > quota.max_agents {
            AdmissionDecision::Deny(format!(
                "agent quota exceeded: {}/{}",
                usage.agents + delta,
                quota.max_agents
            ))
        } else {
            AdmissionDecision::Allow
        }
    }

    pub fn check_tasks(
        quota: &ResourceQuota,
        usage: &ResourceUsage,
        delta: u64,
    ) -> AdmissionDecision {
        if usage.tasks + delta > quota.max_tasks {
            AdmissionDecision::Deny(format!(
                "task quota exceeded: {}/{}",
                usage.tasks + delta,
                quota.max_tasks
            ))
        } else {
            AdmissionDecision::Allow
        }
    }

    pub fn check_memory(
        quota: &ResourceQuota,
        usage: &ResourceUsage,
        delta_mb: u64,
    ) -> AdmissionDecision {
        if usage.memory_mb + delta_mb > quota.max_memory_mb {
            AdmissionDecision::Deny(format!(
                "memory quota exceeded: {}mb/{}",
                usage.memory_mb + delta_mb,
                quota.max_memory_mb
            ))
        } else {
            AdmissionDecision::Allow
        }
    }

    pub fn check_secrets(
        quota: &ResourceQuota,
        usage: &ResourceUsage,
        delta: u64,
    ) -> AdmissionDecision {
        if usage.secrets + delta > quota.max_secrets {
            AdmissionDecision::Deny(format!(
                "secret quota exceeded: {}/{}",
                usage.secrets + delta,
                quota.max_secrets
            ))
        } else {
            AdmissionDecision::Allow
        }
    }

    pub fn check_log_entries(
        quota: &ResourceQuota,
        usage: &ResourceUsage,
        delta: u64,
    ) -> AdmissionDecision {
        if usage.log_entries + delta > quota.max_log_entries {
            AdmissionDecision::Deny(format!(
                "log quota exceeded: {}/{}",
                usage.log_entries + delta,
                quota.max_log_entries
            ))
        } else {
            AdmissionDecision::Allow
        }
    }

    pub fn is_allowed(decision: &AdmissionDecision) -> bool {
        *decision == AdmissionDecision::Allow
    }
}
