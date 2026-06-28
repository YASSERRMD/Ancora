use std::collections::HashMap;
use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};

pub struct IndicatorStore {
    indicators: HashMap<String, Indicator>,
}

impl IndicatorStore {
    pub fn new() -> Self { Self { indicators: HashMap::new() } }

    pub fn insert(&mut self, indicator: Indicator) {
        self.indicators.insert(indicator.id.clone(), indicator);
    }

    pub fn get(&self, id: &str) -> Option<&Indicator> { self.indicators.get(id) }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Indicator> { self.indicators.get_mut(id) }

    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a Indicator> {
        self.indicators.values().filter(|i| i.tenant_id == tenant_id).collect()
    }

    pub fn by_kind<'a>(&'a self, kind: &IndicatorKind) -> Vec<&'a Indicator> {
        self.indicators.values().filter(|i| &i.kind == kind).collect()
    }

    pub fn by_threat_level<'a>(&'a self, level: &ThreatLevel) -> Vec<&'a Indicator> {
        self.indicators.values().filter(|i| &i.threat_level == level).collect()
    }

    pub fn active<'a>(&'a self) -> Vec<&'a Indicator> {
        self.indicators.values().filter(|i| i.active).collect()
    }

    pub fn expired<'a>(&'a self, current_tick: u64) -> Vec<&'a Indicator> {
        self.indicators.values().filter(|i| i.is_expired(current_tick)).collect()
    }

    pub fn count(&self) -> usize { self.indicators.len() }

    pub fn by_value<'a>(&'a self, value: &str) -> Vec<&'a Indicator> {
        self.indicators.values().filter(|i| i.value == value).collect()
    }
}
