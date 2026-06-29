/// Headless config profile for the Ancora agent.
///
/// A self-contained configuration that controls all headless OS integration
/// parameters: model paths, cgroup limits, socket path, network policy, and
/// boot targets.

use serde::{Deserialize, Serialize};

/// The complete headless configuration profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadlessConfig {
    /// Profile name (e.g., "production", "minimal", "dev").
    pub profile: String,
    /// Path to the Unix socket for the local API.
    pub socket_path: String,
    /// Paths to model files to preload on boot.
    pub model_paths: Vec<String>,
    /// Memory limit in MB for the cgroup.
    pub cgroup_memory_mb: u64,
    /// CPU quota percentage for the cgroup.
    pub cgroup_cpu_percent: u8,
    /// Maximum allowed boot-to-ready time in milliseconds.
    pub boot_target_ms: u64,
    /// Whether external network is disabled.
    pub deny_external_network: bool,
    /// Maximum restart attempts before giving up.
    pub max_restarts: u32,
    /// Whether to write a PID file.
    pub write_pid_file: bool,
    /// Path for the PID file.
    pub pid_file_path: String,
}

impl Default for HeadlessConfig {
    fn default() -> Self {
        HeadlessConfig {
            profile: "default".to_string(),
            socket_path: "/run/ancora/agent.sock".to_string(),
            model_paths: Vec::new(),
            cgroup_memory_mb: 4096,
            cgroup_cpu_percent: 80,
            boot_target_ms: 5000,
            deny_external_network: true,
            max_restarts: 10,
            write_pid_file: true,
            pid_file_path: "/run/ancora/agent.pid".to_string(),
        }
    }
}

impl HeadlessConfig {
    pub fn new(profile: impl Into<String>) -> Self {
        HeadlessConfig {
            profile: profile.into(),
            ..Default::default()
        }
    }

    pub fn with_socket(mut self, path: impl Into<String>) -> Self {
        self.socket_path = path.into();
        self
    }

    pub fn with_model(mut self, path: impl Into<String>) -> Self {
        self.model_paths.push(path.into());
        self
    }

    pub fn with_memory_mb(mut self, mb: u64) -> Self {
        self.cgroup_memory_mb = mb;
        self
    }

    pub fn with_cpu_percent(mut self, pct: u8) -> Self {
        self.cgroup_cpu_percent = pct;
        self
    }

    pub fn deny_network(mut self, deny: bool) -> Self {
        self.deny_external_network = deny;
        self
    }

    /// Serializes the config to JSON.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Deserializes a config from JSON.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    /// Validates the configuration and returns any error messages.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.socket_path.is_empty() {
            errors.push("socket_path must not be empty".to_string());
        }
        if self.cgroup_cpu_percent == 0 || self.cgroup_cpu_percent > 100 {
            errors.push("cgroup_cpu_percent must be 1-100".to_string());
        }
        if self.cgroup_memory_mb < 64 {
            errors.push("cgroup_memory_mb must be >= 64".to_string());
        }
        if self.boot_target_ms == 0 {
            errors.push("boot_target_ms must be > 0".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

/// Predefined configuration profiles.
pub mod profiles {
    use super::HeadlessConfig;

    /// Minimal profile for embedded/constrained hardware.
    pub fn minimal() -> HeadlessConfig {
        HeadlessConfig::new("minimal")
            .with_memory_mb(512)
            .with_cpu_percent(50)
            .deny_network(true)
    }

    /// Standard production profile.
    pub fn standard() -> HeadlessConfig {
        HeadlessConfig::new("standard")
            .with_memory_mb(4096)
            .with_cpu_percent(80)
            .deny_network(true)
    }

    /// Development profile (network allowed for debugging).
    pub fn dev() -> HeadlessConfig {
        HeadlessConfig::new("dev")
            .with_memory_mb(2048)
            .with_cpu_percent(90)
            .deny_network(false)
    }
}
