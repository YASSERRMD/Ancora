use crate::capability_grants::CapabilityGrants;
use crate::crash_isolation::CrashIsolationMode;
use crate::filesystem_policy::FilesystemPolicy;
use crate::network_policy::NetworkPolicy;
use crate::resource_limits::ResourceLimits;
use crate::sandbox::{RuntimeKind, Sandbox};
use crate::signature::SignaturePolicy;
use crate::wasm_runtime::WasmInstance;

fn make_wasm_sandbox(limits: ResourceLimits) -> Sandbox {
    Sandbox::new(
        "wasm-sandbox-test",
        RuntimeKind::Wasm,
        limits,
        NetworkPolicy::deny_all(),
        FilesystemPolicy::deny_all(),
        CapabilityGrants::none(),
        CrashIsolationMode::Isolated,
        SignaturePolicy::Required,
    )
}

/// A minimal valid Wasm module header (magic + version bytes).
fn minimal_wasm() -> Vec<u8> {
    vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]
}

#[test]
fn wasm_plugin_runs_sandboxed_within_limits() {
    let limits = ResourceLimits {
        max_cpu_ms: Some(10_000),
        max_memory_bytes: Some(1_048_576), // 1 MiB
        max_threads: Some(1),
        max_open_fds: Some(0),
    };
    let mut inst =
        WasmInstance::instantiate("wasm-test-1", &minimal_wasm(), make_wasm_sandbox(limits))
            .expect("instantiation should succeed");

    let result = inst.call("echo", b"hello").expect("call should succeed");
    assert!(!result.is_empty());
}

#[test]
fn wasm_plugin_rejected_when_cpu_limit_tiny() {
    // Give the plugin zero CPU budget so the very first call exceeds limits.
    let limits = ResourceLimits {
        max_cpu_ms: Some(0),
        max_memory_bytes: Some(1_048_576),
        max_threads: Some(1),
        max_open_fds: Some(0),
    };
    let mut inst =
        WasmInstance::instantiate("wasm-test-2", &minimal_wasm(), make_wasm_sandbox(limits))
            .expect("instantiation should succeed");

    // Even a minimal call should be rejected due to the zero CPU budget.
    let result = inst.call("ping", b"x");
    assert!(result.is_err(), "should fail due to CPU limit exceeded");
}

#[test]
fn wasm_module_with_empty_bytes_rejected() {
    let limits = ResourceLimits::default();
    let result = WasmInstance::instantiate("wasm-test-3", &vec![], make_wasm_sandbox(limits));
    assert!(result.is_err(), "empty wasm bytes must be rejected");
}

#[test]
fn wasm_sandbox_has_deny_all_network_policy() {
    let sb = make_wasm_sandbox(ResourceLimits::default());
    assert!(!sb.network_policy.permits("example.com", 443));
}

#[test]
fn wasm_sandbox_has_deny_all_filesystem_policy() {
    use crate::filesystem_policy::AccessMode;
    let sb = make_wasm_sandbox(ResourceLimits::default());
    assert!(!sb
        .filesystem_policy
        .permits("/etc/passwd", AccessMode::Read));
}
