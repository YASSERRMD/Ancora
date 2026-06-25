use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use crate::registry::ToolRegistry;

/// A minimal MCP server that exposes a `ToolRegistry` over HTTP JSON-RPC 2.0.
///
/// Supports `tools/list` and `tools/call` methods.
/// Optionally enforces a static bearer token on every request.
pub struct McpServer {
    registry: Arc<ToolRegistry>,
    token: Option<String>,
}

impl McpServer {
    pub fn new(registry: ToolRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
            token: None,
        }
    }

    /// Require `Authorization: Bearer <token>` on every request.
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Bind to `addr` and serve until `shutdown` fires.
    pub async fn serve(
        self,
        addr: std::net::SocketAddr,
        shutdown: oneshot::Receiver<()>,
    ) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        let server = Arc::new(self);
        tokio::select! {
            _ = async {
                loop {
                    let (stream, _) = match listener.accept().await {
                        Ok(s) => s,
                        Err(_) => break,
                    };
                    let srv = Arc::clone(&server);
                    tokio::spawn(handle_connection(stream, srv));
                }
            } => {}
            _ = shutdown => {}
        }
        Ok(())
    }
}

async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    server: Arc<McpServer>,
) {
    let mut buf = [0u8; 16384];
    let n = match stream.read(&mut buf).await {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    let raw = &buf[..n];

    // Split HTTP headers from body.
    let Some(header_end) = find_header_end(raw) else { return };
    let headers = String::from_utf8_lossy(&raw[..header_end]);
    let body = &raw[header_end..];

    // Auth check.
    if let Some(required) = &server.token {
        if !bearer_matches(headers.as_ref(), required) {
            let resp = "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(resp.as_bytes()).await.ok();
            return;
        }
    }

    // Only accept POST.
    if !headers.starts_with("POST ") {
        not_found(&mut stream).await;
        return;
    }

    let Ok(request) = serde_json::from_slice::<serde_json::Value>(body) else {
        rpc_error(&mut stream, -32700, "parse error").await;
        return;
    };

    let method = request["method"].as_str().unwrap_or("");
    let id = request.get("id").cloned().unwrap_or(serde_json::Value::Null);

    let result = match method {
        "tools/list" => handle_list(&server.registry),
        "tools/call" => handle_call(&server.registry, &request["params"]),
        _ => Err(format!("unknown method: {}", method)),
    };

    let response_body = match result {
        Ok(r) => serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": r }),
        Err(e) => serde_json::json!({
            "jsonrpc": "2.0", "id": id,
            "error": { "code": -32603, "message": e }
        }),
    };
    let body_str = serde_json::to_string(&response_body).unwrap_or_default();
    let http = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body_str.len(),
        body_str
    );
    stream.write_all(http.as_bytes()).await.ok();
}

fn handle_list(registry: &ToolRegistry) -> Result<serde_json::Value, String> {
    let tools: Vec<serde_json::Value> = registry
        .list()
        .iter()
        .map(|t| {
            serde_json::json!({
                "name": t.name(),
                "description": t.description(),
                "inputSchema": t.input_schema(),
            })
        })
        .collect();
    Ok(serde_json::json!({ "tools": tools }))
}

fn handle_call(registry: &ToolRegistry, params: &serde_json::Value) -> Result<serde_json::Value, String> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| "missing tool name".to_owned())?;
    let args = &params["arguments"];
    registry
        .call(name, args)
        .map_err(|e| e.to_string())
}

/// Return `true` when the HTTP headers contain `Authorization: Bearer <token>`.
pub(crate) fn bearer_matches(headers: &str, token: &str) -> bool {
    let expected = format!("bearer {}", token.to_ascii_lowercase());
    headers.lines().any(|l| {
        l.to_ascii_lowercase()
            .trim_start_matches("authorization:")
            .trim()
            .to_ascii_lowercase()
            == expected
    })
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|p| p + 4)
}

async fn not_found(stream: &mut tokio::net::TcpStream) {
    stream
        .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
        .await
        .ok();
}

async fn rpc_error(stream: &mut tokio::net::TcpStream, code: i32, msg: &str) {
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": null,
        "error": { "code": code, "message": msg }
    });
    let body_str = serde_json::to_string(&body).unwrap_or_default();
    let http = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body_str.len(),
        body_str
    );
    stream.write_all(http.as_bytes()).await.ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{Tool, ToolEffect};
    use crate::error::ToolError;

    struct AddTool;

    impl Tool for AddTool {
        fn name(&self) -> &str { "add" }
        fn description(&self) -> &str { "adds two numbers" }
        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({ "type": "object", "required": ["a", "b"] })
        }
        fn effect(&self) -> ToolEffect { ToolEffect::ReadOnly }
        fn call(&self, input: &serde_json::Value) -> Result<serde_json::Value, ToolError> {
            let a = input["a"].as_f64().unwrap_or(0.0);
            let b = input["b"].as_f64().unwrap_or(0.0);
            Ok(serde_json::json!({ "sum": a + b }))
        }
    }

    fn make_registry() -> ToolRegistry {
        let mut r = ToolRegistry::new();
        r.register(Arc::new(AddTool));
        r
    }

    #[test]
    fn handle_list_returns_registered_tools() {
        let registry = make_registry();
        let result = handle_list(&registry).unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "add");
        assert_eq!(tools[0]["description"], "adds two numbers");
    }

    #[test]
    fn handle_call_returns_tool_output() {
        let registry = make_registry();
        let params = serde_json::json!({ "name": "add", "arguments": { "a": 3, "b": 4 } });
        let result = handle_call(&registry, &params).unwrap();
        assert_eq!(result["sum"], 7.0);
    }

    #[test]
    fn handle_call_unknown_tool_returns_error() {
        let registry = make_registry();
        let params = serde_json::json!({ "name": "ghost", "arguments": {} });
        assert!(handle_call(&registry, &params).is_err());
    }

    #[test]
    fn find_header_end_locates_crlf_separator() {
        let req = b"POST / HTTP/1.1\r\nHost: x\r\n\r\nbody";
        let pos = find_header_end(req).unwrap();
        assert_eq!(&req[pos..], b"body");
    }

    #[test]
    fn bearer_matches_correct_token() {
        let headers = "POST / HTTP/1.1\r\nAuthorization: Bearer secret123\r\nHost: x";
        assert!(bearer_matches(headers, "secret123"));
    }

    #[test]
    fn bearer_matches_rejects_wrong_token() {
        let headers = "POST / HTTP/1.1\r\nAuthorization: Bearer wrong\r\nHost: x";
        assert!(!bearer_matches(headers, "secret123"));
    }

    #[test]
    fn bearer_matches_rejects_missing_header() {
        let headers = "POST / HTTP/1.1\r\nHost: x";
        assert!(!bearer_matches(headers, "secret123"));
    }

    #[test]
    fn bearer_matches_is_case_insensitive() {
        let headers = "POST / HTTP/1.1\r\nauthorization: bearer SECRET123\r\nHost: x";
        assert!(bearer_matches(headers, "secret123"));
    }

    #[test]
    fn bearer_matches_rejects_non_bearer_scheme() {
        let headers = "POST / HTTP/1.1\r\nAuthorization: Basic secret123\r\nHost: x";
        assert!(!bearer_matches(headers, "secret123"));
    }
}
