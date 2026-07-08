use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceKind {
    BuildSystem,
    Vcs,
    Registry,
    ArtifactStore,
}

impl fmt::Display for ProvenanceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProvenanceKind::BuildSystem => "BUILD_SYSTEM",
            ProvenanceKind::Vcs => "VCS",
            ProvenanceKind::Registry => "REGISTRY",
            ProvenanceKind::ArtifactStore => "ARTIFACT_STORE",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct ProvenanceRecord {
    pub component_id: String,
    pub kind: ProvenanceKind,
    pub source: String,
    pub build_id: String,
    pub tick: u64,
    pub metadata: HashMap<String, String>,
}

impl ProvenanceRecord {
    pub fn new(
        component_id: impl Into<String>,
        kind: ProvenanceKind,
        source: impl Into<String>,
        build_id: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            component_id: component_id.into(),
            kind,
            source: source.into(),
            build_id: build_id.into(),
            tick,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

pub struct ProvenanceStore {
    records: HashMap<String, ProvenanceRecord>,
}

impl Default for ProvenanceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvenanceStore {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }
    pub fn record(&mut self, r: ProvenanceRecord) {
        self.records.insert(r.component_id.clone(), r);
    }
    pub fn get(&self, component_id: &str) -> Option<&ProvenanceRecord> {
        self.records.get(component_id)
    }
    pub fn has_provenance(&self, component_id: &str) -> bool {
        self.records.contains_key(component_id)
    }
    pub fn count(&self) -> usize {
        self.records.len()
    }
    pub fn by_kind(&self, kind: &ProvenanceKind) -> Vec<&ProvenanceRecord> {
        self.records.values().filter(|r| &r.kind == kind).collect()
    }
}
