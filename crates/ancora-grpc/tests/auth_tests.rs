use ancora_grpc::auth::AuthInterceptor;
use ancora_grpc::proto::run_service_server::RunServiceServer;
use ancora_grpc::proto::StartRunRequest;
use ancora_grpc::service::RunServiceImpl;

use tonic::transport::Server;
use tonic::Request;

async fn bind_authed_server(token: &str) -> u16 {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let svc = RunServiceServer::with_interceptor(RunServiceImpl::new(), AuthInterceptor::new(token));
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(format!("127.0.0.1:{port}").parse().unwrap())
            .await
            .unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    port
}

#[tokio::test]
async fn authenticated_request_succeeds() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_authed_server("mytoken").await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let mut req = Request::new(StartRunRequest { agent_spec: b"{}".to_vec() });
    req.metadata_mut().insert("authorization", "Bearer mytoken".parse().unwrap());
    let resp = client.start_run(req).await;
    assert!(resp.is_ok(), "authenticated request should succeed");
}

#[tokio::test]
async fn unauthenticated_request_is_rejected() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_authed_server("mytoken").await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let resp = client
        .start_run(Request::new(StartRunRequest { agent_spec: b"{}".to_vec() }))
        .await;
    let err = resp.unwrap_err();
    assert_eq!(err.code(), tonic::Code::Unauthenticated);
}

#[tokio::test]
async fn wrong_token_request_is_rejected() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_authed_server("mytoken").await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let mut req = Request::new(StartRunRequest { agent_spec: b"{}".to_vec() });
    req.metadata_mut().insert("authorization", "Bearer wrongtoken".parse().unwrap());
    let err = client.start_run(req).await.unwrap_err();
    assert_eq!(err.code(), tonic::Code::Unauthenticated);
}

#[tokio::test]
async fn authenticated_start_run_returns_valid_run_id() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_authed_server("tok").await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let mut req = Request::new(StartRunRequest { agent_spec: b"{}".to_vec() });
    req.metadata_mut().insert("authorization", "Bearer tok".parse().unwrap());
    let id = client.start_run(req).await.unwrap().into_inner().run_id;
    assert!(!id.is_empty());
}
