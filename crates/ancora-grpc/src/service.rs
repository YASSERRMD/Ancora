use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use crate::proto::{
    run_service_server::RunService, DecisionRequest, EventResponse, PollRunRequest, PollRunResponse,
    ResumeRunRequest, ResumeRunResponse, StartRunRequest, StartRunResponse, StreamEventsRequest,
};
use crate::store::RunStore;

pub struct RunServiceImpl {
    store: Arc<RunStore>,
}

impl RunServiceImpl {
    pub fn new() -> Self {
        Self { store: Arc::new(RunStore::new()) }
    }

    pub fn with_store(store: Arc<RunStore>) -> Self {
        Self { store }
    }
}

#[tonic::async_trait]
impl RunService for RunServiceImpl {
    type StreamEventsStream = ReceiverStream<Result<EventResponse, Status>>;
    type DecisionStreamStream = ReceiverStream<Result<EventResponse, Status>>;

    async fn start_run(
        &self,
        _request: Request<StartRunRequest>,
    ) -> Result<Response<StartRunResponse>, Status> {
        let run_id = uuid::Uuid::new_v4().to_string();
        self.store.insert(run_id.clone());
        Ok(Response::new(StartRunResponse { run_id }))
    }

    async fn poll_run(
        &self,
        request: Request<PollRunRequest>,
    ) -> Result<Response<PollRunResponse>, Status> {
        let run_id = request.into_inner().run_id;
        let event = self.store.poll(&run_id).unwrap_or_default();
        Ok(Response::new(PollRunResponse { event }))
    }

    async fn resume_run(
        &self,
        request: Request<ResumeRunRequest>,
    ) -> Result<Response<ResumeRunResponse>, Status> {
        let req = request.into_inner();
        let decision = String::from_utf8_lossy(&req.decision).into_owned();
        let found = self.store.resume(&req.run_id, &decision);
        let status = if found { "ok".into() } else { "not_found".into() };
        Ok(Response::new(ResumeRunResponse { status }))
    }

    async fn stream_events(
        &self,
        request: Request<StreamEventsRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let run_id = request.into_inner().run_id;
        let (tx, rx) = mpsc::channel(16);
        let store = Arc::clone(&self.store);
        tokio::spawn(async move {
            loop {
                match store.poll(&run_id) {
                    Some(ev) => {
                        if tx.send(Ok(EventResponse { event: ev })).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn decision_stream(
        &self,
        request: Request<Streaming<DecisionRequest>>,
    ) -> Result<Response<Self::DecisionStreamStream>, Status> {
        let (tx, rx) = mpsc::channel(16);
        let store = Arc::clone(&self.store);
        let mut stream = request.into_inner();
        tokio::spawn(async move {
            while let Ok(Some(req)) = stream.message().await {
                let decision = String::from_utf8_lossy(&req.decision).into_owned();
                store.resume(&req.run_id, &decision);
                loop {
                    match store.poll(&req.run_id) {
                        Some(ev) => {
                            if tx.send(Ok(EventResponse { event: ev })).await.is_err() {
                                return;
                            }
                        }
                        None => break,
                    }
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
