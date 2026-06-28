use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndicatorKind {
    IpAddress,
    Domain,
    Url,
    FileHash,
    Email,
    CertificateHash,
}

impl fmt::Display for IndicatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IndicatorKind::IpAddress => "IP_ADDRESS",
            IndicatorKind::Domain => "DOMAIN",
            IndicatorKind::Url => "URL",
            IndicatorKind::FileHash => "FILE_HASH",
            IndicatorKind::Email => "EMAIL",
            IndicatorKind::CertificateHash => "CERT_HASH",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ThreatLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ThreatLevel::Informational => "INFORMATIONAL",
            ThreatLevel::Low => "LOW",
            ThreatLevel::Medium => "MEDIUM",
            ThreatLevel::High => "HIGH",
            ThreatLevel::Critical => "CRITICAL",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Indicator {
    pub id: String,
    pub tenant_id: String,
    pub kind: IndicatorKind,
    pub value: String,
    pub threat_level: ThreatLevel,
    pub source: String,
    pub observed_tick: u64,
    pub expires_tick: Option<u64>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub active: bool,
}

impl Indicator {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        kind: IndicatorKind,
        value: impl Into<String>,
        threat_level: ThreatLevel,
        source: impl Into<String>,
        observed_tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            kind,
            value: value.into(),
            threat_level,
            source: source.into(),
            observed_tick,
            expires_tick: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
            active: true,
        }
    }

    pub fn with_expiry(mut self, tick: u64) -> Self { self.expires_tick = Some(tick); self }
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self { self.tags.push(tag.into()); self }
    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into(), v.into()); self
    }

    pub fn is_expired(&self, current_tick: u64) -> bool {
        self.expires_tick.map_or(false, |e| current_tick >= e)
    }

    pub fn deactivate(&mut self) { self.active = false; }
}
