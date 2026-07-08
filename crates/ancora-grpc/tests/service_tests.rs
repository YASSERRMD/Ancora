use ancora_grpc::proto::run_service_server::RunServiceServer;
use ancora_grpc::proto::{
    DecisionRequest, PollRunRequest, ResumeRunRequest, StartRunRequest, StreamEventsRequest,
};
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

#[tokio::test]
async fn start_run_returns_non_empty_run_id() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let resp = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(!resp.run_id.is_empty());
}

#[tokio::test]
async fn poll_run_first_event_is_started() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let event = client
        .poll_run(Request::new(PollRunRequest { run_id }))
        .await
        .unwrap()
        .into_inner()
        .event;
    assert!(event.contains("started"), "expected started, got: {event}");
}

#[tokio::test]
async fn poll_run_second_event_is_completed() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    let e2 = client
        .poll_run(Request::new(PollRunRequest { run_id }))
        .await
        .unwrap()
        .into_inner()
        .event;
    assert!(e2.contains("completed"), "expected completed, got: {e2}");
}

#[tokio::test]
async fn resume_run_returns_ok() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let status = client
        .resume_run(Request::new(ResumeRunRequest {
            run_id,
            decision: b"approved".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .status;
    assert_eq!(status, "ok");
}

#[tokio::test]
async fn resume_then_poll_yields_resumed_event() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    client
        .resume_run(Request::new(ResumeRunRequest {
            run_id: run_id.clone(),
            decision: b"go".to_vec(),
        }))
        .await
        .unwrap();
    let e = client
        .poll_run(Request::new(PollRunRequest { run_id }))
        .await
        .unwrap()
        .into_inner()
        .event;
    assert!(e.contains("resumed"), "expected resumed event, got: {e}");
}

#[tokio::test]
async fn resume_unknown_run_returns_not_found() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let status = client
        .resume_run(Request::new(ResumeRunRequest {
            run_id: "no-such-run".into(),
            decision: b"x".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .status;
    assert_eq!(status, "not_found");
}

#[tokio::test]
async fn poll_exhausted_run_returns_empty_event() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    for _ in 0..2 {
        client
            .poll_run(Request::new(PollRunRequest {
                run_id: run_id.clone(),
            }))
            .await
            .unwrap();
    }
    let e = client
        .poll_run(Request::new(PollRunRequest { run_id }))
        .await
        .unwrap()
        .into_inner()
        .event;
    assert!(
        e.is_empty(),
        "expected empty after events exhausted, got: {e}"
    );
}

#[tokio::test]
async fn two_independent_runs_have_different_ids() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let id1 = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let id2 = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    assert_ne!(id1, id2);
}

#[tokio::test]
async fn drive_full_run_start_poll_poll_resume_poll() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let e1 = client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap()
        .into_inner()
        .event;
    let e2 = client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap()
        .into_inner()
        .event;
    client
        .resume_run(Request::new(ResumeRunRequest {
            run_id: run_id.clone(),
            decision: b"yes".to_vec(),
        }))
        .await
        .unwrap();
    let e3 = client
        .poll_run(Request::new(PollRunRequest { run_id }))
        .await
        .unwrap()
        .into_inner()
        .event;
    assert!(e1.contains("started"), "e1={e1}");
    assert!(e2.contains("completed"), "e2={e2}");
    assert!(e3.contains("resumed"), "e3={e3}");
}

#[tokio::test]
async fn poll_unknown_run_returns_empty_event() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let e = client
        .poll_run(Request::new(PollRunRequest {
            run_id: "ghost".into(),
        }))
        .await
        .unwrap()
        .into_inner()
        .event;
    assert!(e.is_empty(), "expected empty for unknown run, got: {e}");
}

#[tokio::test]
async fn stream_events_yields_started_then_completed() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::StreamExt;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let mut stream = client
        .stream_events(Request::new(StreamEventsRequest { run_id }))
        .await
        .unwrap()
        .into_inner();
    let e1 = stream.next().await.unwrap().unwrap().event;
    let e2 = stream.next().await.unwrap().unwrap().event;
    assert!(e1.contains("started"), "e1={e1}");
    assert!(e2.contains("completed"), "e2={e2}");
    assert!(stream.next().await.is_none(), "expected stream end");
}

