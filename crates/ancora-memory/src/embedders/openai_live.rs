//! Live, `ureq`-backed OpenAI-compatible embedder.
//!
//! Requires the `openai-embed` feature. `OpenAiEmbedder` (in `openai.rs`)
//! is a deterministic, network-free stub used by default so existing
//! offline callers and tests keep working; `LiveOpenAiEmbedder` sends real
//! requests to `config.embeddings_url()` and parses the real response.

use crate::embedders::embedder::{
    parse_openai_batch_embeddings, parse_openai_embedding, EmbedError, EmbedResult, Embedder,
    Embedding,
};
use crate::embedders::openai::{batch_request_body, request_body, OpenAiEmbedConfig};

/// An `Embedder` backed by a real HTTP call to an OpenAI-compatible
/// `/v1/embeddings` endpoint (OpenAI, Azure OpenAI, NVIDIA NIM, Ollama's
/// OpenAI shim, or any other compatible server).
pub struct LiveOpenAiEmbedder {
    config: OpenAiEmbedConfig,
    /// Output dimensionality reported by `dims()`. Not verified against
    /// the actual response; callers should set this to `config.dimensions`
    /// or the known model default.
    dims: usize,
}

impl LiveOpenAiEmbedder {
    pub fn new(config: OpenAiEmbedConfig, dims: usize) -> Self {
        Self { config, dims }
    }

