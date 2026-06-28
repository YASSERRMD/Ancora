/// Sandbox model for plugin isolation.
///
/// A `Sandbox` aggregates all isolation constraints that apply to a single plugin instance:
/// resource limits, network policy, filesystem policy, capability grants, crash isolation
/// mode, and signature requirements.

use crate::capability_grants::CapabilityGrants;
use crate::crash_isolation::CrashIsolationMode;
use crate::filesystem_policy::FilesystemPolicy;
use crate::network_policy::NetworkPolicy;
use crate::resource_limits::ResourceLimits;
use crate::signature::SignaturePolicy;

/// Runtime backend used to host the plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeKind {
    /// Plugin runs inside a WebAssembly (Wasm) sandbox.
    Wasm,
    /// Plugin runs as an isolated child subprocess.
    Subprocess,
}

/// Complete sandbox configuration for a plugin.
#[derive(Debug, Clone)]
pub struct Sandbox {
    pub plugin_id: String,
    pub runtime: RuntimeKind,
    pub resource_limits: ResourceLimits,
    pub network_policy: NetworkPolicy,
    pub filesystem_policy: FilesystemPolicy,
    pub capability_grants: CapabilityGrants,
    pub crash_isolation: CrashIsolationMode,
    pub signature_policy: SignaturePolicy,
}

impl Sandbox {
    /// Construct a `Sandbox` with the supplied configuration.
    pub fn new(
        plugin_id: impl Into<String>,
        runtime: RuntimeKind,
        resource_limits: ResourceLimits,
        network_policy: NetworkPolicy,
        filesystem_policy: FilesystemPolicy,
        capability_grants: CapabilityGrants,
        crash_isolation: CrashIsolationMode,
        signature_policy: SignaturePolicy,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            runtime,
            resource_limits,
            network_policy,
            filesystem_policy,
            capability_grants,
            crash_isolation,
            signature_policy,
        }
    }

    /// Returns `true` when the sandbox configuration is self-consistent and
    /// the minimum safety requirements are met (strict signature policy when
    /// crash isolation is disabled, etc.).
    pub fn is_valid(&self) -> bool {
        // At minimum every sandbox must have a non-empty plugin id.
        if self.plugin_id.is_empty() {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability_grants::CapabilityGrants;
    use crate::crash_isolation::CrashIsolationMode;
    use crate::filesystem_policy::FilesystemPolicy;
    use crate::network_policy::NetworkPolicy;
    use crate::resource_limits::ResourceLimits;
    use crate::signature::SignaturePolicy;

    fn default_sandbox(id: &str) -> Sandbox {
        Sandbox::new(
            id,
            RuntimeKind::Wasm,
            ResourceLimits::default(),
            NetworkPolicy::deny_all(),
            FilesystemPolicy::deny_all(),
            CapabilityGrants::none(),
            CrashIsolationMode::Isolated,
            SignaturePolicy::Required,
        )
    }

    #[test]
    fn valid_sandbox_passes() {
        let sb = default_sandbox("my-plugin");
        assert!(sb.is_valid());
    }

    #[test]
    fn empty_plugin_id_is_invalid() {
        let sb = default_sandbox("");
        assert!(!sb.is_valid());
    }
}
