use std::collections::HashMap;
use crate::transfer::{TransferRequest, TransferStatus};

pub struct AirGapStats {
    pub total_transfers: usize,
    pub pending: usize,
    pub approved: usize,
    pub rejected: usize,
    pub completed: usize,
    pub by_media: HashMap<String, usize>,
}

impl AirGapStats {
    pub fn for_tenant(transfers: &[&TransferRequest], tenant_id: &str) -> Self {
        let tenant_transfers: Vec<&&TransferRequest> = transfers
            .iter()
            .filter(|r| r.tenant_id == tenant_id)
            .collect();

        let total_transfers = tenant_transfers.len();
        let pending = tenant_transfers.iter().filter(|r| r.status == TransferStatus::Pending).count();
        let approved = tenant_transfers.iter().filter(|r| r.status == TransferStatus::Approved).count();
        let rejected = tenant_transfers.iter().filter(|r| r.status == TransferStatus::Rejected).count();
        let completed = tenant_transfers.iter().filter(|r| r.status == TransferStatus::Completed).count();

        let mut by_media = HashMap::new();
        for r in &tenant_transfers {
            *by_media.entry(format!("{}", r.media)).or_insert(0) += 1;
        }

        Self { total_transfers, pending, approved, rejected, completed, by_media }
    }

    pub fn rejection_rate(&self) -> f64 {
        if self.total_transfers == 0 { return 0.0; }
        self.rejected as f64 / self.total_transfers as f64
    }
}