    fn post(&self, body: serde_json::Value) -> EmbedResult<serde_json::Value> {
        let mut req = ureq::post(&self.config.embeddings_url())
            .timeout(std::time::Duration::from_secs(self.config.timeout_secs));
        if !self.config.api_key.is_empty() {
            req = req.set("Authorization", &self.config.auth_header());
        }
        match req.send_json(body) {
            Ok(resp) => resp
                .into_json::<serde_json::Value>()
                .map_err(|e| EmbedError::ParseError(e.to_string())),
            Err(ureq::Error::Status(status, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Err(EmbedError::HttpError { status, body })
            }
            Err(ureq::Error::Transport(t)) => Err(EmbedError::Transient(t.to_string())),
        }
    }
}

impl Embedder for LiveOpenAiEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let body = request_body(&self.config, text);
        let resp = self.post(body)?;
        parse_openai_embedding(&resp)
    }

    fn embed_batch(&self, texts: &[&str]) -> EmbedResult<Vec<Embedding>> {
        let body = batch_request_body(&self.config, texts);
        let resp = self.post(body)?;
        parse_openai_batch_embeddings(&resp)
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }

    fn dims(&self) -> usize {
        self.dims
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;

    /// Start a local, offline mock HTTP server that reads one request,
    /// hands its raw bytes back over `request_tx`, and responds with
    /// `status_line` + `response_body`. Returns the server's base URL.
    fn mock_server(
        status_line: &'static str,
        response_body: &'static str,
    ) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let request = read_full_http_request(&mut stream);
                let _ = tx.send(request);
                let response = format!(
                    "{status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{response_body}",
                    response_body.len()
                );
                let _ = stream.write_all(response.as_bytes());
            }
        });
        (format!("http://{addr}"), rx)
    }

    /// Read a complete HTTP/1.1 request off `stream`: headers first (a
    /// single `read()` is not guaranteed to return the whole request, since
    /// a client may write headers and body in separate TCP segments), then
    /// exactly `Content-Length` more bytes if a body is present.
    fn read_full_http_request(stream: &mut std::net::TcpStream) -> String {
        let mut buf = Vec::new();
        let mut chunk = [0u8; 4096];
        let headers_end = loop {
            let n = stream.read(&mut chunk).unwrap_or(0);
            if n == 0 {
                break None;
            }
            buf.extend_from_slice(&chunk[..n]);
            if let Some(pos) = find_subslice(&buf, b"\r\n\r\n") {
                break Some(pos + 4);
            }
        };
        let Some(headers_end) = headers_end else {
            return String::from_utf8_lossy(&buf).into_owned();
        };
        let headers = String::from_utf8_lossy(&buf[..headers_end]);
        let content_length: usize = headers
            .lines()
            .find_map(|l| {
                l.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .map(|v| v.trim().to_owned())
            })
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        while buf.len() < headers_end + content_length {
            let n = stream.read(&mut chunk).unwrap_or(0);
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
        }
        String::from_utf8_lossy(&buf).into_owned()
    }

    fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }

    fn config_for(base_url: String) -> OpenAiEmbedConfig {
        let mut cfg = OpenAiEmbedConfig::new("test-key", "text-embedding-3-small");
        cfg.base_url = base_url;
        cfg
    }

    #[test]
    fn embed_sends_request_and_parses_response() {
        let (base_url, rx) = mock_server(
            "HTTP/1.1 200 OK",
            r#"{"data":[{"embedding":[0.1,0.2,0.3]}],"usage":{"prompt_tokens":2,"total_tokens":2}}"#,
        );
        let embedder = LiveOpenAiEmbedder::new(config_for(base_url), 3);
        let v = embedder.embed("hello world").unwrap();
        assert_eq!(v.len(), 3);
        assert!((v[0] - 0.1).abs() < 1e-6);

        let request = rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        assert!(
            request.starts_with("POST /embeddings"),
            "request: {request}"
        );
        assert!(
            request.contains("Authorization: Bearer test-key"),
            "request: {request}"
        );
        assert!(request.contains("hello world"), "request: {request}");
    }

    #[test]
    fn embed_batch_sends_all_texts_and_parses_all_vectors() {
        let (base_url, rx) = mock_server(
            "HTTP/1.1 200 OK",
            r#"{"data":[{"embedding":[1.0,0.0]},{"embedding":[0.0,1.0]}]}"#,
        );
        let embedder = LiveOpenAiEmbedder::new(config_for(base_url), 2);
        let vs = embedder.embed_batch(&["a", "b"]).unwrap();
        assert_eq!(vs.len(), 2);
        assert_eq!(vs[0], vec![1.0, 0.0]);
        assert_eq!(vs[1], vec![0.0, 1.0]);

        let request = rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        assert!(request.contains("\"a\""), "request: {request}");
        assert!(request.contains("\"b\""), "request: {request}");
    }

    #[test]
    fn embed_propagates_http_error_status() {
        let (base_url, _rx) = mock_server(
            "HTTP/1.1 429 Too Many Requests",
            r#"{"error":"rate limited"}"#,
        );
        let embedder = LiveOpenAiEmbedder::new(config_for(base_url), 3);
        let err = embedder.embed("hello").unwrap_err();
        assert!(matches!(err, EmbedError::HttpError { status: 429, .. }));
    }

    #[test]
    fn embed_propagates_server_error_status() {
        let (base_url, _rx) =
            mock_server("HTTP/1.1 500 Internal Server Error", r#"{"error":"boom"}"#);
        let embedder = LiveOpenAiEmbedder::new(config_for(base_url), 3);
        let err = embedder.embed("hello").unwrap_err();
        assert!(matches!(err, EmbedError::HttpError { status: 500, .. }));
    }

    #[test]
    fn embed_unreachable_endpoint_is_transient() {
        // Port 1 on loopback: nothing listens there, so the connection is
        // refused immediately without needing a mock server or a timeout.
        let embedder = LiveOpenAiEmbedder::new(config_for("http://127.0.0.1:1".to_owned()), 3);
        let err = embedder.embed("hello").unwrap_err();
        assert!(err.is_transient(), "expected transient error, got {err:?}");
    }

    #[test]
    fn embed_omits_auth_header_when_api_key_empty() {
        let (base_url, rx) = mock_server("HTTP/1.1 200 OK", r#"{"data":[{"embedding":[1.0]}]}"#);
        let mut cfg = config_for(base_url);
        cfg.api_key = String::new();
        let embedder = LiveOpenAiEmbedder::new(cfg, 1);
        embedder.embed("hello").unwrap();

        let request = rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        assert!(!request.contains("Authorization"), "request: {request}");
    }

    #[test]
    fn embed_includes_input_type_when_configured() {
        let (base_url, rx) = mock_server("HTTP/1.1 200 OK", r#"{"data":[{"embedding":[1.0]}]}"#);
        let cfg =
            config_for(base_url).with_input_type(crate::embedders::openai::input_type::PASSAGE);
        let embedder = LiveOpenAiEmbedder::new(cfg, 1);
        embedder.embed("hello").unwrap();

        let request = rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
        assert!(
            request.contains("\"input_type\":\"passage\""),
            "request: {request}"
        );
    }

    #[test]
    fn model_name_and_dims_reflect_config() {
        let embedder = LiveOpenAiEmbedder::new(config_for("http://127.0.0.1:1".to_owned()), 1536);
        assert_eq!(embedder.model_name(), "text-embedding-3-small");
        assert_eq!(embedder.dims(), 1536);
    }
}
