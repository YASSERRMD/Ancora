/// Recovery Point Objective and Recovery Time Objective configuration.
#[derive(Debug, Clone)]
pub struct DRConfig {
    /// Maximum acceptable data loss in seconds.
    pub rpo_secs: u64,
    /// Maximum time to restore service in seconds.
    pub rto_secs: u64,
    /// Replication interval in seconds.
    pub replication_interval_secs: u64,
}

impl Default for DRConfig {
    fn default() -> Self {
        Self {
            rpo_secs: 60,
            rto_secs: 300,
            replication_interval_secs: 30,
        }
    }
}

impl DRConfig {
    pub fn replication_satisfies_rpo(&self) -> bool {
        self.replication_interval_secs <= self.rpo_secs
    }
}
