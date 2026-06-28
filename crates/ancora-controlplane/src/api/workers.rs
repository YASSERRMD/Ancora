use crate::auth::TokenAuth;
use crate::model::{Run, Worker};
use crate::store::{ControlPlaneStore, StoreError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkersApiError {
    #[error("auth: {0}")]
    Auth(#[from] crate::auth::AuthError),
    #[error("store: {0}")]
    Store(#[from] StoreError),
}

pub struct WorkersApi<'a> {
    store: &'a mut ControlPlaneStore,
    auth: &'a TokenAuth,
}

impl<'a> WorkersApi<'a> {
    pub fn new(store: &'a mut ControlPlaneStore, auth: &'a TokenAuth) -> Self {
        WorkersApi { store, auth }
    }

    pub fn register(&mut self, token: Option<&str>, concurrency: usize) -> Result<Worker, WorkersApiError> {
        self.auth.verify(token)?;
        Ok(self.store.register_worker(concurrency))
    }

    pub fn heartbeat(&mut self, token: Option<&str>, worker_id: &str) -> Result<(), WorkersApiError> {
        self.auth.verify(token)?;
        self.store.heartbeat_worker(worker_id)?;
        Ok(())
    }

    pub fn claim_run(&mut self, token: Option<&str>, worker_id: &str) -> Result<Option<Run>, WorkersApiError> {
        self.auth.verify(token)?;
        Ok(self.store.claim_run(worker_id)?)
    }

    pub fn release(
        &mut self,
        token: Option<&str>,
        worker_id: &str,
        run_id: &str,
        success: bool,
    ) -> Result<(), WorkersApiError> {
        self.auth.verify(token)?;
        self.store.release_lease(worker_id, run_id, success);
        Ok(())
    }

    pub fn expire_leases(&mut self, token: Option<&str>) -> Result<(), WorkersApiError> {
        self.auth.verify(token)?;
        self.store.expire_leases();
        Ok(())
    }
}
