//! Adapter registry: catalog of known adapters by id.

use crate::model::{AdapterDescriptor, AdapterId};
use crate::runtime::{FtError, FtResult};
use std::collections::HashMap;

/// The adapter registry stores descriptors keyed by their id.
#[derive(Debug, Clone, Default)]
pub struct AdapterRegistry {
    entries: HashMap<AdapterId, AdapterDescriptor>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        AdapterRegistry {
            entries: HashMap::new(),
        }
    }

    /// Register a new adapter. Returns an error if the id already exists.
    pub fn register(&mut self, descriptor: AdapterDescriptor) -> FtResult<()> {
        if self.entries.contains_key(&descriptor.id) {
            return Err(FtError::RegistryConflict(descriptor.id.to_string()));
        }
        self.entries.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    /// Replace an existing entry unconditionally.
    pub fn upsert(&mut self, descriptor: AdapterDescriptor) {
        self.entries.insert(descriptor.id.clone(), descriptor);
    }

    /// Retrieve a descriptor by id.
    pub fn get(&self, id: &AdapterId) -> Option<&AdapterDescriptor> {
        self.entries.get(id)
    }

    /// Remove an entry by id.
    pub fn remove(&mut self, id: &AdapterId) -> Option<AdapterDescriptor> {
        self.entries.remove(id)
    }

    /// List all registered adapter ids in sorted order.
    pub fn list_ids(&self) -> Vec<&AdapterId> {
        let mut ids: Vec<&AdapterId> = self.entries.keys().collect();
        ids.sort_by_key(|id| id.as_str());
        ids
    }

    /// Count of registered adapters.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Filter by base model.
    pub fn by_base_model(&self, base_model: &str) -> Vec<&AdapterDescriptor> {
        self.entries
            .values()
            .filter(|d| d.base_model == base_model)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AdapterDescriptor;
    use std::path::PathBuf;

    fn make_desc(id: &str, base: &str) -> AdapterDescriptor {
        AdapterDescriptor::new(
            id,
            format!("Adapter {}", id),
            base,
            PathBuf::from(format!("/tmp/{}.safetensors", id)),
        )
    }

    #[test]
    fn registry_register_and_get() {
        let mut reg = AdapterRegistry::new();
        reg.register(make_desc("a1", "llama-3.1-8b")).unwrap();
        let d = reg.get(&AdapterId::new("a1")).unwrap();
        assert_eq!(d.name, "Adapter a1");
    }

    #[test]
    fn registry_conflict_error() {
        let mut reg = AdapterRegistry::new();
        reg.register(make_desc("a1", "llama-3.1-8b")).unwrap();
        let err = reg.register(make_desc("a1", "llama-3.1-8b")).unwrap_err();
        assert!(matches!(err, FtError::RegistryConflict(_)));
    }

    #[test]
    fn registry_list_ids_sorted() {
        let mut reg = AdapterRegistry::new();
        reg.register(make_desc("b1", "llama-3.1-8b")).unwrap();
        reg.register(make_desc("a1", "llama-3.1-8b")).unwrap();
        let ids: Vec<&str> = reg.list_ids().iter().map(|id| id.as_str()).collect();
        assert_eq!(ids, vec!["a1", "b1"]);
    }

    #[test]
    fn registry_by_base_model() {
        let mut reg = AdapterRegistry::new();
        reg.register(make_desc("a1", "llama-3.1-8b")).unwrap();
        reg.register(make_desc("b1", "mistral-7b")).unwrap();
        let results = reg.by_base_model("llama-3.1-8b");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_str(), "a1");
    }

    #[test]
    fn registry_remove() {
        let mut reg = AdapterRegistry::new();
        reg.register(make_desc("a1", "llama-3.1-8b")).unwrap();
        reg.remove(&AdapterId::new("a1"));
        assert!(reg.is_empty());
    }
}
