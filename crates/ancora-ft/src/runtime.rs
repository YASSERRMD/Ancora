//! Adapter runtime: load, hot-swap, stacking, and tenant selection.

use crate::model::{AdapterId, AdapterDescriptor, BaseModel, LoadedAdapter};
use std::collections::HashMap;

/// Error type for runtime operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FtError {
    AdapterNotFound(String),
    IncompatibleBaseModel { expected: String, got: String },
    StackingNotSupported(String),
    IntegrityFailure(String),
    RegistryConflict(String),
    TenantNotFound(String),
    ExportError(String),
}

impl std::fmt::Display for FtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FtError::AdapterNotFound(id) => write!(f, "adapter not found: {}", id),
            FtError::IncompatibleBaseModel { expected, got } => {
                write!(f, "base model mismatch: expected {}, got {}", expected, got)
            }
            FtError::StackingNotSupported(id) => {
                write!(f, "adapter {} does not support stacking", id)
            }
            FtError::IntegrityFailure(msg) => write!(f, "integrity failure: {}", msg),
            FtError::RegistryConflict(id) => write!(f, "adapter id already registered: {}", id),
            FtError::TenantNotFound(t) => write!(f, "tenant not found: {}", t),
            FtError::ExportError(msg) => write!(f, "export error: {}", msg),
        }
    }
}

pub type FtResult<T> = Result<T, FtError>;

/// Load an adapter descriptor onto a base model, returning the loaded adapter.
///
/// Validates that the adapter targets the same base model.
pub fn load_adapter_onto(
    model: &mut BaseModel,
    descriptor: AdapterDescriptor,
    weight_bytes: u64,
) -> FtResult<AdapterId> {
    if descriptor.base_model != model.id {
        return Err(FtError::IncompatibleBaseModel {
            expected: descriptor.base_model.clone(),
            got: model.id.clone(),
        });
    }
    let id = descriptor.id.clone();
    let loaded = LoadedAdapter::new(descriptor, weight_bytes);
    model.load_adapter(loaded);
    Ok(id)
}

/// Hot-swap the active adapter on a model to a new descriptor.
///
/// Deactivates all current adapters, then loads the new one.
pub fn hot_swap(
    model: &mut BaseModel,
    new_descriptor: AdapterDescriptor,
    weight_bytes: u64,
) -> FtResult<AdapterId> {
    if new_descriptor.base_model != model.id {
        return Err(FtError::IncompatibleBaseModel {
            expected: new_descriptor.base_model.clone(),
            got: model.id.clone(),
        });
    }
    // Deactivate all existing adapters.
    for adapter in model.loaded_adapters.iter_mut() {
        adapter.deactivate();
    }
    let id = new_descriptor.id.clone();
    let loaded = LoadedAdapter::new(new_descriptor, weight_bytes);
    model.loaded_adapters.push(loaded);
    Ok(id)
}

/// Stack multiple adapters onto a model in order.
///
/// All adapters must be stackable and target the same base model.
pub fn stack_adapters(
    model: &mut BaseModel,
    descriptors: Vec<(AdapterDescriptor, u64)>,
) -> FtResult<Vec<AdapterId>> {
    // Validate all first.
    for (desc, _) in &descriptors {
        if desc.base_model != model.id {
            return Err(FtError::IncompatibleBaseModel {
                expected: desc.base_model.clone(),
                got: model.id.clone(),
            });
        }
        if !desc.stackable {
            return Err(FtError::StackingNotSupported(desc.id.to_string()));
        }
    }
    let mut ids = Vec::new();
    for (desc, bytes) in descriptors {
        let id = desc.id.clone();
        model.load_adapter(LoadedAdapter::new(desc, bytes));
        ids.push(id);
    }
    Ok(ids)
}

/// Per-tenant adapter selection state.
#[derive(Debug, Clone)]
pub struct TenantAdapterMap {
    /// tenant_id -> adapter_id
    assignments: HashMap<String, AdapterId>,
}

impl TenantAdapterMap {
    pub fn new() -> Self {
        TenantAdapterMap {
            assignments: HashMap::new(),
        }
    }

