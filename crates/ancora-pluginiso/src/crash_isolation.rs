/// Plugin crash isolation.
///
/// Determines how the host handles a plugin crash or panic.  When isolation is
/// enabled the host process continues running; the failed plugin is marked as
/// faulted and new invocations return an error until the plugin is restarted or
/// removed.

/// Crash isolation strategy for a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrashIsolationMode {
    /// Plugin crash is fully contained: the host is unaffected.
    Isolated,
    /// Plugin crash propagates to the host (useful only for trusted built-ins).
    Propagate,
}

/// The current liveness state of a plugin instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginHealth {
    /// The plugin is running normally.
    Healthy,
    /// The plugin crashed; the host is waiting for restart or eviction.
    Faulted { reason: String },
    /// The plugin was stopped cleanly.
    Stopped,
}

/// Tracks the health of a plugin and enforces the crash-isolation contract.
#[derive(Debug)]
pub struct CrashIsolationHandle {
    pub plugin_id: String,
    pub mode: CrashIsolationMode,
    pub health: PluginHealth,
}

impl CrashIsolationHandle {
    /// Create a new handle for the given plugin.
    pub fn new(plugin_id: impl Into<String>, mode: CrashIsolationMode) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            mode,
            health: PluginHealth::Healthy,
        }
    }

    /// Record a crash.  Returns `true` when the host should propagate the
    /// crash (i.e., the mode is `Propagate`).
    pub fn record_crash(&mut self, reason: impl Into<String>) -> bool {
        self.health = PluginHealth::Faulted {
            reason: reason.into(),
        };
        self.mode == CrashIsolationMode::Propagate
    }

    /// Returns `true` when the plugin is healthy and may receive requests.
    pub fn is_healthy(&self) -> bool {
        self.health == PluginHealth::Healthy
    }

    /// Reset the plugin to a healthy state (e.g., after restart).
    pub fn reset(&mut self) {
        self.health = PluginHealth::Healthy;
    }

    /// Mark the plugin as stopped.
    pub fn stop(&mut self) {
        self.health = PluginHealth::Stopped;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_crash_does_not_propagate() {
        let mut handle = CrashIsolationHandle::new("plugin-a", CrashIsolationMode::Isolated);
        let should_propagate = handle.record_crash("segfault");
        assert!(!should_propagate);
        assert!(!handle.is_healthy());
    }

    #[test]
    fn propagate_mode_signals_host() {
        let mut handle = CrashIsolationHandle::new("builtin", CrashIsolationMode::Propagate);
        let should_propagate = handle.record_crash("oom");
        assert!(should_propagate);
    }

    #[test]
    fn reset_restores_health() {
        let mut handle = CrashIsolationHandle::new("plugin-b", CrashIsolationMode::Isolated);
        handle.record_crash("timeout");
        assert!(!handle.is_healthy());
        handle.reset();
        assert!(handle.is_healthy());
    }
}
