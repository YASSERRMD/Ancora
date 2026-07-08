//! Example: load a local LoRA adapter onto a base model and hot-swap it.

use ancora_ft::export::{export_adapter, ExportOptions};
use ancora_ft::integrity::attach_integrity;
use ancora_ft::journal::select_for_tenant;
use ancora_ft::runtime::{hot_swap, load_adapter_onto, TenantAdapterMap};
use ancora_ft::{AdapterDescriptor, AdapterId, AdapterRegistry, BaseModel, SelectionJournal};
use std::path::PathBuf;

fn main() {
    // 1. Describe the base model.
    let mut model = BaseModel::new("llama-3.1-8b", PathBuf::from("/models/llama-3.1-8b"), 8.0);

    // 2. Create an adapter descriptor.
    let mut adapter = AdapterDescriptor::new(
        "customer-support-v1",
        "Customer Support LoRA v1",
        "llama-3.1-8b",
        PathBuf::from("/adapters/customer_support_v1.safetensors"),
    );

    // 3. Attach integrity metadata.
    attach_integrity(&mut adapter, 1_048_576);
    println!(
        "Adapter integrity: {:?}",
        adapter.integrity.as_ref().unwrap().sha256
    );

    // 4. Load adapter onto model.
    let id = load_adapter_onto(&mut model, adapter.clone(), 1_048_576).unwrap();
    println!("Loaded adapter: {}", id);
    println!("Active adapters: {}", model.active_adapter_count());

    // 5. Register adapter in the registry.
    let mut registry = AdapterRegistry::new();
    registry.register(adapter.clone()).unwrap();
    println!("Registry size: {}", registry.len());

    // 6. Per-tenant selection with journaling.
    let mut journal = SelectionJournal::new();
    let mut tenant_map = TenantAdapterMap::new();
    select_for_tenant(
        &mut journal,
        &mut tenant_map,
        "tenant-acme",
        AdapterId::new("customer-support-v1"),
    );
    println!("Tenant acme -> {:?}", tenant_map.get("tenant-acme"));

    // 7. Replay journal.
    let replayed = journal.replay();
    println!("Replayed: tenant-acme -> {:?}", replayed.get("tenant-acme"));

    // 8. Hot-swap to a new adapter.
    let mut adapter_v2 = AdapterDescriptor::new(
        "customer-support-v2",
        "Customer Support LoRA v2",
        "llama-3.1-8b",
        PathBuf::from("/adapters/customer_support_v2.safetensors"),
    );
    attach_integrity(&mut adapter_v2, 2_097_152);
    let new_id = hot_swap(&mut model, adapter_v2, 2_097_152).unwrap();
    println!("Hot-swapped to: {}", new_id);
    println!(
        "Active adapters after swap: {}",
        model.active_adapter_count()
    );

    // 9. Export to GGUF pointer.
    let export_opts = ExportOptions::gguf(PathBuf::from("/exports"), "Q4_K_M");
    let result = export_adapter(&adapter, "llama-3.1-8b", &export_opts).unwrap();
    println!("GGUF export path: {:?}", result.path());

    println!("Done.");
}
