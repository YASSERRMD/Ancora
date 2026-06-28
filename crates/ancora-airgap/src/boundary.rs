use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneClassification {
    Public,
    Internal,
    Restricted,
    TopSecret,
}

impl fmt::Display for ZoneClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ZoneClassification::Public => "PUBLIC",
            ZoneClassification::Internal => "INTERNAL",
            ZoneClassification::Restricted => "RESTRICTED",
            ZoneClassification::TopSecret => "TOP_SECRET",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct AirGapZone {
    pub id: String,
    pub name: String,
    pub classification: ZoneClassification,
    pub tenant_id: String,
    pub metadata: HashMap<String, String>,
}

impl AirGapZone {
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
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into(), v.into());
        self
    }

    pub fn is_restricted(&self) -> bool {
        matches!(
            self.classification,
            ZoneClassification::Restricted | ZoneClassification::TopSecret
        )
    }
}

pub struct AirGapBoundary {
    zones: HashMap<String, AirGapZone>,
}

impl AirGapBoundary {
    pub fn new() -> Self { Self { zones: HashMap::new() } }

    pub fn add_zone(&mut self, zone: AirGapZone) {
        self.zones.insert(zone.id.clone(), zone);
    }

    pub fn get(&self, id: &str) -> Option<&AirGapZone> { self.zones.get(id) }

    pub fn restricted_zones(&self) -> Vec<&AirGapZone> {
        self.zones.values().filter(|z| z.is_restricted()).collect()
    }

    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&AirGapZone> {
        self.zones.values().filter(|z| z.tenant_id == tenant_id).collect()
    }

    pub fn count(&self) -> usize { self.zones.len() }
}
