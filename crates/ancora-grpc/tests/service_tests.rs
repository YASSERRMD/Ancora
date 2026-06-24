use ancora_grpc::proto::run_service_server::RunServiceServer;
use ancora_grpc::proto::{PollRunRequest, ResumeRunRequest, StartRunRequest};
use ancora_grpc::service::RunServiceImpl;

use tonic::transport::Server;
use tonic::Request;

async fn bind_server() -> u16 {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let svc = RunServiceImpl::new();
    tokio::spawn(async move {
        Server::builder()
            .add_service(RunServiceServer::new(svc))
            .serve(format!("127.0.0.1:{port}").parse().unwrap())
            .await
            .unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    port
}
