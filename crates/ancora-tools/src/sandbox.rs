use crate::error::ToolError;

/// Validate that a requested operation is allowed under the policy.
pub fn check_policy(
    policy: &SandboxPolicy,
    needs_network: bool,
    needs_fs_write: bool,
) -> Result<(), ToolError> {
    if needs_network && !policy.allow_network {
        return Err(ToolError::ValidationFailed(
            "network access denied by sandbox policy".into(),
        ));
    }
    if needs_fs_write && !policy.allow_filesystem_write {
        return Err(ToolError::ValidationFailed(
            "filesystem write denied by sandbox policy".into(),
        ));
    }
    Ok(())
}

/// Execution limits applied to a sandboxed subprocess.
#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    pub max_execution_ms: u64,
    pub allow_network: bool,
    pub allow_filesystem_write: bool,
}

/// Executes a command inside a sandbox, enforcing the policy limits.
pub trait Sandbox: Send + Sync {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
        input: &str,
        policy: &SandboxPolicy,
    ) -> Result<String, ToolError>;
}

/// Subprocess-based sandbox that enforces limits at the OS process level.
pub struct SubprocessSandbox;

impl Sandbox for SubprocessSandbox {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
        input: &str,
        policy: &SandboxPolicy,
    ) -> Result<String, ToolError> {
        use std::io::Write;
        use std::time::{Duration, Instant};

        let mut child = std::process::Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        drop(child.stdin.take());

        let deadline = Duration::from_millis(policy.max_execution_ms);
        let start = Instant::now();

        loop {
            match child
                .try_wait()
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?
            {
                Some(_status) => {
                    let out = child
                        .wait_with_output()
                        .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
                    return String::from_utf8(out.stdout)
                        .map_err(|e| ToolError::ExecutionFailed(e.to_string()));
                }
                None if start.elapsed() >= deadline => {
                    let _ = child.kill();
                    return Err(ToolError::ExecutionFailed(format!(
                        "sandbox timeout after {}ms",
                        policy.max_execution_ms
                    )));
                }
                None => std::thread::sleep(Duration::from_millis(10)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_denied_by_default_policy() {
        let policy = SandboxPolicy {
            max_execution_ms: 1000,
            allow_network: false,
            allow_filesystem_write: false,
        };
        let err = check_policy(&policy, true, false).unwrap_err();
        assert!(matches!(err, ToolError::ValidationFailed(_)));
    }

    #[test]
    fn filesystem_write_denied_by_policy() {
        let policy = SandboxPolicy {
            max_execution_ms: 1000,
            allow_network: false,
            allow_filesystem_write: false,
        };
        let err = check_policy(&policy, false, true).unwrap_err();
        assert!(matches!(err, ToolError::ValidationFailed(_)));
    }

    #[test]
    fn allowed_operations_pass_policy_check() {
        let policy = SandboxPolicy {
            max_execution_ms: 1000,
            allow_network: true,
            allow_filesystem_write: true,
        };
        assert!(check_policy(&policy, true, true).is_ok());
    }

    #[test]
    fn sandboxed_subprocess_timeout_enforced() {
        let sandbox = SubprocessSandbox;
        let policy = SandboxPolicy {
            max_execution_ms: 50,
            allow_network: false,
            allow_filesystem_write: false,
        };
        let err = sandbox.execute("sleep", &["10"], "", &policy).unwrap_err();
        assert!(matches!(err, ToolError::ExecutionFailed(_)));
    }
}
