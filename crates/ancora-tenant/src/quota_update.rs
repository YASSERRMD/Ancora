use crate::quota::ResourceQuota;

pub struct QuotaUpdate {
    pub max_agents: Option<u64>,
    pub max_tasks: Option<u64>,
    pub max_memory_mb: Option<u64>,
    pub max_cpu_millicores: Option<u64>,
    pub max_secrets: Option<u64>,
    pub max_log_entries: Option<u64>,
}

impl QuotaUpdate {
    pub fn new() -> Self {
        Self {
            max_agents: None,
            max_tasks: None,
            max_memory_mb: None,
            max_cpu_millicores: None,
            max_secrets: None,
            max_log_entries: None,
        }
    }

    pub fn agents(mut self, v: u64) -> Self {
        self.max_agents = Some(v);
        self
    }
    pub fn tasks(mut self, v: u64) -> Self {
        self.max_tasks = Some(v);
        self
    }
    pub fn memory_mb(mut self, v: u64) -> Self {
        self.max_memory_mb = Some(v);
        self
    }
    pub fn cpu_millicores(mut self, v: u64) -> Self {
        self.max_cpu_millicores = Some(v);
        self
    }
    pub fn secrets(mut self, v: u64) -> Self {
        self.max_secrets = Some(v);
        self
    }
    pub fn log_entries(mut self, v: u64) -> Self {
        self.max_log_entries = Some(v);
        self
    }

    pub fn apply(&self, quota: &mut ResourceQuota) {
        if let Some(v) = self.max_agents {
            quota.max_agents = v;
        }
        if let Some(v) = self.max_tasks {
            quota.max_tasks = v;
        }
        if let Some(v) = self.max_memory_mb {
            quota.max_memory_mb = v;
        }
        if let Some(v) = self.max_cpu_millicores {
            quota.max_cpu_millicores = v;
        }
        if let Some(v) = self.max_secrets {
            quota.max_secrets = v;
        }
        if let Some(v) = self.max_log_entries {
            quota.max_log_entries = v;
        }
    }
}

impl Default for QuotaUpdate {
    fn default() -> Self {
        Self::new()
    }
}
