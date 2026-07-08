/// Enumeration of all stable extension points in the Ancora plugin SDK.
use crate::manifest::PluginKind;

/// Metadata about a single extension point.
#[derive(Debug, Clone)]
pub struct ExtensionPoint {
    /// Stable identifier matching `PluginKind::as_str()`.
    pub id: &'static str,
    /// Human-readable name.
    pub display_name: &'static str,
    /// Short description of this extension point.
    pub description: &'static str,
    /// SDK major version when this extension point was introduced.
    pub since_major: u32,
    /// Whether the extension point is stable (vs experimental).
    pub stable: bool,
}

/// All registered extension points, in definition order.
pub static EXTENSION_POINTS: &[ExtensionPoint] = &[
    ExtensionPoint {
        id: "provider",
        display_name: "LLM Provider",
        description: "Integrate an external large-language-model API.",
        since_major: 1,
        stable: true,
    },
    ExtensionPoint {
        id: "vector_store",
        display_name: "Vector Store",
        description: "Provide a vector similarity search back-end.",
        since_major: 1,
        stable: true,
    },
    ExtensionPoint {
        id: "tool",
        display_name: "Agent Tool",
        description: "Expose a callable function to agents.",
        since_major: 1,
        stable: true,
    },
    ExtensionPoint {
        id: "memory",
        display_name: "Memory Backend",
        description: "Persist and retrieve agent memory.",
        since_major: 1,
        stable: true,
    },
    ExtensionPoint {
        id: "guardrail",
        display_name: "Guardrail",
        description: "Intercept and filter agent inputs and outputs.",
        since_major: 1,
        stable: true,
    },
    ExtensionPoint {
        id: "grader",
        display_name: "Response Grader",
        description: "Score or rank agent responses.",
        since_major: 1,
        stable: true,
    },
    ExtensionPoint {
        id: "exporter",
        display_name: "Telemetry Exporter",
        description: "Forward telemetry spans and metrics to an external sink.",
        since_major: 1,
        stable: true,
    },
];

/// Look up an extension point by its stable identifier.
pub fn find(id: &str) -> Option<&'static ExtensionPoint> {
    EXTENSION_POINTS.iter().find(|ep| ep.id == id)
}

/// Return all extension points that correspond to the given `PluginKind`.
pub fn for_kind(kind: &PluginKind) -> Option<&'static ExtensionPoint> {
    find(kind.as_str())
}

/// Return every stable extension point.
pub fn all_stable() -> impl Iterator<Item = &'static ExtensionPoint> {
    EXTENSION_POINTS.iter().filter(|ep| ep.stable)
}
