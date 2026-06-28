use crate::connector_entry::ConnectorEntry;
use crate::provider_entry::ProviderEntry;
use crate::tool_entry::ToolEntry;
use crate::vectorstore_entry::VectorStoreEntry;

/// The in-memory catalog index holding all registered entries.
#[derive(Debug, Default)]
pub struct CatalogIndex {
    pub tools: Vec<ToolEntry>,
    pub connectors: Vec<ConnectorEntry>,
    pub providers: Vec<ProviderEntry>,
    pub vector_stores: Vec<VectorStoreEntry>,
}

impl CatalogIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_tool(&mut self, entry: ToolEntry) {
        self.tools.push(entry);
    }

    pub fn add_connector(&mut self, entry: ConnectorEntry) {
        self.connectors.push(entry);
    }

    pub fn add_provider(&mut self, entry: ProviderEntry) {
        self.providers.push(entry);
    }

    pub fn add_vector_store(&mut self, entry: VectorStoreEntry) {
        self.vector_stores.push(entry);
    }

    /// Total number of entries across all categories.
    pub fn len(&self) -> usize {
        self.tools.len()
            + self.connectors.len()
            + self.providers.len()
            + self.vector_stores.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Look up a tool by its id.
    pub fn find_tool(&self, id: &str) -> Option<&ToolEntry> {
        self.tools.iter().find(|t| t.id == id)
    }

    /// Look up a connector by its id.
    pub fn find_connector(&self, id: &str) -> Option<&ConnectorEntry> {
        self.connectors.iter().find(|c| c.id == id)
    }

    /// Look up a provider by its id.
    pub fn find_provider(&self, id: &str) -> Option<&ProviderEntry> {
        self.providers.iter().find(|p| p.id == id)
    }

    /// Look up a vector store by its id.
    pub fn find_vector_store(&self, id: &str) -> Option<&VectorStoreEntry> {
        self.vector_stores.iter().find(|v| v.id == id)
    }
}
