#[cfg(test)]
mod tests {
    use crate::{
        layer::ConfigLayers,
        schema::{AncoraCfg, CoreCfg, WorkerCfg},
    };

    #[test]
    fn file_overlay_overrides_base() {
        let base = AncoraCfg::default();
        let overlay = AncoraCfg {
            core: CoreCfg { log_level: "debug".into(), ..Default::default() },
            ..Default::default()
        };
        let merged = ConfigLayers::new(base).with_file(overlay).resolve();
        assert_eq!(merged.core.log_level, "debug");
    }

    #[test]
    fn env_overlay_wins_over_file() {
        let base = AncoraCfg::default();
        let file_overlay = AncoraCfg {
            core: CoreCfg { log_level: "debug".into(), ..Default::default() },
            ..Default::default()
        };
        let env_overlay = AncoraCfg {
            core: CoreCfg { log_level: "warn".into(), ..Default::default() },
            ..Default::default()
        };
        let merged = ConfigLayers::new(base)
            .with_file(file_overlay)
            .with_env(env_overlay)
            .resolve();
        assert_eq!(merged.core.log_level, "warn");
    }

    #[test]
    fn tenant_overlay_wins_over_env() {
        let base = AncoraCfg::default();
        let env_overlay = AncoraCfg {
            worker: WorkerCfg { concurrency: 2, ..Default::default() },
            ..Default::default()
        };
        let tenant_overlay = AncoraCfg {
            worker: WorkerCfg { concurrency: 16, ..Default::default() },
            ..Default::default()
        };
        let merged = ConfigLayers::new(base)
            .with_env(env_overlay)
            .with_tenant(tenant_overlay)
            .resolve();
        assert_eq!(merged.worker.concurrency, 16);
    }

    #[test]
    fn base_fields_preserved_when_no_overlay() {
        let mut base = AncoraCfg::default();
        base.core.data_dir = "/custom".into();
        let merged = ConfigLayers::new(base).resolve();
        assert_eq!(merged.core.data_dir, "/custom");
    }
}
