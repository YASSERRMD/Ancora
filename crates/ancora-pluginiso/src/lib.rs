/// ancora-pluginiso - Plugin isolation and safety for the Ancora agent framework.
///
/// Plugins run sandboxed with resource, network, and filesystem limits,
/// cryptographic signature verification, and crash isolation.

pub mod audit;
pub mod capability_grants;
pub mod crash_isolation;
pub mod filesystem_policy;
pub mod network_policy;
pub mod residency;
pub mod resource_limits;
pub mod sandbox;
pub mod signature;
pub mod subprocess_runtime;
pub mod wasm_runtime;

#[cfg(test)]
mod tests {
    mod test_crash_isolated;
    mod test_filesystem_blocked;
    mod test_network_blocked;
    mod test_plugin_audited;
    mod test_residency_enforced;
    mod test_resource_limits;
    mod test_subprocess_isolated;
    mod test_unsigned_rejected;
    mod test_wasm_sandboxed;
}
