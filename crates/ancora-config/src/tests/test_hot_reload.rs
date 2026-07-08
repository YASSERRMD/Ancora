#[cfg(test)]
mod tests {
    use crate::{hot_reload::HotReloadState, schema::AncoraCfg};

    #[test]
    fn hot_reload_applies_log_level() {
        let mut state = HotReloadState::new(AncoraCfg::default());
        let mut new_cfg = AncoraCfg::default();
        new_cfg.core.log_level = "warn".into();
        state.apply_safe_reload(&new_cfg);
        assert_eq!(state.current.core.log_level, "warn");
        assert_eq!(state.reload_count, 1);
    }

    #[test]
    fn hot_reload_applies_telemetry() {
        let mut state = HotReloadState::new(AncoraCfg::default());
        let mut new_cfg = AncoraCfg::default();
        new_cfg.telemetry.tracing_enabled = true;
        new_cfg.telemetry.scrape_interval_ms = 5000;
        state.apply_safe_reload(&new_cfg);
        assert!(state.current.telemetry.tracing_enabled);
        assert_eq!(state.current.telemetry.scrape_interval_ms, 5000);
    }

    #[test]
    fn hot_reload_does_not_change_static_fields() {
        let mut base = AncoraCfg::default();
        base.core.data_dir = "/immutable".into();
        let mut state = HotReloadState::new(base);
        let mut new_cfg = AncoraCfg::default();
        new_cfg.core.data_dir = "/should-not-change".into();
        state.apply_safe_reload(&new_cfg);
        // data_dir is NOT in hot-reloadable fields, must not change
        assert_eq!(state.current.core.data_dir, "/immutable");
    }

    #[test]
    fn reload_count_increments() {
        let mut state = HotReloadState::new(AncoraCfg::default());
        state.apply_safe_reload(&AncoraCfg::default());
        state.apply_safe_reload(&AncoraCfg::default());
        assert_eq!(state.reload_count, 2);
    }
}
