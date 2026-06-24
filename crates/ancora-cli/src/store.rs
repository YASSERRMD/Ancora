use std::sync::Arc;

use ancora_core::journal::MemoryStore;

/// A type-erased journal+checkpoint store.
pub type DynStore = Arc<MemoryStore>;

/// Open a store by name.
pub fn open_store(kind: &str) -> Result<DynStore, Box<dyn std::error::Error>> {
    match kind {
        "memory" | "sqlite" => {
            Ok(Arc::new(MemoryStore::new()))
        }
        other => Err(format!("unknown store: {other}").into()),
    }
}
