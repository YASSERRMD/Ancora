use crate::auth::{AuthError, TokenAuth};
use crate::model::{ResumeDecision, Run, RunPriority, RunState};
use crate::pagination::{Page, PageCursor};
use crate::store::{ControlPlaneStore, StoreError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunsApiError {
    #[error("auth: {0}")]
    Auth(#[from] AuthError),
    #[error("store: {0}")]
    Store(#[from] StoreError),
}

pub struct RunsApi<'a> {
    store: &'a mut ControlPlaneStore,
    auth: &'a TokenAuth,
}

impl<'a> RunsApi<'a> {
    pub fn new(store: &'a mut ControlPlaneStore, auth: &'a TokenAuth) -> Self {
        RunsApi { store, auth }
    }

    pub fn create(
        &mut self,
        token: Option<&str>,
        tenant_id: &str,
        priority: RunPriority,
    ) -> Result<Run, RunsApiError> {
        self.auth.verify(token)?;
        Ok(self.store.create_run(tenant_id, priority))
    }

    pub fn get(&self, token: Option<&str>, id: &str) -> Result<Run, RunsApiError> {
        self.auth.verify(token)?;
        self.store
            .get_run(id)
            .cloned()
            .ok_or_else(|| RunsApiError::Store(StoreError::RunNotFound(id.to_string())))
    }

    pub fn list(
        &self,
        token: Option<&str>,
        tenant_id: Option<&str>,
        state: Option<&RunState>,
        cursor: Option<&PageCursor>,
        limit: usize,
    ) -> Result<Page<Run>, RunsApiError> {
        self.auth.verify(token)?;
        Ok(self.store.list_runs(tenant_id, state, cursor, limit))
    }

    pub fn cancel(&mut self, token: Option<&str>, id: &str) -> Result<(), RunsApiError> {
        self.auth.verify(token)?;
        self.store.cancel_run(id)?;
        Ok(())
    }

    pub fn pause(&mut self, token: Option<&str>, id: &str) -> Result<(), RunsApiError> {
        self.auth.verify(token)?;
        self.store.pause_run(id)?;
        Ok(())
    }

    pub fn resume(
        &mut self,
        token: Option<&str>,
        id: &str,
        decision: ResumeDecision,
    ) -> Result<(), RunsApiError> {
        self.auth.verify(token)?;
        self.store.resume_run(id, decision)?;
        Ok(())
    }

    pub fn journal_tail(
        &self,
        token: Option<&str>,
        run_id: &str,
        from_seq: u64,
    ) -> Result<Vec<crate::model::JournalEntry>, RunsApiError> {
        self.auth.verify(token)?;
        Ok(self.store.tail_journal(run_id, from_seq))
    }

    pub fn cost_per_run(
        &self,
        token: Option<&str>,
        run_id: &str,
    ) -> Result<Option<crate::model::CostSummary>, RunsApiError> {
        self.auth.verify(token)?;
        Ok(self.store.cost_per_run(run_id))
    }

    pub fn cost_aggregate(
        &self,
        token: Option<&str>,
        tenant_id: &str,
    ) -> Result<crate::model::CostSummary, RunsApiError> {
        self.auth.verify(token)?;
        Ok(self.store.cost_aggregate(tenant_id))
    }
}
