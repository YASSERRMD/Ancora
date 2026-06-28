use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostureLevel {
    Critical,
    Poor,
    Fair,
    Good,
    Excellent,
}

impl fmt::Display for PostureLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PostureLevel::Critical => "CRITICAL",
            PostureLevel::Poor => "POOR",
            PostureLevel::Fair => "FAIR",
            PostureLevel::Good => "GOOD",
            PostureLevel::Excellent => "EXCELLENT",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct DomainScore {
    pub domain: String,
    pub score: u8,
    pub findings: u32,
    pub critical_findings: u32,
}

impl DomainScore {
    pub fn new(domain: impl Into<String>, score: u8, findings: u32, critical_findings: u32) -> Self {
        Self { domain: domain.into(), score: score.min(100), findings, critical_findings }
    }
}

#[derive(Debug, Clone)]
pub struct SecurityPosture {
    pub tenant_id: String,
    domain_scores: HashMap<String, DomainScore>,
    pub tick: u64,
}

impl SecurityPosture {
    pub fn new(tenant_id: impl Into<String>, tick: u64) -> Self {
        Self { tenant_id: tenant_id.into(), domain_scores: HashMap::new(), tick }
    }

    pub fn add_domain(&mut self, score: DomainScore) {
        self.domain_scores.insert(score.domain.clone(), score);
    }

    pub fn overall_score(&self) -> u8 {
        if self.domain_scores.is_empty() { return 0; }
        let sum: u32 = self.domain_scores.values().map(|d| d.score as u32).sum();
        (sum / self.domain_scores.len() as u32) as u8
    }

    pub fn posture_level(&self) -> PostureLevel {
        match self.overall_score() {
            0..=29 => PostureLevel::Critical,
            30..=49 => PostureLevel::Poor,
            50..=69 => PostureLevel::Fair,
            70..=84 => PostureLevel::Good,
            _ => PostureLevel::Excellent,
        }
    }

    pub fn total_critical_findings(&self) -> u32 {
        self.domain_scores.values().map(|d| d.critical_findings).sum()
    }

    pub fn domain_count(&self) -> usize { self.domain_scores.len() }
    pub fn get_domain(&self, name: &str) -> Option<&DomainScore> { self.domain_scores.get(name) }
    pub fn domains(&self) -> impl Iterator<Item = &DomainScore> { self.domain_scores.values() }
}
