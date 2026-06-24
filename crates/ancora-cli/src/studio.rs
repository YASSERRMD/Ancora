use std::sync::Arc;

use ancora_core::journal::MemoryStore;

/// Timeline for a single run, returned by GET /runs/:id/timeline.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RunTimeline {
    pub run_id: String,
    pub events: Vec<String>,
}

/// Response for POST /runs/:id/replay.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ReplayResponse {
    pub run_id: String,
    pub status: String,
}

/// Local HTTP server exposing run timelines and replay.
pub struct StudioServer {
    listener: std::net::TcpListener,
    store: Arc<MemoryStore>,
}

impl StudioServer {
    /// Bind to the given port on localhost. Pass 0 for an OS-assigned port.
    pub fn bind(port: u16, store: Arc<MemoryStore>) -> std::io::Result<Self> {
        let listener = std::net::TcpListener::bind(format!("127.0.0.1:{}", port))?;
        Ok(Self { listener, store })
    }

    /// Return the port the server is listening on.
    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
