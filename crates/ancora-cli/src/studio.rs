use std::io::Write;
use std::net::TcpStream;
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

fn write_response(stream: &mut TcpStream, status: u16, body: &str) {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Internal Server Error",
    };
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status, status_text, body.len(), body
    );
    stream.write_all(response.as_bytes()).ok();
}
