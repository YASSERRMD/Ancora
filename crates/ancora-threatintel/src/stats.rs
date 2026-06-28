use std::collections::HashMap;
use crate::indicator::{Indicator, ThreatLevel};

pub struct ThreatIntelStats {
    pub tenant_id: String,
    pub total_indicators: usize,
    pub active_indicators: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub by_kind: HashMap<String, usize>,
    pub by_level: HashMap<String, usize>,
}

impl ThreatIntelStats {
    pub fn for_tenant(indicators: &[&Indicator], tenant_id: &str) -> Self {
        let tenant: Vec<&&Indicator> = indicators.iter().filter(|i| i.tenant_id == tenant_id).collect();
        let total_indicators = tenant.len();
        let active_indicators = tenant.iter().filter(|i| i.active).count();
        let critical_count = tenant.iter().filter(|i| i.threat_level == ThreatLevel::Critical).count();
        let high_count = tenant.iter().filter(|i| i.threat_level == ThreatLevel::High).count();
        let mut by_kind = HashMap::new();
        let mut by_level = HashMap::new();
        for i in &tenant {
            *by_kind.entry(format!("{}", i.kind)).or_insert(0) += 1;
            *by_level.entry(format!("{}", i.threat_level)).or_insert(0) += 1;
        }
        Self { tenant_id: tenant_id.to_string(), total_indicators, active_indicators, critical_count, high_count, by_kind, by_level }
    }

    pub fn is_critical_free(&self) -> bool { self.critical_count == 0 }
}
