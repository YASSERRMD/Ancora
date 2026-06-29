/// Init service integration for the headless OS agent.
///
/// Provides abstractions for registering Ancora as a system service
/// (systemd-compatible unit descriptor, PID file management, readiness
/// signalling via sd_notify-style protocol).

use std::collections::HashMap;

/// The name used to register Ancora with the init system.
pub const SERVICE_NAME: &str = "ancora-agent";

/// Represents the current lifecycle state of the init service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    /// Service is not yet started.
    Idle,
    /// Service is starting up (loading config, preloading models).
    Starting,
    /// Service is fully ready to accept requests.
    Ready,
    /// Service is shutting down gracefully.
    Stopping,
    /// Service has stopped.
    Stopped,
    /// Service encountered a fatal error.
    Failed(String),
}

impl std::fmt::Display for ServiceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceState::Idle => write!(f, "idle"),
            ServiceState::Starting => write!(f, "starting"),
            ServiceState::Ready => write!(f, "ready"),
            ServiceState::Stopping => write!(f, "stopping"),
            ServiceState::Stopped => write!(f, "stopped"),
            ServiceState::Failed(e) => write!(f, "failed: {}", e),
        }
    }
}

/// Systemd-compatible unit descriptor for the Ancora agent service.
#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub after: Vec<String>,
    pub wanted_by: String,
    pub exec_start: String,
    pub restart_policy: RestartPolicy,
    pub environment: HashMap<String, String>,
}

/// Restart policy for the service unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnFailure,
    Always,
    UnlessStopped,
}

impl std::fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestartPolicy::No => write!(f, "no"),
            RestartPolicy::OnFailure => write!(f, "on-failure"),
            RestartPolicy::Always => write!(f, "always"),
            RestartPolicy::UnlessStopped => write!(f, "unless-stopped"),
        }
    }
}

impl Default for ServiceUnit {
    fn default() -> Self {
        ServiceUnit {
            name: SERVICE_NAME.to_string(),
            description: "Ancora Headless Agent Service".to_string(),
            after: vec!["network.target".to_string(), "local-fs.target".to_string()],
            wanted_by: "multi-user.target".to_string(),
            exec_start: "/usr/local/bin/ancora-headless".to_string(),
            restart_policy: RestartPolicy::OnFailure,
            environment: HashMap::new(),
        }
    }
}

impl ServiceUnit {
    /// Renders the unit file content as a string.
    pub fn render(&self) -> String {
        let after = self.after.join(" ");
        let restart = self.restart_policy.to_string();
        let mut env_lines = String::new();
        for (k, v) in &self.environment {
            env_lines.push_str(&format!("Environment=\"{}={}\"\n", k, v));
        }
        format!(
            "[Unit]\nDescription={}\nAfter={}\n\n[Service]\nExecStart={}\nRestart={}\n{}Type=notify\n\n[Install]\nWantedBy={}\n",
            self.description, after, self.exec_start, restart, env_lines, self.wanted_by
        )
    }

    /// Returns true if the unit is configured for automatic restart.
    pub fn auto_restarts(&self) -> bool {
        !matches!(self.restart_policy, RestartPolicy::No)
    }
}

/// Manages the PID file for the headless service.
pub struct PidFile {
    pub path: String,
    pub pid: u32,
}

impl PidFile {
    pub fn new(path: impl Into<String>, pid: u32) -> Self {
        PidFile { path: path.into(), pid }
    }

    /// Returns the PID file content as a string.
    pub fn content(&self) -> String {
        format!("{}\n", self.pid)
    }
}

/// Signals readiness to the init system (sd_notify-compatible).
pub fn notify_ready(status: &ServiceState) -> String {
    match status {
        ServiceState::Ready => "READY=1\nSTATUS=agent ready\n".to_string(),
        ServiceState::Starting => "STATUS=starting\n".to_string(),
        ServiceState::Stopping => "STOPPING=1\nSTATUS=stopping\n".to_string(),
        other => format!("STATUS={}\n", other),
    }
}

/// Tracks state transitions for the service lifecycle.
pub struct ServiceLifecycle {
    state: ServiceState,
    history: Vec<ServiceState>,
}

impl ServiceLifecycle {
    pub fn new() -> Self {
        ServiceLifecycle {
            state: ServiceState::Idle,
            history: vec![ServiceState::Idle],
        }
    }

    pub fn transition(&mut self, next: ServiceState) {
        self.state = next.clone();
        self.history.push(next);
    }

    pub fn current(&self) -> &ServiceState {
        &self.state
    }

    pub fn history(&self) -> &[ServiceState] {
        &self.history
    }

    pub fn is_ready(&self) -> bool {
        self.state == ServiceState::Ready
    }
}

impl Default for ServiceLifecycle {
    fn default() -> Self {
        Self::new()
    }
}
