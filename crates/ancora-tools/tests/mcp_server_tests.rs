use std::sync::Arc;
use tokio::sync::oneshot;

use ancora_tools::{
    error::ToolError,
    mcp_server::McpServer,
    registry::ToolRegistry,
    tool::{Tool, ToolEffect},
};

struct MathTool;

impl Tool for MathTool {
    fn name(&self) -> &str {
        "multiply"
    }
    fn description(&self) -> &str {
        "multiplies two numbers"
    }
    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!({ "type": "object", "required": ["x", "y"] })
    }
    fn effect(&self) -> ToolEffect {
        ToolEffect::ReadOnly
    }
    fn call(&self, input: &serde_json::Value) -> Result<serde_json::Value, ToolError> {
        let x = input["x"].as_f64().unwrap_or(0.0);
        let y = input["y"].as_f64().unwrap_or(0.0);
        Ok(serde_json::json!({ "product": x * y }))
    }
}

async fn spawn_server(
    registry: ToolRegistry,
    token: Option<&str>,
) -> (std::net::SocketAddr, oneshot::Sender<()>) {
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let bound = listener.local_addr().unwrap();
    drop(listener);

    let mut srv = McpServer::new(registry);
    if let Some(t) = token {
        srv = srv.with_token(t);
    }

    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(async move { srv.serve(bound, rx).await.unwrap() });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    (bound, tx)
}

async fn post(addr: std::net::SocketAddr, body: &str, auth: Option<&str>) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let auth_header = auth
        .map(|t| format!("Authorization: Bearer {}\r\n", t))
        .unwrap_or_default();
    let req = format!(
        "POST / HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{}\r\n{}",
        addr,
        body.len(),
        auth_header,
        body
    );
    stream.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    String::from_utf8(buf).unwrap()
}

fn make_registry() -> ToolRegistry {
    let mut r = ToolRegistry::new();
    r.register(Arc::new(MathTool));
    r
}

#[tokio::test]
async fn mcp_client_lists_ancora_tools() {
    let (addr, tx) = spawn_server(make_registry(), None).await;

    let body = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#;
    let response = post(addr, body, None).await;

    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("multiply"));
    assert!(response.contains("multiplies two numbers"));

    let _ = tx.send(());
}

#[tokio::test]
async fn mcp_client_calls_ancora_tool() {
    let (addr, tx) = spawn_server(make_registry(), None).await;

    let body = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"multiply","arguments":{"x":6,"y":7}}}"#;
    let response = post(addr, body, None).await;

    assert!(
        response.contains("42"),
        "expected product 42 in: {}",
        response
    );

    let _ = tx.send(());
}

#[tokio::test]
async fn mcp_server_rejects_unauthenticated_request() {
    let (addr, tx) = spawn_server(make_registry(), Some("s3cr3t")).await;

    let body = r#"{"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}}"#;
    let response = post(addr, body, None).await;

    assert!(
        response.starts_with("HTTP/1.1 401"),
        "expected 401, got: {}",
        &response[..60.min(response.len())]
    );

    let _ = tx.send(());
}

#[tokio::test]
async fn mcp_server_accepts_authenticated_request() {
    let (addr, tx) = spawn_server(make_registry(), Some("s3cr3t")).await;

    let body = r#"{"jsonrpc":"2.0","id":4,"method":"tools/list","params":{}}"#;
    let response = post(addr, body, Some("s3cr3t")).await;

    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "expected 200, got: {}",
        &response[..60.min(response.len())]
    );
    assert!(response.contains("multiply"));

    let _ = tx.send(());
}

#[tokio::test]
async fn mcp_server_returns_error_for_unknown_method() {
    let (addr, tx) = spawn_server(make_registry(), None).await;

    let body = r#"{"jsonrpc":"2.0","id":5,"method":"tools/unknown","params":{}}"#;
    let response = post(addr, body, None).await;

    assert!(
        response.contains("error"),
        "expected error in: {}",
        response
    );
    assert!(response.contains("unknown method"));

    let _ = tx.send(());
}

#[tokio::test]
async fn mcp_server_returns_error_for_missing_tool() {
    let (addr, tx) = spawn_server(make_registry(), None).await;

    let body = r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"ghost","arguments":{}}}"#;
    let response = post(addr, body, None).await;

    assert!(
        response.contains("error"),
        "expected error in: {}",
        response
    );

    let _ = tx.send(());
}