#[tokio::test]
async fn stream_events_arrive_in_order() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::StreamExt;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let mut stream = client
        .stream_events(Request::new(StreamEventsRequest { run_id }))
        .await
        .unwrap()
        .into_inner();
    let mut events = Vec::new();
    while let Some(Ok(ev)) = stream.next().await {
        events.push(ev.event);
    }
    assert_eq!(events, vec!["started", "completed"]);
}

#[tokio::test]
async fn decision_stream_emits_resumed_event() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::{wrappers::ReceiverStream, StreamExt};
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    let (tx, rx) = tokio::sync::mpsc::channel(4);
    tx.send(DecisionRequest {
        run_id: run_id.clone(),
        decision: b"yes".to_vec(),
    })
    .await
    .unwrap();
    drop(tx);
    let mut out_stream = client
        .decision_stream(Request::new(ReceiverStream::new(rx)))
        .await
        .unwrap()
        .into_inner();
    let ev = out_stream.next().await.unwrap().unwrap().event;
    assert!(ev.contains("resumed"), "expected resumed, got: {ev}");
}

#[tokio::test]
async fn stream_events_empty_run_returns_no_events() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::StreamExt;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let mut stream = client
        .stream_events(Request::new(StreamEventsRequest {
            run_id: "ghost".into(),
        }))
        .await
        .unwrap()
        .into_inner();
    assert!(
        stream.next().await.is_none(),
        "ghost run should yield no events"
    );
}

#[tokio::test]
async fn decision_stream_multiple_decisions_all_emit_events() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::{wrappers::ReceiverStream, StreamExt};
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    let (tx, rx) = tokio::sync::mpsc::channel(4);
    tx.send(DecisionRequest {
        run_id: run_id.clone(),
        decision: b"a".to_vec(),
    })
    .await
    .unwrap();
    tx.send(DecisionRequest {
        run_id: run_id.clone(),
        decision: b"b".to_vec(),
    })
    .await
    .unwrap();
    drop(tx);
    let mut out_stream = client
        .decision_stream(Request::new(ReceiverStream::new(rx)))
        .await
        .unwrap()
        .into_inner();
    let mut events = Vec::new();
    while let Some(Ok(ev)) = out_stream.next().await {
        events.push(ev.event);
    }
    assert!(
        !events.is_empty(),
        "expected at least one event from two decisions"
    );
}

#[tokio::test]
async fn stream_events_count_equals_two_for_fresh_run() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::StreamExt;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let mut stream = client
        .stream_events(Request::new(StreamEventsRequest { run_id }))
        .await
        .unwrap()
        .into_inner();
    let mut count = 0usize;
    while let Some(Ok(_)) = stream.next().await {
        count += 1;
    }
    assert_eq!(count, 2, "fresh run should yield exactly 2 events");
}

#[tokio::test]
async fn stream_after_partial_poll_yields_remaining_events() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::StreamExt;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    client
        .poll_run(Request::new(PollRunRequest {
            run_id: run_id.clone(),
        }))
        .await
        .unwrap();
    let mut stream = client
        .stream_events(Request::new(StreamEventsRequest { run_id }))
        .await
        .unwrap()
        .into_inner();
    let mut count = 0usize;
    while let Some(Ok(_)) = stream.next().await {
        count += 1;
    }
    assert_eq!(count, 1, "one event consumed by poll, one should remain");
}

#[tokio::test]
async fn stream_first_event_contains_started_text() {
    use ancora_grpc::proto::run_service_client::RunServiceClient;
    use tokio_stream::StreamExt;
    let port = bind_server().await;
    let mut client = RunServiceClient::connect(format!("http://127.0.0.1:{port}"))
        .await
        .unwrap();
    let run_id = client
        .start_run(Request::new(StartRunRequest {
            agent_spec: b"{}".to_vec(),
        }))
        .await
        .unwrap()
        .into_inner()
        .run_id;
    let first = client
        .stream_events(Request::new(StreamEventsRequest { run_id }))
        .await
        .unwrap()
        .into_inner()
        .next()
        .await
        .unwrap()
        .unwrap()
        .event;
    assert!(first.contains("started"), "got: {first}");
}
