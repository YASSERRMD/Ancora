use ancora_grpc::{
    agent_card::AgentCard,
    client::A2aClient,
    identity::AgentIdentity,
};
use tokio::sync::oneshot;

async fn spawn_card_server(card: AgentCard) -> (std::net::SocketAddr, oneshot::Sender<()>) {
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = std::net::TcpListener::bind(addr).unwrap();
    let bound = listener.local_addr().unwrap();
    drop(listener);

    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        card.serve(bound, rx).await.unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    (bound, tx)
}

#[tokio::test]
async fn ancora_fetches_remote_agent_card() {
    let card = AgentCard::new("remote-agent", "A remote agent", "grpc://127.0.0.1:50051");
    let (bound, tx) = spawn_card_server(card).await;

    let client = A2aClient::new("127.0.0.1", bound.port());
    let fetched = client.fetch_card().await.expect("should fetch card");

    assert_eq!(fetched.name, "remote-agent");
    assert_eq!(fetched.description, "A remote agent");

    let _ = tx.send(());
}

#[tokio::test]
async fn ancora_verifies_signed_remote_agent() {
    let card = AgentCard::new("signed-remote", "A signed remote agent", "grpc://127.0.0.1:50051");
    let id = AgentIdentity::generate();
    let signed = id.attach_to(card);
    let (bound, tx) = spawn_card_server(signed).await;

    let client = A2aClient::new("127.0.0.1", bound.port());
    let verified = client
        .fetch_and_verify_card()
        .await
        .expect("signed card should verify");

    assert_eq!(verified.name, "signed-remote");
    assert!(verified.identity_key.is_some());

    let _ = tx.send(());
}

#[tokio::test]
async fn ancora_rejects_unsigned_remote_agent() {
    let card = AgentCard::new("unsigned-remote", "No signature", "grpc://127.0.0.1:50051");
    let (bound, tx) = spawn_card_server(card).await;

    let client = A2aClient::new("127.0.0.1", bound.port());
    let result = client.fetch_and_verify_card().await;

    assert!(result.is_err(), "unsigned card should be rejected");
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("signature is missing or invalid"),
        "unexpected error: {}",
        err
    );

    let _ = tx.send(());
}

#[tokio::test]
async fn from_url_client_fetches_card() {
    let card = AgentCard::new("url-agent", "URL-parsed client", "grpc://127.0.0.1:50051");
    let (bound, tx) = spawn_card_server(card).await;

    let url = format!("http://127.0.0.1:{}", bound.port());
    let client = A2aClient::from_url(&url).expect("valid URL");
    let fetched = client.fetch_card().await.expect("should fetch card");
    assert_eq!(fetched.name, "url-agent");

    let _ = tx.send(());
}

#[tokio::test]
async fn ancora_submits_task_to_remote_agent() {
    let client = A2aClient::new("127.0.0.1", 59999);
    let task = client.submit_task("task-001", "Summarise this document.").await;

    assert_eq!(task.id, "task-001");
    assert_eq!(
        task.input.as_deref(),
        Some("Summarise this document.")
    );
}
