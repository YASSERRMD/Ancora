use crate::media::MediaType;
use crate::transfer::{TransferDirection, TransferRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyVerdict {
    Allow,
    Deny(String),
    RequireApproval,
}

pub struct AirGapPolicy {
    pub tenant_id: String,
    blocked_media: Vec<MediaType>,
    require_approval_media: Vec<MediaType>,
    block_all_outbound: bool,
    require_checksum: bool,
}

impl AirGapPolicy {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            blocked_media: Vec::new(),
            require_approval_media: Vec::new(),
            block_all_outbound: false,
            require_checksum: false,
        }
    }

    pub fn block_media(mut self, media: MediaType) -> Self {
        self.blocked_media.push(media);
        self
    }

    pub fn require_approval_for(mut self, media: MediaType) -> Self {
        self.require_approval_media.push(media);
        self
    }

    pub fn block_all_outbound(mut self) -> Self {
        self.block_all_outbound = true;
        self
    }

    pub fn require_checksum(mut self) -> Self {
        self.require_checksum = true;
        self
    }

    pub fn evaluate(&self, request: &TransferRequest) -> PolicyVerdict {
        if self.blocked_media.contains(&request.media) {
            return PolicyVerdict::Deny(format!("{} is blocked", request.media));
        }
        if self.block_all_outbound && request.direction == TransferDirection::Outbound {
            return PolicyVerdict::Deny("all outbound transfers are blocked".into());
        }
        if self.require_checksum && request.checksum.is_none() {
            return PolicyVerdict::Deny("checksum required".into());
        }
        if self.require_approval_media.contains(&request.media) {
            return PolicyVerdict::RequireApproval;
        }
        PolicyVerdict::Allow
    }

    pub fn media_blocked(&self, media: &MediaType) -> bool {
        self.blocked_media.contains(media)
    }
}
