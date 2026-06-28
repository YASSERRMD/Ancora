use crate::indicator::{Indicator, ThreatLevel};
use crate::score::ThreatScore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Block,
    Alert,
    Monitor,
    Allow,
}

pub struct ThreatPolicy {
    pub tenant_id: String,
    pub block_threshold: f64,
    pub alert_threshold: f64,
    pub min_confidence: f64,
}

impl ThreatPolicy {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self { tenant_id: tenant_id.into(), block_threshold: 70.0, alert_threshold: 40.0, min_confidence: 0.5 }
    }

    pub fn block_threshold(mut self, t: f64) -> Self { self.block_threshold = t; self }
    pub fn alert_threshold(mut self, t: f64) -> Self { self.alert_threshold = t; self }
    pub fn min_confidence(mut self, c: f64) -> Self { self.min_confidence = c; self }

    pub fn evaluate(&self, score: &ThreatScore) -> PolicyDecision {
        if score.confidence < self.min_confidence { return PolicyDecision::Monitor; }
        if score.raw_score >= self.block_threshold { PolicyDecision::Block }
        else if score.raw_score >= self.alert_threshold { PolicyDecision::Alert }
        else if score.raw_score > 0.0 { PolicyDecision::Monitor }
        else { PolicyDecision::Allow }
    }

    pub fn should_block_indicator(&self, indicator: &Indicator) -> bool {
        indicator.threat_level >= ThreatLevel::High && indicator.active
    }
}
