/// Data residency region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Region {
    UsEast,
    EuWest,
    ApSoutheast,
    SelfHosted,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Region::UsEast => "us-east",
            Region::EuWest => "eu-west",
            Region::ApSoutheast => "ap-southeast",
            Region::SelfHosted => "self-hosted",
        };
        write!(f, "{}", s)
    }
}

/// Privacy posture for the observability data pipeline.
#[derive(Debug, Clone)]
pub struct PrivacySummary {
    pub data_at_rest_encrypted: bool,
    pub data_in_transit_encrypted: bool,
    pub pii_scrubbing_enabled: bool,
    pub audit_log_enabled: bool,
    pub retention_days: u32,
    pub residency: Region,
}

impl PrivacySummary {
    pub fn new(region: Region, retention_days: u32) -> Self {
        Self {
            data_at_rest_encrypted: true,
            data_in_transit_encrypted: true,
            pii_scrubbing_enabled: false,
            audit_log_enabled: false,
            retention_days,
            residency: region,
        }
    }

    pub fn with_pii_scrubbing(mut self) -> Self {
        self.pii_scrubbing_enabled = true;
        self
    }

    pub fn with_audit_log(mut self) -> Self {
        self.audit_log_enabled = true;
        self
    }

    pub fn compliance_ready(&self) -> bool {
        self.data_at_rest_encrypted
            && self.data_in_transit_encrypted
            && self.pii_scrubbing_enabled
            && self.audit_log_enabled
    }

    pub fn render(&self) -> String {
        format!(
            "Privacy Summary\n\
             Region: {}\n\
             Retention: {} days\n\
             At-rest encryption: {}\n\
             In-transit encryption: {}\n\
             PII scrubbing: {}\n\
             Audit log: {}\n\
             Compliance ready: {}",
            self.residency,
            self.retention_days,
            self.data_at_rest_encrypted,
            self.data_in_transit_encrypted,
            self.pii_scrubbing_enabled,
            self.audit_log_enabled,
            self.compliance_ready(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_not_compliance_ready() {
        let s = PrivacySummary::new(Region::UsEast, 30);
        assert!(!s.compliance_ready());
    }

    #[test]
    fn full_posture_compliance_ready() {
        let s = PrivacySummary::new(Region::EuWest, 90)
            .with_pii_scrubbing()
            .with_audit_log();
        assert!(s.compliance_ready());
    }

    #[test]
    fn render_contains_region() {
        let s = PrivacySummary::new(Region::SelfHosted, 365);
        let r = s.render();
        assert!(r.contains("self-hosted"));
    }
}
