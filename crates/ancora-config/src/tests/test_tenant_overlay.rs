#[cfg(test)]
mod tests {
    use crate::{
        schema::{AncoraCfg, WorkerCfg},
        tenant_overlay::TenantOverlayRegistry,
    };

    #[test]
    fn no_overlay_returns_base() {
        let base = AncoraCfg::default();
        let reg = TenantOverlayRegistry::default();
        let resolved = reg.resolve_for("tenant-x", &base);
        assert_eq!(resolved.worker.concurrency, base.worker.concurrency);
    }

    #[test]
    fn tenant_overlay_merges_concurrency() {
        let base = AncoraCfg::default();
        let overlay = AncoraCfg {
            worker: WorkerCfg { concurrency: 32, ..Default::default() },
            ..Default::default()
        };
        let mut reg = TenantOverlayRegistry::default();
        reg.register("tenant-a", overlay);
        let resolved = reg.resolve_for("tenant-a", &base);
        assert_eq!(resolved.worker.concurrency, 32);
    }

    #[test]
    fn tenant_overlay_carries_api_key_ref() {
        let base = AncoraCfg::default();
        let overlay = AncoraCfg {
            worker: WorkerCfg {
                api_key_ref: Some("env:TENANT_A_KEY".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut reg = TenantOverlayRegistry::default();
        reg.register("tenant-a", overlay);
        let resolved = reg.resolve_for("tenant-a", &base);
        assert_eq!(resolved.worker.api_key_ref.as_deref(), Some("env:TENANT_A_KEY"));
    }

    #[test]
    fn has_overlay_returns_correct_bool() {
        let mut reg = TenantOverlayRegistry::default();
        assert!(!reg.has_overlay("tenant-x"));
        reg.register("tenant-x", AncoraCfg::default());
        assert!(reg.has_overlay("tenant-x"));
    }
}
