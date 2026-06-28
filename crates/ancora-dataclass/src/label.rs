use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SensitivityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

impl SensitivityLevel {
    pub fn numeric(&self) -> u8 {
        match self {
            SensitivityLevel::Public => 0,
            SensitivityLevel::Internal => 1,
            SensitivityLevel::Confidential => 2,
            SensitivityLevel::Restricted => 3,
            SensitivityLevel::TopSecret => 4,
        }
    }

    pub fn is_above(&self, other: &SensitivityLevel) -> bool {
        self.numeric() > other.numeric()
    }

    pub fn is_at_least(&self, other: &SensitivityLevel) -> bool {
        self.numeric() >= other.numeric()
    }
}

impl fmt::Display for SensitivityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensitivityLevel::Public => write!(f, "PUBLIC"),
            SensitivityLevel::Internal => write!(f, "INTERNAL"),
            SensitivityLevel::Confidential => write!(f, "CONFIDENTIAL"),
            SensitivityLevel::Restricted => write!(f, "RESTRICTED"),
            SensitivityLevel::TopSecret => write!(f, "TOP_SECRET"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataCategory {
    Pii,
    Financial,
    Health,
    Credentials,
    Intellectual,
    Operational,
    Generic,
}

impl fmt::Display for DataCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataCategory::Pii => write!(f, "PII"),
            DataCategory::Financial => write!(f, "FINANCIAL"),
            DataCategory::Health => write!(f, "HEALTH"),
            DataCategory::Credentials => write!(f, "CREDENTIALS"),
            DataCategory::Intellectual => write!(f, "INTELLECTUAL"),
            DataCategory::Operational => write!(f, "OPERATIONAL"),
            DataCategory::Generic => write!(f, "GENERIC"),
        }
    }
}