    /// Assign an adapter to a tenant.
    pub fn assign(&mut self, tenant_id: impl Into<String>, adapter_id: AdapterId) {
        self.assignments.insert(tenant_id.into(), adapter_id);
    }

    /// Look up the adapter assigned to a tenant.
    pub fn get(&self, tenant_id: &str) -> Option<&AdapterId> {
        self.assignments.get(tenant_id)
    }

    /// Remove a tenant's assignment.
    pub fn remove(&mut self, tenant_id: &str) -> Option<AdapterId> {
        self.assignments.remove(tenant_id)
    }

    /// List all tenant assignments.
    pub fn assignments(&self) -> &HashMap<String, AdapterId> {
        &self.assignments
    }
}

impl Default for TenantAdapterMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AdapterDescriptor, BaseModel};
    use std::path::PathBuf;

    fn make_model(id: &str) -> BaseModel {
        BaseModel::new(id, PathBuf::from("/tmp/model"), 8.0)
    }

    fn make_desc(adapter_id: &str, base: &str) -> AdapterDescriptor {
        AdapterDescriptor::new(
            adapter_id,
            "Test Adapter",
            base,
            PathBuf::from(format!("/tmp/{}.safetensors", adapter_id)),
        )
    }

    #[test]
    fn load_adapter_onto_succeeds() {
        let mut model = make_model("llama-3.1-8b");
        let desc = make_desc("a1", "llama-3.1-8b");
        let id = load_adapter_onto(&mut model, desc, 1024).unwrap();
        assert_eq!(id.as_str(), "a1");
        assert_eq!(model.active_adapter_count(), 1);
    }

    #[test]
    fn load_adapter_onto_fails_wrong_base() {
        let mut model = make_model("llama-3.1-8b");
        let desc = make_desc("a1", "mistral-7b");
        let err = load_adapter_onto(&mut model, desc, 1024).unwrap_err();
        assert!(matches!(err, FtError::IncompatibleBaseModel { .. }));
    }

    #[test]
    fn hot_swap_deactivates_old() {
        let mut model = make_model("llama-3.1-8b");
        let desc1 = make_desc("a1", "llama-3.1-8b");
        load_adapter_onto(&mut model, desc1, 512).unwrap();
        let desc2 = make_desc("a2", "llama-3.1-8b");
        hot_swap(&mut model, desc2, 512).unwrap();
        // a1 should be deactivated, a2 active
        let a1 = model.get_adapter(&AdapterId::new("a1")).unwrap();
        assert!(!a1.active);
        let a2 = model.get_adapter(&AdapterId::new("a2")).unwrap();
        assert!(a2.active);
    }

    #[test]
    fn stack_adapters_multiple() {
        let mut model = make_model("llama-3.1-8b");
        let descs = vec![
            (make_desc("a1", "llama-3.1-8b"), 256u64),
            (make_desc("a2", "llama-3.1-8b"), 256u64),
        ];
        let ids = stack_adapters(&mut model, descs).unwrap();
        assert_eq!(ids.len(), 2);
        assert_eq!(model.active_adapter_count(), 2);
    }

    #[test]
    fn stack_adapters_non_stackable_fails() {
        let mut model = make_model("llama-3.1-8b");
        let mut desc = make_desc("a1", "llama-3.1-8b");
        desc.stackable = false;
        let err = stack_adapters(&mut model, vec![(desc, 256)]).unwrap_err();
        assert!(matches!(err, FtError::StackingNotSupported(_)));
    }

    #[test]
    fn tenant_adapter_map_assign_get() {
        let mut map = TenantAdapterMap::new();
        map.assign("tenant-alpha", AdapterId::new("a1"));
        let id = map.get("tenant-alpha").unwrap();
        assert_eq!(id.as_str(), "a1");
    }

    #[test]
    fn tenant_adapter_map_remove() {
        let mut map = TenantAdapterMap::new();
        map.assign("t1", AdapterId::new("a1"));
        let removed = map.remove("t1").unwrap();
        assert_eq!(removed.as_str(), "a1");
        assert!(map.get("t1").is_none());
    }
}
