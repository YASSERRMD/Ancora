use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Framework {
    Soc2,
    Fedramp,
    Iso27001,
    Pci,
    Hipaa,
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Framework::Soc2 => write!(f, "SOC 2"),
            Framework::Fedramp => write!(f, "FedRAMP"),
            Framework::Iso27001 => write!(f, "ISO 27001"),
            Framework::Pci => write!(f, "PCI DSS"),
            Framework::Hipaa => write!(f, "HIPAA"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlId(pub String);

impl ControlId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }
}

impl fmt::Display for ControlId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}
