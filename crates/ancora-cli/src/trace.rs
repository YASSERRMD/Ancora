use ancora_core::journal::MemoryStore;

use crate::spec::GraphSpec;

/// Print a run trace and cost summary to stdout.
pub fn print_trace(spec: &GraphSpec, _store: &MemoryStore) {
    println!();
    println!("=== run trace: {} ===", spec.name);
    for (i, node) in spec.nodes.iter().enumerate() {
        println!("  [{i}] node={} kind={}", node.id, node.kind);
    }
    println!("=== cost summary ===");
    println!("  total nodes : {}", spec.nodes.len());
    println!("  total cost  : $0.000 (offline)");
}
