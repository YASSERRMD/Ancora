use crate::indicator::{Indicator, ThreatLevel};

pub struct ThreatScore {
    pub indicator_id: String,
    pub raw_score: f64,
    pub level: ThreatLevel,
    pub confidence: f64,
}

impl ThreatScore {
    pub fn new(indicator_id: impl Into<String>, raw_score: f64, confidence: f64) -> Self {
        let level = Self::level_for_score(raw_score);
        Self { indicator_id: indicator_id.into(), raw_score, level, confidence }
    }

    fn level_for_score(score: f64) -> ThreatLevel {
        if score >= 90.0 { ThreatLevel::Critical }
        else if score >= 70.0 { ThreatLevel::High }
        else if score >= 40.0 { ThreatLevel::Medium }
        else if score >= 10.0 { ThreatLevel::Low }
        else { ThreatLevel::Informational }
    }

    pub fn is_actionable(&self) -> bool {
        self.level >= ThreatLevel::Medium && self.confidence >= 0.5
    }
}

pub struct ThreatScorer;

impl ThreatScorer {
    pub fn score(indicator: &Indicator, recency_ticks: u64, max_recency: u64) -> ThreatScore {
        let base = match indicator.threat_level {
            ThreatLevel::Critical => 95.0,
            ThreatLevel::High => 75.0,
            ThreatLevel::Medium => 50.0,
            ThreatLevel::Low => 25.0,
            ThreatLevel::Informational => 5.0,
        };
        let recency_factor = if max_recency == 0 { 1.0 } else {
            1.0 - (recency_ticks as f64 / max_recency as f64).min(1.0)
        };
        let adjusted = base * (0.5 + 0.5 * recency_factor);
        let confidence = if indicator.tags.is_empty() { 0.6 } else { 0.8 };
        ThreatScore::new(indicator.id.clone(), adjusted.min(100.0), confidence)
    }
}
