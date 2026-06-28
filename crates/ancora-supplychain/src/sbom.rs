use std::collections::HashMap;
use crate::component::Component;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SbomFormat {
    CycloneDx,
    Spdx,
    Internal,
}

pub struct Sbom {
    pub id: String,
    pub tenant_id: String,
    pub format: SbomFormat,
    pub components: Vec<Component>,
    pub created_tick: u64,
    pub metadata: HashMap<String, String>,
}

impl Sbom {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        format: SbomFormat,
        created_tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            format,
            components: Vec::new(),
            created_tick,
            metadata: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, c: Component) {
        self.components.push(c);
    }

    pub fn component_count(&self) -> usize { self.components.len() }

    pub fn find_by_name(&self, name: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.name == name)
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.id == id)
    }

    pub fn proprietary_count(&self) -> usize {
        self.components.iter().filter(|c| !c.is_open_source()).count()
    }
}

pub struct SbomStore {
    sboms: HashMap<String, Sbom>,
}

impl SbomStore {
    pub fn new() -> Self { Self { sboms: HashMap::new() } }
    pub fn insert(&mut self, sbom: Sbom) { self.sboms.insert(sbom.id.clone(), sbom); }
    pub fn get(&self, id: &str) -> Option<&Sbom> { self.sboms.get(id) }
    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&Sbom> {
        self.sboms.values().filter(|s| s.tenant_id == tenant_id).collect()
    }
    pub fn count(&self) -> usize { self.sboms.len() }
}
