#[derive(Debug, Clone)]
pub struct ResourceQuota {
    pub max_agents: u64,
    pub max_tasks: u64,
    pub max_memory_mb: u64,
    pub max_cpu_millicores: u64,
    pub max_secrets: u64,
    pub max_log_entries: u64,
}

impl ResourceQuota {
    pub fn new(
        max_agents: u64,
        max_tasks: u64,
        max_memory_mb: u64,
        max_cpu_millicores: u64,
        max_secrets: u64,
        max_log_entries: u64,
    ) -> Self {
        Self { max_agents, max_tasks, max_memory_mb, max_cpu_millicores, max_secrets, max_log_entries }
    }

    pub fn unlimited() -> Self {
        Self::new(u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX)
    }

    pub fn standard() -> Self {
        Self::new(10, 100, 4096, 4000, 50, 100_000)
    }

    pub fn restricted() -> Self {
        Self::new(2, 20, 512, 500, 10, 10_000)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResourceUsage {
    pub agents: u64,
    pub tasks: u64,
    pub memory_mb: u64,
    pub cpu_millicores: u64,
    pub secrets: u64,
    pub log_entries: u64,
}

impl ResourceUsage {
    pub fn new() -> Self { Self::default() }
}
