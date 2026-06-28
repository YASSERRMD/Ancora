/// Plugin discovery and loading.

use crate::manifest::{ManifestError, PluginManifest};

/// Errors that can occur when registering or discovering plugins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryError {
    ManifestError(ManifestError),
    DuplicateId(String),
    NotFound(String),
}

impl From<ManifestError> for DiscoveryError {
    fn from(e: ManifestError) -> Self {
        DiscoveryError::ManifestError(e)
    }
}

impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryError::ManifestError(e) => write!(f, "manifest error: {e}"),
            DiscoveryError::DuplicateId(id) => write!(f, "plugin already registered: {id}"),
            DiscoveryError::NotFound(id) => write!(f, "plugin not found: {id}"),
        }
    }
}

impl std::error::Error for DiscoveryError {}

/// A registry of loaded plugin manifests.
#[derive(Debug, Default)]
pub struct PluginRegistry {
    manifests: Vec<PluginManifest>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin manifest. Returns an error if the id is already registered.
    pub fn register(&mut self, manifest: PluginManifest) -> Result<(), DiscoveryError> {
        if self.manifests.iter().any(|m| m.id == manifest.id) {
            return Err(DiscoveryError::DuplicateId(manifest.id.clone()));
        }
        self.manifests.push(manifest);
        Ok(())
    }

    /// Look up a plugin manifest by id.
    pub fn get(&self, id: &str) -> Option<&PluginManifest> {
        self.manifests.iter().find(|m| m.id == id)
    }

    /// Remove a plugin by id.
    pub fn unregister(&mut self, id: &str) -> Result<(), DiscoveryError> {
        let before = self.manifests.len();
        self.manifests.retain(|m| m.id != id);
        if self.manifests.len() == before {
            Err(DiscoveryError::NotFound(id.to_string()))
        } else {
            Ok(())
        }
    }

    /// Return all registered manifests.
    pub fn all(&self) -> &[PluginManifest] {
        &self.manifests
    }

    /// Return the number of registered plugins.
    pub fn count(&self) -> usize {
        self.manifests.len()
    }

    /// Return all manifests of a given kind.
    pub fn by_kind(
        &self,
        kind: &crate::manifest::PluginKind,
    ) -> impl Iterator<Item = &PluginManifest> {
        let kind_str = kind.as_str().to_owned();
        self.manifests
            .iter()
            .filter(move |m| m.kind.as_str() == kind_str)
    }
}
