/// WebAssembly plugin runtime.
///
/// In a production integration this module would drive a Wasm engine such as
/// Wasmtime or Wasmer.  Here we model the runtime interface without pulling in
/// heavy dependencies, so the types and contracts are clear and testable.

use crate::resource_limits::ResourceViolation;
use crate::sandbox::Sandbox;

/// The bytes of a compiled or source Wasm module.
pub type WasmBytes = Vec<u8>;

/// Handle to a running Wasm plugin instance.
#[derive(Debug)]
pub struct WasmInstance {
    /// Logical identifier for this instance.
    pub instance_id: String,
    /// The sandbox constraints applied to this instance.
    pub sandbox: Sandbox,
    /// Simulated memory usage (bytes).
    simulated_memory: u64,
    /// Simulated CPU usage (ms).
    simulated_cpu_ms: u64,
}

impl WasmInstance {
    /// Instantiate a Wasm module under the given sandbox.
    ///
    /// In production this would validate the Wasm binary, compile it through
    /// the engine, apply fuel limits for CPU, and configure the WASI imports
    /// according to the sandbox policies.
    pub fn instantiate(
        instance_id: impl Into<String>,
        _wasm: &WasmBytes,
        sandbox: Sandbox,
    ) -> Result<Self, WasmError> {
        if _wasm.is_empty() {
            return Err(WasmError::InvalidModule("empty wasm bytes".into()));
        }
        Ok(Self {
            instance_id: instance_id.into(),
            sandbox,
            simulated_memory: 0,
            simulated_cpu_ms: 0,
        })
    }

    /// Call an exported function by name with raw byte arguments.
    ///
    /// Returns the raw byte result or a `WasmError`.
    pub fn call(&mut self, func: &str, args: &[u8]) -> Result<Vec<u8>, WasmError> {
        // Simulate resource consumption proportional to argument size.
        self.simulated_cpu_ms += 1 + args.len() as u64;
        self.simulated_memory += args.len() as u64;

        // Check resource limits against simulated usage.
        self.sandbox
            .resource_limits
            .check(
                self.simulated_cpu_ms,
                self.simulated_memory,
                1,
                0,
            )
            .map_err(WasmError::ResourceExceeded)?;

        // Echo the function name and args as a trivial "result".
        let mut result = func.as_bytes().to_vec();
        result.push(b':');
        result.extend_from_slice(args);
        Ok(result)
    }

    /// Terminate the instance and release its resources.
    pub fn terminate(self) {
        // In production: drop the engine store and free linear memory.
        let _ = self.instance_id;
    }
}

/// Errors produced by the Wasm runtime.
#[derive(Debug)]
pub enum WasmError {
    InvalidModule(String),
    ResourceExceeded(ResourceViolation),
    TrapOrPanic(String),
    LinkError(String),
}

impl std::fmt::Display for WasmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidModule(msg) => write!(f, "invalid wasm module: {}", msg),
            Self::ResourceExceeded(v) => write!(f, "resource exceeded: {}", v),
            Self::TrapOrPanic(msg) => write!(f, "wasm trap/panic: {}", msg),
            Self::LinkError(msg) => write!(f, "wasm link error: {}", msg),
        }
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
    use crate::sandbox::RuntimeKind;
    use crate::signature::SignaturePolicy;

    fn sandbox_for_test() -> Sandbox {
        Sandbox::new(
            "wasm-test",
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
    fn empty_wasm_rejected() {
        let result = WasmInstance::instantiate("id", &vec![], sandbox_for_test());
        assert!(result.is_err());
    }

    #[test]
    fn valid_wasm_call_succeeds() {
        let mut inst = WasmInstance::instantiate("id", &vec![0x00, 0x61, 0x73, 0x6d], sandbox_for_test())
            .expect("instantiation ok");
        let res = inst.call("add", b"12").expect("call ok");
        assert!(!res.is_empty());
    }
}
