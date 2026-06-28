/// Subprocess plugin runtime.
///
/// Plugins running as subprocesses execute in a separate OS process, which
/// provides OS-level address-space isolation. The host communicates with the
/// subprocess over a stdio-based protocol. Resource limits are enforced via
/// OS mechanisms (rlimit on POSIX, Job Objects on Windows).

use crate::resource_limits::ResourceViolation;
use crate::sandbox::Sandbox;

/// The path to the plugin executable.
pub type ExecutablePath = String;

/// A simple request sent to the subprocess plugin over the IPC channel.
#[derive(Debug, Clone)]
pub struct PluginRequest {
    pub method: String,
    pub payload: Vec<u8>,
}

/// A response received from the subprocess plugin.
#[derive(Debug, Clone)]
pub struct PluginResponse {
    pub status: ResponseStatus,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseStatus {
    Ok,
    Error,
}

/// Handle to a running subprocess plugin instance.
#[derive(Debug)]
pub struct SubprocessInstance {
    pub instance_id: String,
    pub sandbox: Sandbox,
    /// Simulated resource counters.
    simulated_cpu_ms: u64,
    simulated_memory: u64,
    /// Whether the subprocess is still running.
    running: bool,
}

impl SubprocessInstance {
    /// Spawn the subprocess plugin, applying sandbox limits.
    ///
    /// In production this would `std::process::Command` the executable,
    /// set `setrlimit` calls, configure a seccomp-bpf filter, and start
    /// the IPC protocol handshake.
    pub fn spawn(
        instance_id: impl Into<String>,
        _executable: &ExecutablePath,
        sandbox: Sandbox,
    ) -> Result<Self, SubprocessError> {
        if _executable.is_empty() {
            return Err(SubprocessError::SpawnFailed("executable path is empty".into()));
        }
        Ok(Self {
            instance_id: instance_id.into(),
            sandbox,
            simulated_cpu_ms: 0,
            simulated_memory: 0,
            running: true,
        })
    }

    /// Send a request to the subprocess and wait for a response.
    pub fn send(&mut self, req: PluginRequest) -> Result<PluginResponse, SubprocessError> {
        if !self.running {
            return Err(SubprocessError::ProcessExited);
        }

        // Simulate resource consumption.
        self.simulated_cpu_ms += 2 + req.payload.len() as u64;
        self.simulated_memory += req.payload.len() as u64;

        self.sandbox
            .resource_limits
            .check(self.simulated_cpu_ms, self.simulated_memory, 1, 1)
            .map_err(SubprocessError::ResourceExceeded)?;

        // Simulate an echo response.
        let mut payload = req.method.as_bytes().to_vec();
        payload.push(b'=');
        payload.extend_from_slice(&req.payload);

        Ok(PluginResponse { status: ResponseStatus::Ok, payload })
    }

    /// Terminate the subprocess.
    pub fn terminate(&mut self) {
        self.running = false;
    }

    /// Returns `true` if the subprocess is still running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}

/// Errors produced by the subprocess runtime.
#[derive(Debug)]
pub enum SubprocessError {
    SpawnFailed(String),
    ProcessExited,
    IpcError(String),
    ResourceExceeded(ResourceViolation),
}

impl std::fmt::Display for SubprocessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpawnFailed(msg) => write!(f, "spawn failed: {}", msg),
            Self::ProcessExited => write!(f, "subprocess exited unexpectedly"),
            Self::IpcError(msg) => write!(f, "ipc error: {}", msg),
            Self::ResourceExceeded(v) => write!(f, "resource exceeded: {}", v),
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
            "subprocess-test",
            RuntimeKind::Subprocess,
            ResourceLimits::default(),
            NetworkPolicy::deny_all(),
            FilesystemPolicy::deny_all(),
            CapabilityGrants::none(),
            CrashIsolationMode::Isolated,
            SignaturePolicy::Required,
        )
    }

    #[test]
    fn empty_executable_rejected() {
        let res = SubprocessInstance::spawn("id", &"".to_string(), sandbox_for_test());
        assert!(res.is_err());
    }

    #[test]
    fn valid_spawn_and_send() {
        let mut inst = SubprocessInstance::spawn(
            "id",
            &"/usr/local/bin/my-plugin".to_string(),
            sandbox_for_test(),
        )
        .expect("spawn ok");
        let resp = inst
            .send(PluginRequest { method: "ping".into(), payload: b"hello".to_vec() })
            .expect("send ok");
        assert_eq!(resp.status, ResponseStatus::Ok);
    }

    #[test]
    fn terminated_instance_returns_error() {
        let mut inst = SubprocessInstance::spawn(
            "id",
            &"/usr/local/bin/my-plugin".to_string(),
            sandbox_for_test(),
        )
        .expect("spawn ok");
        inst.terminate();
        let res = inst.send(PluginRequest { method: "op".into(), payload: vec![] });
        assert!(res.is_err());
    }
}
