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
