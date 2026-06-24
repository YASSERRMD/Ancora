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
