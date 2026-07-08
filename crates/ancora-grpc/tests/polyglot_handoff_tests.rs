/// Polyglot handoff integration tests.
///
/// Each test simulates a cross-language handoff at the A2A protocol layer.
/// The "sending" agent is represented by `A2aClient` + `perform_handoff`
/// and the "receiving" agent is a live `AgentCard::serve` HTTP server.
///
/// Language labels in test names document the intended binding pairing;
/// the protocol layer is identical regardless of which SDK wraps it.
use ancora_grpc::{
    agent_card::AgentCard,
    handoff::{perform_handoff, HandoffRequest},
    identity::AgentIdentity,
    task::TaskStatus,
};
use tokio::sync::oneshot;

async fn spawn_agent(card: AgentCard) -> (std::net::SocketAddr, oneshot::Sender<()>) {
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let bound = listener.local_addr().unwrap();
    drop(listener);

    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(async move { card.serve(bound, rx).await.unwrap() });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    (bound, tx)
}

#[tokio::test]
async fn python_agent_hands_off_to_a_go_agent() {
    let go_card = AgentCard::new(
        "go-agent",
        "A Go-based Ancora agent",
        "grpc://127.0.0.1:50051",
    );
    let id = AgentIdentity::generate();
    let signed_go_card = id.attach_to(go_card);
    let (bound, tx) = spawn_agent(signed_go_card).await;

    let req = HandoffRequest {
        agent_url: format!("http://127.0.0.1:{}", bound.port()),
        task_id: "py-to-go-001".into(),
        input: "Summarise the quarterly report.".into(),
        require_signed_identity: true,
    };

    let result = perform_handoff(req).await.expect("handoff should succeed");

    assert_eq!(result.remote_card.name, "go-agent");
    assert!(result.remote_card.identity_key.is_some());
    assert_eq!(result.task.id, "py-to-go-001");
    assert_eq!(result.task.status, TaskStatus::Queued);
    assert_eq!(
        result.task.input.as_deref(),
        Some("Summarise the quarterly report.")
    );

    let _ = tx.send(());
}

#[tokio::test]
async fn ts_agent_hands_off_to_a_dotnet_agent() {
    let dotnet_card = AgentCard::new(
        "dotnet-agent",
        "A .NET-based Ancora agent",
        "grpc://127.0.0.1:50052",
    );
    let id = AgentIdentity::generate();
    let signed_dotnet_card = id.attach_to(dotnet_card);
    let (bound, tx) = spawn_agent(signed_dotnet_card).await;

    let req = HandoffRequest {
        agent_url: format!("http://127.0.0.1:{}", bound.port()),
        task_id: "ts-to-dotnet-001".into(),
        input: "Translate this text to French.".into(),
        require_signed_identity: true,
    };

    let result = perform_handoff(req).await.expect("handoff should succeed");

    assert_eq!(result.remote_card.name, "dotnet-agent");
    assert!(result.remote_card.signature.is_some());
    assert_eq!(result.task.id, "ts-to-dotnet-001");

    let _ = tx.send(());
}

#[tokio::test]
async fn handoff_without_identity_requirement_accepts_unsigned_card() {
    let card = AgentCard::new("unsigned-agent", "No signature", "grpc://localhost:50051");
    let (bound, tx) = spawn_agent(card).await;

    let req = HandoffRequest {
        agent_url: format!("http://127.0.0.1:{}", bound.port()),
        task_id: "open-handoff-001".into(),
        input: "process this".into(),
        require_signed_identity: false,
    };

    let result = perform_handoff(req)
        .await
        .expect("unsigned handoff should succeed");
    assert_eq!(result.remote_card.name, "unsigned-agent");
    assert!(result.remote_card.identity_key.is_none());

    let _ = tx.send(());
}

#[tokio::test]
async fn handoff_with_identity_requirement_rejects_unsigned_card() {
    let card = AgentCard::new(
        "unsigned-go-agent",
        "No signature",
        "grpc://localhost:50051",
    );
    let (bound, tx) = spawn_agent(card).await;

    let req = HandoffRequest {
        agent_url: format!("http://127.0.0.1:{}", bound.port()),
        task_id: "fail-001".into(),
        input: "should not reach here".into(),
        require_signed_identity: true,
    };

    let err = perform_handoff(req).await.unwrap_err();
    assert!(
        err.contains("identity verification failed"),
        "expected identity error, got: {}",
        err
    );

    let _ = tx.send(());
}
