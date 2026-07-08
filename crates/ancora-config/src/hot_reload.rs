use crate::schema::AncoraCfg;

/// Fields that are safe to hot-reload without restarting workers.
#[derive(Clone, Debug, PartialEq)]
pub struct HotReloadableFields {
    pub log_level: String,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    pub scrape_interval_ms: u64,
}

impl HotReloadableFields {
    pub fn from_cfg(cfg: &AncoraCfg) -> Self {
        Self {
            log_level: cfg.core.log_level.clone(),
            metrics_enabled: cfg.telemetry.metrics_enabled,
            tracing_enabled: cfg.telemetry.tracing_enabled,
            scrape_interval_ms: cfg.telemetry.scrape_interval_ms,
        }
    }
}

/// Tracks active config and applies safe hot-reload deltas.
pub struct HotReloadState {
    pub current: AncoraCfg,
    pub reload_count: u32,
}

impl HotReloadState {
    pub fn new(cfg: AncoraCfg) -> Self {
        Self {
            current: cfg,
            reload_count: 0,
        }
    }

    /// Apply safe fields from a new config snapshot without full restart.
    pub fn apply_safe_reload(&mut self, new_cfg: &AncoraCfg) {
        self.current.core.log_level = new_cfg.core.log_level.clone();
        self.current.telemetry.metrics_enabled = new_cfg.telemetry.metrics_enabled;
        self.current.telemetry.tracing_enabled = new_cfg.telemetry.tracing_enabled;
        self.current.telemetry.scrape_interval_ms = new_cfg.telemetry.scrape_interval_ms;
        self.reload_count += 1;
    }

    pub fn hot_fields(&self) -> HotReloadableFields {
        HotReloadableFields::from_cfg(&self.current)
    }
}
