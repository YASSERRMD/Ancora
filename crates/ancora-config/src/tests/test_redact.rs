#[cfg(test)]
mod tests {
    use crate::{
        redact::redacted_dump,
        schema::{AncoraCfg, WorkerCfg},
    };

    #[test]
    fn dump_redacts_api_key_ref() {
        let cfg = AncoraCfg {
            worker: WorkerCfg {
                api_key_ref: Some("env:SECRET_KEY".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let dump = redacted_dump(&cfg);
        let worker = &dump["worker"];
        assert_eq!(worker["api_key_ref"], "[REDACTED]");
    }

    #[test]
    fn dump_preserves_non_secret_fields() {
        let cfg = AncoraCfg::default();
        let dump = redacted_dump(&cfg);
        assert_eq!(dump["core"]["log_level"], "info");
        assert_eq!(dump["worker"]["concurrency"], 4);
    }
}
