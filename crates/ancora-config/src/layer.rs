use crate::schema::AncoraCfg;

/// Config layering: base defaults, file overlay, env overlay, tenant overlay.
/// Each layer overrides only the fields it specifies.
#[derive(Default)]
pub struct ConfigLayers {
    base: AncoraCfg,
    file: Option<AncoraCfg>,
    env: Option<AncoraCfg>,
    tenant: Option<AncoraCfg>,
}

impl ConfigLayers {
    pub fn new(base: AncoraCfg) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

    pub fn with_file(mut self, overlay: AncoraCfg) -> Self {
        self.file = Some(overlay);
        self
    }

    pub fn with_env(mut self, overlay: AncoraCfg) -> Self {
        self.env = Some(overlay);
        self
    }

    pub fn with_tenant(mut self, overlay: AncoraCfg) -> Self {
        self.tenant = Some(overlay);
        self
    }

    /// Merge all layers: base < file < env < tenant.
    pub fn resolve(self) -> AncoraCfg {
        let mut merged = self.base;
        for layer in [self.file, self.env, self.tenant].into_iter().flatten() {
            merge_into(&mut merged, layer);
        }
        merged
    }
}

fn merge_into(base: &mut AncoraCfg, overlay: AncoraCfg) {
    // Core
    if overlay.core.log_level != "info" || base.core.log_level == "info" {
        base.core.log_level = overlay.core.log_level;
    }
    if !overlay.core.data_dir.is_empty() {
        base.core.data_dir = overlay.core.data_dir;
    }
    if overlay.core.max_concurrent_runs != 0 {
        base.core.max_concurrent_runs = overlay.core.max_concurrent_runs;
    }
    // Journal
    if overlay.journal.flush_interval_ms != 0 {
        base.journal.flush_interval_ms = overlay.journal.flush_interval_ms;
    }
    if overlay.journal.max_entries_per_batch != 0 {
        base.journal.max_entries_per_batch = overlay.journal.max_entries_per_batch;
    }
    // Worker
    if overlay.worker.concurrency != 0 {
        base.worker.concurrency = overlay.worker.concurrency;
    }
    if !overlay.worker.provider.is_empty() {
        base.worker.provider = overlay.worker.provider;
    }
    if overlay.worker.api_key_ref.is_some() {
        base.worker.api_key_ref = overlay.worker.api_key_ref;
    }
    // Telemetry
    base.telemetry.metrics_enabled = overlay.telemetry.metrics_enabled;
    base.telemetry.tracing_enabled = overlay.telemetry.tracing_enabled;
    if overlay.telemetry.scrape_interval_ms != 0 {
        base.telemetry.scrape_interval_ms = overlay.telemetry.scrape_interval_ms;
    }
}
