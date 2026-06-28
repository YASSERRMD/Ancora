//! Plugin SDK documentation and type definitions.
//!
//! Provides the core `Plugin` trait and associated metadata types
//! that third-party crates must implement.

/// Version of the Plugin SDK API.
pub const SDK_VERSION: &str = "0.1.0";

/// Metadata describing a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

/// Lifecycle event a plugin can receive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginEvent {
    Init,
    BeforeRun,
    AfterRun,
    Shutdown,
}

/// Result type used by plugin hooks.
pub type PluginResult<T> = Result<T, PluginError>;

/// Errors that a plugin may produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginError {
    InitFailed(String),
    HookFailed(String),
    UnsupportedEvent,
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitFailed(msg) => write!(f, "init failed: {msg}"),
            Self::HookFailed(msg) => write!(f, "hook failed: {msg}"),
            Self::UnsupportedEvent => write!(f, "unsupported event"),
        }
    }
}

/// Core trait that all Ancora plugins must implement.
pub trait Plugin: Send + Sync {
    fn meta(&self) -> PluginMeta;
    fn on_event(&self, event: &PluginEvent) -> PluginResult<()>;
}

/// A no-op plugin used for documentation examples.
pub struct NoOpPlugin;

impl Plugin for NoOpPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "no-op".into(),
            version: "0.0.1".into(),
            author: "Ancora".into(),
            description: "Does nothing; useful for testing".into(),
        }
    }

    fn on_event(&self, _event: &PluginEvent) -> PluginResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_plugin_handles_all_events() {
        let p = NoOpPlugin;
        for event in [
            PluginEvent::Init,
            PluginEvent::BeforeRun,
            PluginEvent::AfterRun,
            PluginEvent::Shutdown,
        ] {
            assert!(p.on_event(&event).is_ok());
        }
    }

    #[test]
    fn sdk_version_is_semver() {
        let parts: Vec<&str> = SDK_VERSION.split('.').collect();
        assert_eq!(parts.len(), 3);
    }
}
