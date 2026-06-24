use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/// A single-request mock OTLP HTTP collector for tests.
pub struct MockCollector {
    pub port: u16,
    received: Arc<Mutex<Vec<String>>>,
}

impl MockCollector {
    /// Start the collector on an OS-assigned port and accept one request.
    pub fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let received: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let received2 = received.clone();
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                handle_request(&mut stream, &received2);
            }
        });
        MockCollector { port, received }
    }

    /// Returns all request bodies received.
    pub fn bodies(&self) -> Vec<String> {
        self.received.lock().unwrap().clone()
    }

    /// Block until at least one request is received (up to 2 seconds).
    pub fn wait_for_request(&self) {
        let start = std::time::Instant::now();
        while self.received.lock().unwrap().is_empty() {
            if start.elapsed().as_secs() > 2 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

fn handle_request(stream: &mut TcpStream, received: &Arc<Mutex<Vec<String>>>) {
    let mut buf = [0u8; 65536];
    let n = stream.read(&mut buf).unwrap_or(0);
    let raw = String::from_utf8_lossy(&buf[..n]).to_string();
    let body = extract_body(&raw);
    received.lock().unwrap().push(body);
    let resp = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}";
    let _ = stream.write_all(resp);
}

fn extract_body(raw: &str) -> String {
    if let Some(idx) = raw.find("\r\n\r\n") {
        raw[idx + 4..].to_string()
    } else {
        raw.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exporter::SpanEmitter;
    use crate::otlp_http::OtlpHttpExporter;
    use crate::span::Span;

    #[test]
    fn exporter_sends_spans_to_mock_collector() {
        let collector = MockCollector::start();
        let endpoint = format!("http://127.0.0.1:{}/v1/traces", collector.port);
        let exporter = OtlpHttpExporter::new(endpoint);
        exporter.emit(Span::new("test-span").set("k", "v"));
        exporter.export().unwrap();
        collector.wait_for_request();
        let bodies = collector.bodies();
        assert_eq!(bodies.len(), 1);
        assert!(bodies[0].contains("test-span"));
    }

    #[test]
    fn exporter_body_is_valid_otlp_json() {
        let collector = MockCollector::start();
        let endpoint = format!("http://127.0.0.1:{}/v1/traces", collector.port);
        let exporter = OtlpHttpExporter::new(endpoint);
        exporter.emit(Span::new("json-check").set("model", "gpt-4o"));
        exporter.export().unwrap();
        collector.wait_for_request();
        let body = &collector.bodies()[0];
        let parsed: serde_json::Value = serde_json::from_str(body).expect("body should be valid json");
        assert!(parsed.get("resourceSpans").is_some());
    }
}
