use crate::schema::AncoraCfg;
use std::collections::HashMap;

/// Per-tenant config overrides layered on top of global config.
#[derive(Default)]
pub struct TenantOverlayRegistry {
    overlays: HashMap<String, AncoraCfg>,
}

impl TenantOverlayRegistry {
    pub fn register(&mut self, tenant_id: impl Into<String>, overlay: AncoraCfg) {
        self.overlays.insert(tenant_id.into(), overlay);
    }

    /// Return the merged config for a tenant: global base + tenant overlay.
    pub fn resolve_for(&self, tenant_id: &str, base: &AncoraCfg) -> AncoraCfg {
        match self.overlays.get(tenant_id) {
            Some(overlay) => merge(base, overlay),
            None => base.clone(),
        }
    }

    pub fn has_overlay(&self, tenant_id: &str) -> bool {
        self.overlays.contains_key(tenant_id)
    }
}

fn merge(base: &AncoraCfg, overlay: &AncoraCfg) -> AncoraCfg {
    let mut out = base.clone();
    if overlay.worker.concurrency != 0 {
        out.worker.concurrency = overlay.worker.concurrency;
    }
    if !overlay.worker.provider.is_empty() {
        out.worker.provider = overlay.worker.provider.clone();
    }
    if overlay.worker.api_key_ref.is_some() {
        out.worker.api_key_ref = overlay.worker.api_key_ref.clone();
    }
    if overlay.core.max_concurrent_runs != 0 {
        out.core.max_concurrent_runs = overlay.core.max_concurrent_runs;
    }
    out
}
