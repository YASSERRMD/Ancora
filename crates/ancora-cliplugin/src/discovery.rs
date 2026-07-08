/// Plugin discovery - locates plugin manifests on the file system and returns
/// descriptors that can be used to load them.
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// The source from which a plugin was discovered.
#[derive(Debug, Clone, PartialEq)]
pub enum DiscoverySource {
    /// Found in a well-known system-wide plugin directory.
    SystemDir(PathBuf),
    /// Found in the user's personal plugin directory.
    UserDir(PathBuf),
    /// Explicitly provided path (e.g., via `--plugin` flag).
    Explicit(PathBuf),
}

/// A lightweight descriptor produced during discovery before the plugin is
/// actually loaded.
#[derive(Debug, Clone)]
pub struct PluginDescriptor {
    /// Stable identifier derived from the manifest.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Semantic version.
    pub version: String,
    /// Path to the plugin's root directory.
    pub path: PathBuf,
    /// Where the plugin was found.
    pub source: DiscoverySource,
    /// Additional key/value metadata from the manifest.
    pub metadata: HashMap<String, String>,
}

impl PluginDescriptor {
    /// Construct a new descriptor.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        path: PathBuf,
        source: DiscoverySource,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            path,
            source,
            metadata: HashMap::new(),
        }
    }

    /// Attach extra metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Configuration for the discovery process.
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// System-level directories to scan.
    pub system_dirs: Vec<PathBuf>,
    /// User-level directories to scan.
    pub user_dirs: Vec<PathBuf>,
    /// Explicitly listed plugin directories (highest priority).
    pub explicit_paths: Vec<PathBuf>,
    /// Name of the manifest file to look for inside each candidate directory.
    pub manifest_filename: String,
}

impl DiscoveryConfig {
    /// Create a config with no directories and the default manifest filename.
    pub fn new() -> Self {
        Self {
            system_dirs: Vec::new(),
            user_dirs: Vec::new(),
            explicit_paths: Vec::new(),
            manifest_filename: "plugin.toml".to_string(),
        }
    }

    /// Add a system directory.
    pub fn with_system_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.system_dirs.push(path.into());
        self
    }

    /// Add a user directory.
    pub fn with_user_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.user_dirs.push(path.into());
        self
    }

    /// Add an explicit path.
    pub fn with_explicit(mut self, path: impl Into<PathBuf>) -> Self {
        self.explicit_paths.push(path.into());
        self
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a minimal TOML-like manifest from a string (no external dependency).
///
/// Supports only `key = "value"` lines; returns the parsed pairs.
fn parse_simple_manifest(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim().to_string();
            let val = v.trim().trim_matches('"').to_string();
            map.insert(key, val);
        }
    }
    map
}

/// Discover plugins in a single directory.
fn scan_dir(
    dir: &Path,
    manifest_name: &str,
    source_fn: impl Fn(PathBuf) -> DiscoverySource,
) -> Vec<PluginDescriptor> {
    let mut found = Vec::new();

    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return found,
    };

    for entry in read_dir.flatten() {
        let candidate = entry.path();
        if !candidate.is_dir() {
            continue;
        }
        let manifest_path = candidate.join(manifest_name);
        if !manifest_path.exists() {
            continue;
        }
        let content = match std::fs::read_to_string(&manifest_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let kv = parse_simple_manifest(&content);
        let id = kv.get("id").cloned().unwrap_or_else(|| {
            candidate
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        });
        let name = kv.get("name").cloned().unwrap_or_else(|| id.clone());
        let version = kv
            .get("version")
            .cloned()
            .unwrap_or_else(|| "0.0.0".to_string());
        let source = source_fn(candidate.clone());
        let mut desc = PluginDescriptor::new(id, name, version, candidate, source);
        for (k, v) in &kv {
            if k != "id" && k != "name" && k != "version" {
                desc.metadata.insert(k.clone(), v.clone());
            }
        }
        found.push(desc);
    }

    found
}

/// Run the discovery process and return all found plugin descriptors.
///
/// Order: explicit paths first, then user dirs, then system dirs.
/// Plugins with the same id are deduplicated (first occurrence wins).
pub fn discover(config: &DiscoveryConfig) -> Vec<PluginDescriptor> {
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut results: Vec<PluginDescriptor> = Vec::new();

    let manifest = &config.manifest_filename;

    // Explicit paths - treat each path directly as a plugin dir.
    for path in &config.explicit_paths {
        let manifest_path = path.join(manifest.as_str());
        if manifest_path.exists() {
            let content = std::fs::read_to_string(&manifest_path).unwrap_or_default();
            let kv = parse_simple_manifest(&content);
            let id = kv.get("id").cloned().unwrap_or_else(|| {
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });
            if seen_ids.insert(id.clone()) {
                let name = kv.get("name").cloned().unwrap_or_else(|| id.clone());
                let version = kv
                    .get("version")
                    .cloned()
                    .unwrap_or_else(|| "0.0.0".to_string());
                let source = DiscoverySource::Explicit(path.clone());
                let mut desc = PluginDescriptor::new(id, name, version, path.clone(), source);
                for (k, v) in &kv {
                    if k != "id" && k != "name" && k != "version" {
                        desc.metadata.insert(k.clone(), v.clone());
                    }
                }
                results.push(desc);
            }
        }
    }

    // User dirs.
    for dir in &config.user_dirs {
        let dir_clone = dir.clone();
        for desc in scan_dir(dir, manifest, |p| {
            DiscoverySource::UserDir(dir_clone.clone().join(p.file_name().unwrap_or_default()))
        }) {
            if seen_ids.insert(desc.id.clone()) {
                results.push(desc);
            }
        }
    }

    // System dirs.
    for dir in &config.system_dirs {
        let dir_clone = dir.clone();
        for desc in scan_dir(dir, manifest, |p| {
            DiscoverySource::SystemDir(dir_clone.clone().join(p.file_name().unwrap_or_default()))
        }) {
            if seen_ids.insert(desc.id.clone()) {
                results.push(desc);
            }
        }
    }

    results
}
