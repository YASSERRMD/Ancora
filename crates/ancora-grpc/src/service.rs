use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::proto::{
    run_service_server::RunService, PollRunRequest, PollRunResponse, ResumeRunRequest,
    ResumeRunResponse, StartRunRequest, StartRunResponse,
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
}
