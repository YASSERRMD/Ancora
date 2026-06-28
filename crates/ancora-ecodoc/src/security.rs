//! Security guidelines for Ancora extension authors.
//!
//! Documents security requirements, secret handling policies,
//! and vulnerability disclosure procedures.

/// Security requirement category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityCategory {
    SecretHandling,
    InputValidation,
    DependencyManagement,
    Disclosure,
    Sandboxing,
}

impl std::fmt::Display for SecurityCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::SecretHandling => "secret-handling",
            Self::InputValidation => "input-validation",
            Self::DependencyManagement => "dependency-management",
            Self::Disclosure => "disclosure",
            Self::Sandboxing => "sandboxing",
        };
        write!(f, "{label}")
    }
}

/// A single security requirement.
#[derive(Debug, Clone)]
pub struct SecurityRequirement {
    pub id: &'static str,
    pub category: SecurityCategory,
    pub description: &'static str,
    pub mandatory: bool,
}

/// Returns the full set of security requirements for Ancora plugins.
pub fn security_requirements() -> Vec<SecurityRequirement> {
    vec![
        SecurityRequirement {
            id: "sec-001",
            category: SecurityCategory::SecretHandling,
            description: "Never log or expose secret values in plain text",
            mandatory: true,
        },
        SecurityRequirement {
            id: "sec-002",
            category: SecurityCategory::InputValidation,
            description: "Validate and sanitize all inputs received from external sources",
            mandatory: true,
        },
        SecurityRequirement {
            id: "sec-003",
            category: SecurityCategory::DependencyManagement,
            description: "Pin transitive dependencies and audit with `cargo audit`",
            mandatory: true,
        },
        SecurityRequirement {
            id: "sec-004",
            category: SecurityCategory::Disclosure,
            description: "Report vulnerabilities to security@ancora.dev before public disclosure",
            mandatory: true,
        },
        SecurityRequirement {
            id: "sec-005",
            category: SecurityCategory::Sandboxing,
            description: "Request only the capabilities the plugin genuinely requires",
            mandatory: false,
        },
    ]
}

/// Returns mandatory security requirements.
pub fn mandatory_requirements() -> Vec<SecurityRequirement> {
    security_requirements()
        .into_iter()
        .filter(|r| r.mandatory)
        .collect()
}

/// Checks that a list of requirement IDs covers all mandatory ones.
pub fn covers_mandatory(provided_ids: &[&str]) -> Result<(), Vec<&'static str>> {
    let missing: Vec<&'static str> = mandatory_requirements()
        .iter()
        .filter(|r| !provided_ids.contains(&r.id))
        .map(|r| r.id)
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_requirements_have_ids() {
        for req in security_requirements() {
            assert!(!req.id.is_empty());
        }
    }

    #[test]
    fn mandatory_requirements_are_a_subset() {
        let all = security_requirements();
        let mandatory = mandatory_requirements();
        assert!(mandatory.len() < all.len() || mandatory.len() == all.len());
        for req in &mandatory {
            assert!(req.mandatory);
        }
    }

    #[test]
    fn covers_mandatory_with_all_ids() {
        let ids: Vec<&str> = mandatory_requirements().iter().map(|r| r.id).collect();
        assert!(covers_mandatory(&ids).is_ok());
    }

    #[test]
    fn covers_mandatory_with_missing_id_fails() {
        assert!(covers_mandatory(&[]).is_err());
    }
}
