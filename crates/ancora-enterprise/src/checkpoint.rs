use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
    NotApplicable,
}

impl fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CheckStatus::Pass => "PASS",
            CheckStatus::Warn => "WARN",
            CheckStatus::Fail => "FAIL",
            CheckStatus::NotApplicable => "NOT_APPLICABLE",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub status: CheckStatus,
    pub message: String,
    pub tick: u64,
}

impl HealthCheck {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        domain: impl Into<String>,
        status: CheckStatus,
        message: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            domain: domain.into(),
            status,
            message: message.into(),
            tick,
        }
    }

    pub fn is_healthy(&self) -> bool { self.status == CheckStatus::Pass }
    pub fn is_failing(&self) -> bool { self.status == CheckStatus::Fail }
    pub fn is_warning(&self) -> bool { self.status == CheckStatus::Warn }
}

pub struct EnterpriseCheckpoint {
    checks: Vec<HealthCheck>,
    pub tick: u64,
}

impl EnterpriseCheckpoint {
    pub fn new(tick: u64) -> Self { Self { checks: Vec::new(), tick } }
    pub fn add(&mut self, check: HealthCheck) { self.checks.push(check); }
    pub fn count(&self) -> usize { self.checks.len() }
    pub fn passing(&self) -> Vec<&HealthCheck> { self.checks.iter().filter(|c| c.is_healthy()).collect() }
    pub fn failing(&self) -> Vec<&HealthCheck> { self.checks.iter().filter(|c| c.is_failing()).collect() }
    pub fn warnings(&self) -> Vec<&HealthCheck> { self.checks.iter().filter(|c| c.is_warning()).collect() }
    pub fn for_domain<'a>(&'a self, domain: &str) -> Vec<&'a HealthCheck> {
        self.checks.iter().filter(|c| c.domain == domain).collect()
    }
    pub fn all_healthy(&self) -> bool { self.failing().is_empty() }
    pub fn pass_rate(&self) -> f64 {
        if self.checks.is_empty() { return 0.0; }
        self.passing().len() as f64 / self.checks.len() as f64
    }
}
