use ancora_grpc::{
    agent_card::{AgentCapability, AgentCard},
    identity::{verify_card, AgentIdentity},
    task::{Task, TaskStatus},
};
use tokio::sync::oneshot;

#[test]
fn agent_card_round_trips_json() {
    let card = AgentCard::new("echo-agent", "Echoes input", "grpc://localhost:50051");
    let json = card.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["name"], "echo-agent");
    assert_eq!(parsed["endpoint"], "grpc://localhost:50051");
    let caps: Vec<String> = parsed["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    assert!(caps.contains(&"run".to_owned()));
    assert!(caps.contains(&"stream".to_owned()));
}

#[test]
fn signed_card_serialises_with_identity_fields() {
    let card = AgentCard::new("signed-agent", "Signed", "grpc://localhost:50051");
    let id = AgentIdentity::generate();
    let signed = id.attach_to(card);
    assert!(signed.identity_key.is_some());
    assert!(signed.signature.is_some());
    let json = signed.to_json();
    assert!(json.contains("identity_key"));
    assert!(json.contains("signature"));
}

#[test]
fn signed_card_verifies_after_json_round_trip() {
    let card = AgentCard::new("agent", "desc", "grpc://localhost:50051");
    let id = AgentIdentity::generate();
    let signed = id.attach_to(card);
    let json = signed.to_json();
    let restored: AgentCard = serde_json::from_str(&json).unwrap();
    assert!(verify_card(&restored));
}

#[test]
fn task_full_lifecycle() {
    let t = Task::new("abc", "summarise this document")
        .running()
        .completed("The document is about Rust.");
    assert_eq!(t.status, TaskStatus::Completed);
    assert_eq!(t.output.as_deref(), Some("The document is about Rust."));
    assert!(t.is_terminal());
}

#[test]
fn agent_card_capabilities_include_tools_when_set() {
    let mut card = AgentCard::new("tool-agent", "Has tools", "grpc://localhost:50051");
    card.capabilities.push(AgentCapability::Tools);
    assert!(card.capabilities.contains(&AgentCapability::Tools));
    let json = card.to_json();
    assert!(json.contains("tools"));
}

#[tokio::test]
async fn a2a_client_fetches_agent_card_over_http() {
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let bound = listener.local_addr().unwrap();
    drop(listener);

    let card = AgentCard::new(
        "http-agent",
        "Served over HTTP",
        format!("grpc://{}", bound),
    );
    let id = AgentIdentity::generate();
    let signed = id.attach_to(card);

    let (tx, rx) = oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        signed.serve(bound, rx).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let stream = tokio::net::TcpStream::connect(bound).await.unwrap();
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = stream;
    let req = format!(
        "GET /.well-known/agent.json HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        bound
    );
    stream.write_all(req.as_bytes()).await.unwrap();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    let response = String::from_utf8(buf).unwrap();

    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "expected 200, got: {}",
        &response[..50.min(response.len())]
    );
    assert!(response.contains("http-agent"));
    assert!(response.contains("identity_key"));
    assert!(response.contains("signature"));

    let _ = tx.send(());
    let _ = server.await;
}

#[tokio::test]
async fn a2a_unknown_path_returns_404() {
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let bound = listener.local_addr().unwrap();
    drop(listener);

    let card = AgentCard::new("agent", "desc", "grpc://localhost:50051");
    let (tx, rx) = oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        card.serve(bound, rx).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let stream = tokio::net::TcpStream::connect(bound).await.unwrap();
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = stream;
    let req = format!(
        "GET /unknown HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        bound
    );
    stream.write_all(req.as_bytes()).await.unwrap();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    let response = String::from_utf8(buf).unwrap();
    assert!(
        response.starts_with("HTTP/1.1 404"),
        "expected 404, got: {}",
        &response[..50.min(response.len())]
    );

    let _ = tx.send(());
    let _ = server.await;
}
