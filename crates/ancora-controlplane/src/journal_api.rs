use crate::auth::{AuthError, TokenAuth};
use crate::model::JournalEntry;
use crate::store::ControlPlaneStore;

pub struct JournalApi<'a> {
    store: &'a mut ControlPlaneStore,
    auth: &'a TokenAuth,
}

impl<'a> JournalApi<'a> {
    pub fn new(store: &'a mut ControlPlaneStore, auth: &'a TokenAuth) -> Self {
        JournalApi { store, auth }
    }

    pub fn append(
        &mut self,
        token: Option<&str>,
        run_id: &str,
        payload: String,
    ) -> Result<(), AuthError> {
        self.auth.verify(token)?;
        self.store.append_journal(run_id, payload);
        Ok(())
    }

    pub fn tail(
        &self,
        token: Option<&str>,
        run_id: &str,
        from_seq: u64,
    ) -> Result<Vec<JournalEntry>, AuthError> {
        self.auth.verify(token)?;
        Ok(self.store.tail_journal(run_id, from_seq))
    }
}
