use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};

pub struct IndicatorBuilder {
    id: String,
    tenant_id: String,
    kind: IndicatorKind,
    value: String,
    threat_level: ThreatLevel,
    source: String,
    tick: u64,
    expiry: Option<u64>,
    tags: Vec<String>,
}

impl IndicatorBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        kind: IndicatorKind,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            kind,
            value: value.into(),
            threat_level: ThreatLevel::Medium,
            source: "unknown".into(),
            tick: 0,
            expiry: None,
            tags: Vec::new(),
        }
    }

    pub fn threat_level(mut self, l: ThreatLevel) -> Self {
        self.threat_level = l;
        self
    }
    pub fn source(mut self, s: impl Into<String>) -> Self {
        self.source = s.into();
        self
    }
    pub fn tick(mut self, t: u64) -> Self {
        self.tick = t;
        self
    }
    pub fn expires_at(mut self, t: u64) -> Self {
        self.expiry = Some(t);
        self
    }
    pub fn tag(mut self, t: impl Into<String>) -> Self {
        self.tags.push(t.into());
        self
    }

    pub fn build(self) -> Indicator {
        let mut ind = Indicator::new(
            self.id,
            self.tenant_id,
            self.kind,
            self.value,
            self.threat_level,
            self.source,
            self.tick,
        );
        if let Some(e) = self.expiry {
            ind = ind.with_expiry(e);
        }
        for tag in self.tags {
            ind = ind.with_tag(tag);
        }
        ind
    }
}
