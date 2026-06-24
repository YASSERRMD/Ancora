use std::io::{BufRead, BufReader, Write};
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

    fn list_runs(&self) -> (u16, String) {
        (200, r#"{"runs":[]}"#.into())
    }

    fn run_timeline(&self, run_id: &str) -> (u16, String) {
        if run_id.is_empty() {
            return (400, r#"{"error":"missing run id"}"#.into());
        }
        let timeline = RunTimeline { run_id: run_id.into(), events: vec![] };
        let body = serde_json::to_string(&timeline).unwrap_or_default();
        (200, body)
    }

    fn replay_run(&self, run_id: &str) -> (u16, String) {
        if run_id.is_empty() {
            return (400, r#"{"error":"missing run id"}"#.into());
        }
        let resp = ReplayResponse { run_id: run_id.into(), status: "ok".into() };
        let body = serde_json::to_string(&resp).unwrap_or_default();
        (200, body)
    }

    fn route(&self, method: &str, path: &str) -> (u16, String) {
        match (method, path) {
            ("GET", "/runs") => self.list_runs(),
            ("GET", p) if p.starts_with("/runs/") && p.ends_with("/timeline") => {
                let id = p.trim_start_matches("/runs/").trim_end_matches("/timeline");
                self.run_timeline(id)
            }
            ("POST", p) if p.starts_with("/runs/") && p.ends_with("/replay") => {
                let id = p.trim_start_matches("/runs/").trim_end_matches("/replay");
                self.replay_run(id)
            }
            _ => (404, r#"{"error":"not found"}"#.into()),
        }
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
