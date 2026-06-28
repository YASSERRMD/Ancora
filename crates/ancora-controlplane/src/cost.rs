use crate::model::CostSummary;
use crate::store::ControlPlaneStore;

pub struct CostApi<'a> {
    store: &'a mut ControlPlaneStore,
    auth: &'a crate::auth::TokenAuth,
}

impl<'a> CostApi<'a> {
    pub fn new(store: &'a mut ControlPlaneStore, auth: &'a crate::auth::TokenAuth) -> Self {
        CostApi { store, auth }
    }

    pub fn record(
        &mut self,
        token: Option<&str>,
        run_id: &str,
        tokens: u64,
        usd_micro: u64,
    ) -> Result<(), crate::auth::AuthError> {
        self.auth.verify(token)?;
        self.store.record_cost(run_id, tokens, usd_micro);
        Ok(())
    }

    pub fn per_run(
        &self,
        token: Option<&str>,
        run_id: &str,
    ) -> Result<Option<CostSummary>, crate::auth::AuthError> {
        self.auth.verify(token)?;
        Ok(self.store.cost_per_run(run_id))
    }

    pub fn aggregate(
        &self,
        token: Option<&str>,
        tenant_id: &str,
    ) -> Result<CostSummary, crate::auth::AuthError> {
        self.auth.verify(token)?;
        Ok(self.store.cost_aggregate(tenant_id))
    }
}
