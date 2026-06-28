/// Severity level of a security vulnerability.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of a security disclosure report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisclosureStatus {
    Received,
    Triaging,
    Confirmed,
    Patched,
    Disclosed,
}

/// A security vulnerability report for an extension.
#[derive(Debug, Clone)]
pub struct SecurityDisclosure {
    pub id: String,
    pub extension_id: String,
    pub severity: Severity,
    pub description: String,
    pub status: DisclosureStatus,
}

impl SecurityDisclosure {
    pub fn new(
        id: impl Into<String>,
        extension_id: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
    ) -> Self {
        SecurityDisclosure {
            id: id.into(),
            extension_id: extension_id.into(),
            severity,
            description: description.into(),
            status: DisclosureStatus::Received,
        }
    }

    /// Advance the disclosure to the next status.
    pub fn advance(&mut self) -> Result<(), String> {
        use DisclosureStatus::*;
        self.status = match self.status {
            Received => Triaging,
            Triaging => Confirmed,
            Confirmed => Patched,
            Patched => Disclosed,
            Disclosed => return Err("already disclosed".to_string()),
        };
        Ok(())
    }

    /// Returns true if the vulnerability has been patched.
    pub fn is_patched(&self) -> bool {
        matches!(
            self.status,
            DisclosureStatus::Patched | DisclosureStatus::Disclosed
        )
    }
}

/// Registry of security disclosures.
#[derive(Debug, Default)]
pub struct DisclosureRegistry {
    disclosures: Vec<SecurityDisclosure>,
}

impl DisclosureRegistry {
    pub fn new() -> Self {
        DisclosureRegistry { disclosures: Vec::new() }
    }

    pub fn report(&mut self, disclosure: SecurityDisclosure) {
        self.disclosures.push(disclosure);
    }

    pub fn open_count(&self) -> usize {
        self.disclosures
            .iter()
            .filter(|d| !d.is_patched())
            .count()
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut SecurityDisclosure> {
        self.disclosures.iter_mut().find(|d| d.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disclosure_advances_to_patched() {
        let mut d = SecurityDisclosure::new(
            "CVE-2026-001",
            "my-ext",
            Severity::High,
            "Arbitrary code execution via hook",
        );
        d.advance().unwrap(); // Triaging
        d.advance().unwrap(); // Confirmed
        d.advance().unwrap(); // Patched
        assert!(d.is_patched());
    }

    #[test]
    fn disclosed_cannot_advance() {
        let mut d = SecurityDisclosure::new(
            "CVE-2026-002",
            "other-ext",
            Severity::Low,
            "Info leak",
        );
        for _ in 0..4 {
            d.advance().unwrap();
        }
        assert!(d.advance().is_err());
    }
}
