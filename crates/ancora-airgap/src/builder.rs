use crate::boundary::{AirGapZone, ZoneClassification};
use crate::media::MediaType;
use crate::transfer::{TransferDirection, TransferRequest};

pub struct TransferBuilder {
    id: String,
    tenant_id: String,
    requestor: String,
    media: MediaType,
    direction: TransferDirection,
    description: String,
    tick: u64,
    checksum: Option<String>,
}

impl TransferBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        requestor: impl Into<String>,
        media: MediaType,
        direction: TransferDirection,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            requestor: requestor.into(),
            media,
            direction,
            description: String::new(),
            tick: 0,
            checksum: None,
        }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = d.into();
        self
    }

    pub fn tick(mut self, t: u64) -> Self { self.tick = t; self }

    pub fn checksum(mut self, cs: impl Into<String>) -> Self {
        self.checksum = Some(cs.into());
        self
    }

    pub fn build(self) -> TransferRequest {
        let mut req = TransferRequest::new(
            self.id, self.tenant_id, self.requestor, self.media, self.direction,
            self.description, self.tick,
        );
        if let Some(cs) = self.checksum { req.checksum = Some(cs); }
        req
    }
}

pub struct ZoneBuilder {
    id: String,
    name: String,
    classification: ZoneClassification,
    tenant_id: String,
}

impl ZoneBuilder {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        classification: ZoneClassification,
        tenant_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            classification,
            tenant_id: tenant_id.into(),
        }
    }

    pub fn build(self) -> AirGapZone {
        AirGapZone::new(self.id, self.name, self.classification, self.tenant_id)
    }
}
