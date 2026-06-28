/// ancora-contrib: plugin template
///
/// Copy this module as the starting point for a full ancora plugin.
/// A plugin bundles a manifest and one or more extension-point implementations.

/// Semantic version triplet.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(s: &str) -> Result<Self, PluginError> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(PluginError::InvalidVersion(s.to_string()));
        }
        let mut nums = [0u32; 3];
        for (i, p) in parts.iter().enumerate() {
            nums[i] = p
                .parse::<u32>()
                .map_err(|_| PluginError::InvalidVersion(s.to_string()))?;
        }
        Ok(Self::new(nums[0], nums[1], nums[2]))
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The category of extension point this plugin provides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginKind {
    Provider,
    VectorStore,
    Tool,
    Grader,
    Guardrail,
    Exporter,
    Custom(String),
}

impl std::fmt::Display for PluginKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PluginKind::Provider => "provider",
            PluginKind::VectorStore => "vector_store",
            PluginKind::Tool => "tool",
            PluginKind::Grader => "grader",
            PluginKind::Guardrail => "guardrail",
            PluginKind::Exporter => "exporter",
            PluginKind::Custom(c) => c.as_str(),
        };
        write!(f, "{s}")
    }
}

/// Metadata that uniquely identifies and describes a plugin.
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: SemVer,
    pub kind: PluginKind,
    pub author: Option<String>,
    pub description: Option<String>,
    pub min_sdk: SemVer,
}

/// Errors from plugin lifecycle operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginError {
    InvalidVersion(String),
    MissingField(String),
    InitFailed(String),
    ShutdownFailed(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::InvalidVersion(v) => write!(f, "invalid version: {v}"),
            PluginError::MissingField(field) => write!(f, "missing required field: {field}"),
            PluginError::InitFailed(s) => write!(f, "plugin init failed: {s}"),
            PluginError::ShutdownFailed(s) => write!(f, "plugin shutdown failed: {s}"),
        }
    }
}

impl std::error::Error for PluginError {}

/// Lifecycle trait every plugin must implement.
pub trait Plugin: Send + Sync {
    /// Return the plugin's manifest.
    fn manifest(&self) -> &PluginManifest;

    /// Called once when the plugin is loaded. Perform resource allocation here.
    fn init(&mut self) -> Result<(), PluginError>;

    /// Called once when the plugin is unloaded. Release resources here.
    fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Health check: return `Ok(())` if the plugin is functioning normally.
    fn health(&self) -> Result<(), PluginError>;
}

// ---------------------------------------------------------------------------
// Template implementation
// ---------------------------------------------------------------------------

/// Template plugin with a minimal manifest and no-op lifecycle methods.
pub struct MyPlugin {
    manifest: PluginManifest,
    initialised: bool,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "my-plugin".to_string(),
                name: "My Plugin".to_string(),
                version: SemVer::new(0, 1, 0),
                kind: PluginKind::Tool,
                author: Some("YASSERRMD".to_string()),
                description: Some("Template plugin - replace with real implementation.".to_string()),
                min_sdk: SemVer::new(0, 1, 0),
            },
            initialised: false,
        }
    }

    pub fn is_initialised(&self) -> bool {
        self.initialised
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MyPlugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    fn init(&mut self) -> Result<(), PluginError> {
        // TODO: allocate resources, open connections, etc.
        self.initialised = true;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        // TODO: close connections, flush buffers, etc.
        self.initialised = false;
        Ok(())
    }

    fn health(&self) -> Result<(), PluginError> {
        // TODO: verify that internal resources are alive.
        if self.initialised {
            Ok(())
        } else {
            Err(PluginError::InitFailed("plugin not initialised".to_string()))
        }
    }
}
