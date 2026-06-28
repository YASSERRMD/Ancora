use crate::connector_entry::ConnectorEntry;
use crate::metadata::License;
use crate::provider_entry::ProviderEntry;
use crate::tool_entry::ToolEntry;
use crate::validation::{validate_connector, validate_provider, validate_tool};
use crate::vectorstore_entry::VectorStoreEntry;

/// A record of a successfully installed catalog entry.
#[derive(Debug, Clone)]
pub struct InstalledEntry {
    pub id: String,
    pub name: String,
    pub kind: InstalledKind,
    pub license: License,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstalledKind {
    Tool,
    Connector,
    Provider,
    VectorStore,
}

/// Error returned when an entry cannot be installed.
#[derive(Debug, Clone)]
pub struct InstallError {
    pub id: String,
    pub reason: String,
}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot install '{}': {}", self.id, self.reason)
    }
}

/// Simulated project registry - holds entries that have been installed.
#[derive(Debug, Default)]
pub struct ProjectRegistry {
    pub entries: Vec<InstalledEntry>,
}

impl ProjectRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Install a tool into this registry. Validates the entry before adding.
    pub fn install_tool(&mut self, entry: &ToolEntry) -> Result<(), InstallError> {
        let errors = validate_tool(entry);
        if !errors.is_empty() {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: errors[0].to_string(),
            });
        }
        if self.is_installed(&entry.id) {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: "already installed".into(),
            });
        }
        self.entries.push(InstalledEntry {
            id: entry.id.clone(),
            name: entry.name.clone(),
            kind: InstalledKind::Tool,
            license: entry.metadata.license.clone(),
        });
        Ok(())
    }

    /// Install a connector into this registry.
    pub fn install_connector(&mut self, entry: &ConnectorEntry) -> Result<(), InstallError> {
        let errors = validate_connector(entry);
        if !errors.is_empty() {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: errors[0].to_string(),
            });
        }
        if self.is_installed(&entry.id) {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: "already installed".into(),
            });
        }
        self.entries.push(InstalledEntry {
            id: entry.id.clone(),
            name: entry.name.clone(),
            kind: InstalledKind::Connector,
            license: entry.metadata.license.clone(),
        });
        Ok(())
    }

    /// Install a provider into this registry.
    pub fn install_provider(&mut self, entry: &ProviderEntry) -> Result<(), InstallError> {
        let errors = validate_provider(entry);
        if !errors.is_empty() {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: errors[0].to_string(),
            });
        }
        if self.is_installed(&entry.id) {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: "already installed".into(),
            });
        }
        self.entries.push(InstalledEntry {
            id: entry.id.clone(),
            name: entry.name.clone(),
            kind: InstalledKind::Provider,
            license: entry.metadata.license.clone(),
        });
        Ok(())
    }

    /// Install a vector store into this registry.
    pub fn install_vector_store(&mut self, entry: &VectorStoreEntry) -> Result<(), InstallError> {
        if !entry.is_valid() {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: "invalid vector store entry".into(),
            });
        }
        if self.is_installed(&entry.id) {
            return Err(InstallError {
                id: entry.id.clone(),
                reason: "already installed".into(),
            });
        }
        self.entries.push(InstalledEntry {
            id: entry.id.clone(),
            name: entry.name.clone(),
            kind: InstalledKind::VectorStore,
            license: entry.metadata.license.clone(),
        });
        Ok(())
    }

    pub fn is_installed(&self, id: &str) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }

    pub fn find_installed(&self, id: &str) -> Option<&InstalledEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn installed_count(&self) -> usize {
        self.entries.len()
    }
}
