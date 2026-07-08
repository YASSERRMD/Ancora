use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};

pub struct IndicatorQuery {
    pub kind: Option<IndicatorKind>,
    pub min_level: Option<ThreatLevel>,
    pub source: Option<String>,
    pub tag: Option<String>,
    pub active_only: bool,
}

impl Default for IndicatorQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl IndicatorQuery {
    pub fn new() -> Self {
        Self {
            kind: None,
            min_level: None,
            source: None,
            tag: None,
            active_only: false,
        }
    }
    pub fn kind(mut self, k: IndicatorKind) -> Self {
        self.kind = Some(k);
        self
    }
    pub fn min_level(mut self, l: ThreatLevel) -> Self {
        self.min_level = Some(l);
        self
    }
    pub fn source(mut self, s: impl Into<String>) -> Self {
        self.source = Some(s.into());
        self
    }
    pub fn tag(mut self, t: impl Into<String>) -> Self {
        self.tag = Some(t.into());
        self
    }
    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }

    pub fn run<'a>(&self, indicators: impl Iterator<Item = &'a Indicator>) -> Vec<&'a Indicator> {
        indicators
            .filter(|i| {
                if let Some(ref k) = self.kind {
                    if &i.kind != k {
                        return false;
                    }
                }
                if let Some(ref ml) = self.min_level {
                    if i.threat_level < *ml {
                        return false;
                    }
                }
                if let Some(ref s) = self.source {
                    if i.source != *s {
                        return false;
                    }
                }
                if let Some(ref t) = self.tag {
                    if !i.tags.contains(t) {
                        return false;
                    }
                }
                if self.active_only && !i.active {
                    return false;
                }
                true
            })
            .collect()
    }
}
