use ancora_controlplane::model::{Run, RunId};
use ancora_controlplane::store::ControlPlaneStore;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("no run to claim")]
    NoClaim,
    #[error("store error: {0}")]
    Store(#[from] ancora_controlplane::store::StoreError),
    #[error("step failed: {0}")]
    StepFailed(String),
}

pub type StepFn = Box<dyn Fn(&Run) -> Result<String, String> + Send + Sync>;

pub struct WorkerExecutor {
    worker_id: String,
    store: Arc<Mutex<ControlPlaneStore>>,
    step_fn: Arc<StepFn>,
}

impl WorkerExecutor {
    pub fn new(worker_id: String, store: Arc<Mutex<ControlPlaneStore>>, step_fn: StepFn) -> Self {
        WorkerExecutor {
            worker_id,
            store,
            step_fn: Arc::new(step_fn),
        }
    }

    pub fn claim_and_execute(&self) -> Result<Option<RunId>, ExecutorError> {
        let claimed = {
            let mut s = self.store.lock().unwrap();
            s.claim_run(&self.worker_id)?
        };

        let run = match claimed {
            Some(r) => r,
            None => return Ok(None),
        };

        info!(run_id = %run.id, worker_id = %self.worker_id, "claimed run");
        let run_id = run.id.clone();

        let result = (self.step_fn)(&run);

        let success = result.is_ok();
        let payload = match result {
            Ok(msg) => msg,
            Err(e) => {
                warn!(run_id = %run_id, error = %e, "step failed");
                e
            }
        };

        {
            let mut s = self.store.lock().unwrap();
            s.append_journal(&run_id, payload);
            s.release_lease(&self.worker_id, &run_id, success);
        }

        if success {
            Ok(Some(run_id))
        } else {
            Err(ExecutorError::StepFailed(run_id))
        }
    }

    pub fn renew_lease(&self) {
        let mut s = self.store.lock().unwrap();
        let _ = s.heartbeat_worker(&self.worker_id);
    }

    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }
}
