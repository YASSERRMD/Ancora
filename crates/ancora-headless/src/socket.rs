//! Local Unix socket API for the headless agent.
//!
//! Provides a request/response protocol over a local Unix domain socket,
//! allowing local processes to interact with the Ancora agent without any
//! network exposure.

use std::collections::HashMap;

/// Default path for the headless agent Unix socket.
pub const DEFAULT_SOCKET_PATH: &str = "/run/ancora/agent.sock";

/// An API request sent over the local socket.
#[derive(Debug, Clone)]
pub struct SocketRequest {
    pub id: u64,
    pub method: String,
    pub params: HashMap<String, String>,
}

impl SocketRequest {
    pub fn new(id: u64, method: impl Into<String>) -> Self {
        SocketRequest {
            id,
            method: method.into(),
            params: HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }
}

/// An API response returned over the local socket.
#[derive(Debug, Clone)]
pub struct SocketResponse {
    pub id: u64,
    pub ok: bool,
    pub body: String,
    pub error: Option<String>,
}

impl SocketResponse {
    pub fn success(id: u64, body: impl Into<String>) -> Self {
        SocketResponse {
            id,
            ok: true,
            body: body.into(),
            error: None,
        }
    }

    pub fn error(id: u64, err: impl Into<String>) -> Self {
        SocketResponse {
            id,
            ok: false,
            body: String::new(),
            error: Some(err.into()),
        }
    }
}

/// Methods supported by the socket API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiMethod {
    /// Probe whether the agent is alive.
    Ping,
    /// Run an agent task (prompt -> response).
    Run,
    /// Query agent status.
    Status,
    /// List loaded models.
    ListModels,
    /// Reload configuration.
    ReloadConfig,
    /// Graceful shutdown.
    Shutdown,
    /// Unknown method.
    Unknown(String),
}

impl From<&str> for ApiMethod {
    fn from(s: &str) -> Self {
        match s {
            "ping" => ApiMethod::Ping,
            "run" => ApiMethod::Run,
            "status" => ApiMethod::Status,
            "list_models" => ApiMethod::ListModels,
            "reload_config" => ApiMethod::ReloadConfig,
            "shutdown" => ApiMethod::Shutdown,
            other => ApiMethod::Unknown(other.to_string()),
        }
    }
}

impl std::fmt::Display for ApiMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiMethod::Ping => write!(f, "ping"),
            ApiMethod::Run => write!(f, "run"),
            ApiMethod::Status => write!(f, "status"),
            ApiMethod::ListModels => write!(f, "list_models"),
            ApiMethod::ReloadConfig => write!(f, "reload_config"),
            ApiMethod::Shutdown => write!(f, "shutdown"),
            ApiMethod::Unknown(s) => write!(f, "unknown({})", s),
        }
    }
}

/// A simple in-process socket API dispatcher (no actual I/O).
/// In production this would wrap a UnixListener.
pub struct SocketApiHandler {
    pub socket_path: String,
    pub loaded_models: Vec<String>,
    pub status: String,
    request_log: Vec<(u64, String)>,
}

impl SocketApiHandler {
    pub fn new(socket_path: impl Into<String>) -> Self {
        SocketApiHandler {
            socket_path: socket_path.into(),
            loaded_models: Vec::new(),
            status: "ready".to_string(),
            request_log: Vec::new(),
        }
    }

    /// Dispatches a request and returns a response.
    pub fn handle(&mut self, req: SocketRequest) -> SocketResponse {
        let method = ApiMethod::from(req.method.as_str());
        self.request_log.push((req.id, req.method.clone()));
        match method {
            ApiMethod::Ping => SocketResponse::success(req.id, "pong"),
            ApiMethod::Status => SocketResponse::success(req.id, &self.status),
            ApiMethod::ListModels => {
                let list = self.loaded_models.join(",");
                SocketResponse::success(req.id, list)
            }
            ApiMethod::Run => {
                let prompt = req.params.get("prompt").cloned().unwrap_or_default();
                if prompt.is_empty() {
                    SocketResponse::error(req.id, "missing param: prompt")
                } else {
                    SocketResponse::success(req.id, format!("result for: {}", prompt))
                }
            }
            ApiMethod::ReloadConfig => SocketResponse::success(req.id, "config reloaded"),
            ApiMethod::Shutdown => {
                self.status = "stopping".to_string();
                SocketResponse::success(req.id, "shutting down")
            }
            ApiMethod::Unknown(m) => {
                SocketResponse::error(req.id, format!("unknown method: {}", m))
            }
        }
    }

    pub fn request_count(&self) -> usize {
        self.request_log.len()
    }
}

/// Configuration for the socket API.
#[derive(Debug, Clone)]
pub struct SocketConfig {
    pub path: String,
    pub backlog: u32,
    pub timeout_ms: u64,
    pub max_connections: usize,
}

impl Default for SocketConfig {
    fn default() -> Self {
        SocketConfig {
            path: DEFAULT_SOCKET_PATH.to_string(),
            backlog: 128,
            timeout_ms: 5000,
            max_connections: 16,
        }
    }
}

impl SocketConfig {
    /// Returns true if the socket path is an abstract namespace path.
    pub fn is_abstract(&self) -> bool {
        self.path.starts_with('@')
    }

    /// Returns true if the socket path is in /run (tmpfs).
    pub fn is_tmpfs(&self) -> bool {
        self.path.starts_with("/run/")
    }
}
