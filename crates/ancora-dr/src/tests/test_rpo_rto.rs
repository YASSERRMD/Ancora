#[cfg(test)]
mod tests {
    use crate::DRConfig;

    #[test]
    fn replication_interval_satisfies_rpo() {
        let cfg = DRConfig {
            rpo_secs: 60,
            rto_secs: 300,
            replication_interval_secs: 30,
        };
        assert!(cfg.replication_satisfies_rpo());
    }

    #[test]
    fn replication_interval_violates_rpo() {
        let cfg = DRConfig {
            rpo_secs: 10,
            rto_secs: 300,
            replication_interval_secs: 30,
        };
        assert!(!cfg.replication_satisfies_rpo());
    }

    #[test]
    fn default_config_satisfies_rpo() {
        let cfg = DRConfig::default();
        assert!(cfg.replication_satisfies_rpo());
    }
}
