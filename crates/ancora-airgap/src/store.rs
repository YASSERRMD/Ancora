use crate::transfer::{TransferRequest, TransferStatus};
use std::collections::HashMap;

pub struct TransferStore {
    transfers: HashMap<String, TransferRequest>,
}

impl Default for TransferStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferStore {
    pub fn new() -> Self {
        Self {
            transfers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, req: TransferRequest) {
        self.transfers.insert(req.id.clone(), req);
    }

    pub fn get(&self, id: &str) -> Option<&TransferRequest> {
        self.transfers.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut TransferRequest> {
        self.transfers.get_mut(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<TransferRequest> {
        self.transfers.remove(id)
    }

    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a TransferRequest> {
        self.transfers
            .values()
            .filter(|r| r.tenant_id == tenant_id)
            .collect()
    }

    pub fn pending(&self) -> Vec<&TransferRequest> {
        self.transfers.values().filter(|r| r.is_pending()).collect()
    }

    pub fn by_status<'a>(&'a self, status: &TransferStatus) -> Vec<&'a TransferRequest> {
        self.transfers
            .values()
            .filter(|r| &r.status == status)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.transfers.len()
    }
}
