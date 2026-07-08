/// MCP server authentication rejection tests.
///
/// Verifies that every request path requires a valid bearer token when the
/// server is configured with one:
/// - No Authorization header -> 401
/// - Wrong token value -> 401
/// - Partial header ("Bearer" with no value) -> 401
/// - Token with extra whitespace padding -> 401 (rejected, not accepted)
/// - Correct token -> 200
/// - tools/call (not just tools/list) is also gated behind auth
use std::sync::Arc;

use tokio::sync::oneshot;

use ancora_tools::{
    error::ToolError,
    mcp_server::McpServer,
    registry::ToolRegistry,
    tool::{Tool, ToolEffect},
};

const TOKEN: &str = "super-secret-xyz";

struct NopTool;

impl Tool for NopTool {
    fn name(&self) -> &str {
        "nop"
    }
    fn description(&self) -> &str {
        "does nothing"
    }
    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!({ "type": "object", "required": [] })
    }
    fn effect(&self) -> ToolEffect {
        ToolEffect::ReadOnly
    }
    fn call(&self, _input: &serde_json::Value) -> Result<serde_json::Value, ToolError> {
        Ok(serde_json::json!({}))
    }
}

async fn spawn_protected() -> (std::net::SocketAddr, oneshot::Sender<()>) {
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let bound = listener.local_addr().unwrap();
    drop(listener);

    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(NopTool));

    let srv = McpServer::new(registry).with_token(TOKEN);
    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(async move { srv.serve(bound, rx).await.unwrap() });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    (bound, tx)
}

async fn http_post(addr: std::net::SocketAddr, body: &str, auth_header: Option<&str>) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let auth_line = auth_header
        .map(|h| format!("{}\r\n", h))
        .unwrap_or_default();
    let req = format!(
        "POST / HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{auth_line}\r\n{body}",
        body.len(),
    );
    stream.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    String::from_utf8(buf).unwrap()
}

fn status(resp: &str) -> u16 {
    resp.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

#[tokio::test]
async fn no_auth_header_returns_401() {
    let (addr, tx) = spawn_protected().await;
    let body = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#;
    let resp = http_post(addr, body, None).await;
    assert_eq!(
        status(&resp),
        401,
        "missing header must be 401, got: {resp}"
    );
    let _ = tx.send(());
}

#[tokio::test]
async fn wrong_token_returns_401() {
    let (addr, tx) = spawn_protected().await;
    let body = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#;
    let resp = http_post(addr, body, Some("Authorization: Bearer wrong-token")).await;
    assert_eq!(status(&resp), 401, "wrong token must be 401, got: {resp}");
    let _ = tx.send(());
}

#[tokio::test]
async fn empty_bearer_value_returns_401() {
    let (addr, tx) = spawn_protected().await;
    let body = r#"{"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}}"#;
    let resp = http_post(addr, body, Some("Authorization: Bearer ")).await;
    assert_eq!(status(&resp), 401, "empty bearer must be 401, got: {resp}");
    let _ = tx.send(());
}

#[tokio::test]
async fn padded_token_is_rejected() {
    let (addr, tx) = spawn_protected().await;
    let body = r#"{"jsonrpc":"2.0","id":4,"method":"tools/list","params":{}}"#;
    let padded = format!("Authorization: Bearer  {TOKEN} ");
    let resp = http_post(addr, body, Some(&padded)).await;
    assert_eq!(status(&resp), 401, "padded token must be 401, got: {resp}");
    let _ = tx.send(());
}

#[tokio::test]
async fn correct_token_returns_200() {
    let (addr, tx) = spawn_protected().await;
    let body = r#"{"jsonrpc":"2.0","id":5,"method":"tools/list","params":{}}"#;
    let auth = format!("Authorization: Bearer {TOKEN}");
    let resp = http_post(addr, body, Some(&auth)).await;
    assert_eq!(status(&resp), 200, "correct token must be 200, got: {resp}");
    assert!(resp.contains("nop"), "expected tool listing in body");
    let _ = tx.send(());
}

#[tokio::test]
async fn tools_call_also_requires_auth() {
    let (addr, tx) = spawn_protected().await;
    let body =
        r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"nop","arguments":{}}}"#;
    let resp = http_post(addr, body, None).await;
    assert_eq!(
        status(&resp),
        401,
        "tools/call without token must be 401, got: {resp}"
    );
    let _ = tx.send(());
}

#[tokio::test]
async fn basic_scheme_is_not_accepted() {
    let (addr, tx) = spawn_protected().await;
    let body = r#"{"jsonrpc":"2.0","id":7,"method":"tools/list","params":{}}"#;
    let auth = format!("Authorization: Basic {TOKEN}");
    let resp = http_post(addr, body, Some(&auth)).await;
    assert_eq!(status(&resp), 401, "Basic scheme must be 401, got: {resp}");
    let _ = tx.send(());
}
