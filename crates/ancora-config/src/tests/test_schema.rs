#[cfg(test)]
mod tests {
    use crate::schema::{AncoraCfg, CoreCfg, WorkerCfg};

    #[test]
    fn default_cfg_is_valid() {
        let cfg = AncoraCfg::default();
        assert_eq!(cfg.core.log_level, "info");
        assert_eq!(cfg.worker.concurrency, 4);
        assert_eq!(cfg.journal.flush_interval_ms, 500);
    }

    #[test]
    fn worker_cfg_carries_api_key_ref() {
        let cfg = WorkerCfg {
            api_key_ref: Some("env:OPENAI_API_KEY".into()),
            ..Default::default()
        };
        assert!(cfg.api_key_ref.is_some());
    }

    #[test]
    fn core_cfg_roundtrip_json() {
        let c = CoreCfg {
            log_level: "debug".into(),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        let c2: CoreCfg = serde_json::from_str(&json).unwrap();
        assert_eq!(c, c2);
    }
}
