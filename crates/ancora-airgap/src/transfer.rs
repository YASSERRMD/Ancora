use crate::media::MediaType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferDirection {
    Inbound,
    Outbound,
}

impl fmt::Display for TransferDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TransferDirection::Inbound => "INBOUND",
            TransferDirection::Outbound => "OUTBOUND",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
    Cancelled,
}

impl fmt::Display for TransferStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TransferStatus::Pending => "PENDING",
            TransferStatus::Approved => "APPROVED",
            TransferStatus::Rejected => "REJECTED",
            TransferStatus::Completed => "COMPLETED",
            TransferStatus::Cancelled => "CANCELLED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct TransferRequest {
    pub id: String,
    pub tenant_id: String,
    pub requestor: String,
    pub media: MediaType,
    pub direction: TransferDirection,
    pub description: String,
    pub status: TransferStatus,
    pub created_tick: u64,
    pub resolved_tick: Option<u64>,
    pub checksum: Option<String>,
}

impl TransferRequest {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        requestor: impl Into<String>,
        media: MediaType,
        direction: TransferDirection,
        description: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            requestor: requestor.into(),
            media,
            direction,
            description: description.into(),
            status: TransferStatus::Pending,
            created_tick: tick,
            resolved_tick: None,
            checksum: None,
        }
    }

    pub fn approve(&mut self, tick: u64) {
        self.status = TransferStatus::Approved;
        self.resolved_tick = Some(tick);
    }

    pub fn reject(&mut self, tick: u64) {
        self.status = TransferStatus::Rejected;
        self.resolved_tick = Some(tick);
    }

    pub fn complete(&mut self, tick: u64) {
        self.status = TransferStatus::Completed;
        self.resolved_tick = Some(tick);
    }

    pub fn cancel(&mut self) {
        self.status = TransferStatus::Cancelled;
    }

    pub fn with_checksum(mut self, cs: impl Into<String>) -> Self {
        self.checksum = Some(cs.into());
        self
    }

    pub fn is_pending(&self) -> bool {
        self.status == TransferStatus::Pending
    }
    pub fn is_approved(&self) -> bool {
        self.status == TransferStatus::Approved
    }
}
