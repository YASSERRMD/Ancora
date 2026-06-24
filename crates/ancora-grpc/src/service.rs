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
