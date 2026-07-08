use crate::sbom::Sbom;
use std::collections::HashMap;

pub struct SbomStats {
    pub total_components: usize,
    pub by_kind: HashMap<String, usize>,
    pub by_license: HashMap<String, usize>,
    pub open_source_count: usize,
}

impl SbomStats {
    pub fn from(sbom: &Sbom) -> Self {
        let mut by_kind = HashMap::new();
        let mut by_license = HashMap::new();
        let mut oss = 0;
        for c in &sbom.components {
            *by_kind.entry(format!("{}", c.kind)).or_insert(0) += 1;
            *by_license.entry(format!("{}", c.license)).or_insert(0) += 1;
            if c.is_open_source() {
                oss += 1;
            }
        }
        Self {
            total_components: sbom.component_count(),
            by_kind,
            by_license,
            open_source_count: oss,
        }
    }

    pub fn oss_rate(&self) -> f64 {
        if self.total_components == 0 {
            return 0.0;
        }
        self.open_source_count as f64 / self.total_components as f64
    }
}
