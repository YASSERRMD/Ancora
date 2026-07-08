use crate::control::ComplianceControl;
use crate::framework::Framework;

pub fn soc2_controls() -> Vec<ComplianceControl> {
    vec![
        ComplianceControl::new(
            "CC6.1",
            Framework::Soc2,
            "Logical access controls",
            "Access to system components is restricted",
        ),
        ComplianceControl::new(
            "CC6.2",
            Framework::Soc2,
            "Authentication mechanisms",
            "User authentication is enforced",
        ),
        ComplianceControl::new(
            "CC7.1",
            Framework::Soc2,
            "Change management",
            "System changes are controlled and tested",
        ),
        ComplianceControl::new(
            "CC8.1",
            Framework::Soc2,
            "Incident response",
            "Security incidents are identified and responded to",
        ),
        ComplianceControl::new(
            "A1.1",
            Framework::Soc2,
            "Availability monitoring",
            "System availability is monitored and measured",
        ),
    ]
}

pub fn iso27001_controls() -> Vec<ComplianceControl> {
    vec![
        ComplianceControl::new(
            "A.5.1",
            Framework::Iso27001,
            "Information security policies",
            "Policies are defined and approved by management",
        ),
        ComplianceControl::new(
            "A.6.1",
            Framework::Iso27001,
            "Information security roles",
            "Roles and responsibilities are defined",
        ),
        ComplianceControl::new(
            "A.9.1",
            Framework::Iso27001,
            "Access control policy",
            "Access control is implemented",
        ),
        ComplianceControl::new(
            "A.12.1",
            Framework::Iso27001,
            "Operational procedures",
            "Documented operating procedures exist",
        ),
        ComplianceControl::new(
            "A.16.1",
            Framework::Iso27001,
            "Incident management",
            "Incidents are reported and managed",
        ),
    ]
}

pub fn fedramp_controls() -> Vec<ComplianceControl> {
    vec![
        ComplianceControl::new(
            "AC-1",
            Framework::Fedramp,
            "Access control policy",
            "Access control policy is documented",
        ),
        ComplianceControl::new(
            "AU-2",
            Framework::Fedramp,
            "Event logging",
            "Audit events are defined and logged",
        ),
        ComplianceControl::new(
            "IA-2",
            Framework::Fedramp,
            "Identification and authentication",
            "Users are uniquely identified",
        ),
        ComplianceControl::new(
            "SC-7",
            Framework::Fedramp,
            "Boundary protection",
            "Network boundaries are protected",
        ),
        ComplianceControl::new(
            "SI-2",
            Framework::Fedramp,
            "Flaw remediation",
            "Security flaws are identified and remediated",
        ),
    ]
}
