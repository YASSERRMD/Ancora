//! Residency and data-handling declaration for marketplace extensions.
//!
//! Extensions that process user data must declare where that data is stored
//! and processed. Install policies can restrict extensions based on residency.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    /// European Economic Area.
    EEA,
    /// United States of America.
    US,
    /// United Kingdom.
    UK,
    /// Asia-Pacific.
    APAC,
    /// No data leaves the end-user's device.
    LocalOnly,
    /// Unspecified - the extension makes no claims about data location.
    Unspecified,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::EEA => write!(f, "EEA"),
            Region::US => write!(f, "US"),
            Region::UK => write!(f, "UK"),
            Region::APAC => write!(f, "APAC"),
            Region::LocalOnly => write!(f, "local-only"),
            Region::Unspecified => write!(f, "unspecified"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResidencyDeclaration {
    /// Regions where user data may be stored at rest.
    pub storage_regions: Vec<Region>,
    /// Regions where user data may be processed.
    pub processing_regions: Vec<Region>,
    /// Whether the extension transfers data to third-party services.
    pub third_party_transfer: bool,
    /// Contact email for data-handling questions.
    pub data_contact: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResidencyError {
    DeclarationMissing,
    UnspecifiedRegion,
    RegionNotAllowed(Region),
}

impl std::fmt::Display for ResidencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResidencyError::DeclarationMissing => write!(f, "residency declaration is missing"),
            ResidencyError::UnspecifiedRegion => {
                write!(f, "residency declaration must not use Unspecified region")
            }
            ResidencyError::RegionNotAllowed(r) => {
                write!(f, "region {} is not allowed by install policy", r)
            }
        }
    }
}

impl ResidencyDeclaration {
    pub fn new(storage_regions: Vec<Region>, processing_regions: Vec<Region>) -> Self {
        ResidencyDeclaration {
            storage_regions,
            processing_regions,
            third_party_transfer: false,
            data_contact: None,
        }
    }

    /// Check that the declaration contains no Unspecified entries.
    pub fn is_complete(&self) -> bool {
        !self.storage_regions.contains(&Region::Unspecified)
            && !self.processing_regions.contains(&Region::Unspecified)
    }
}

/// Enforce residency policy on install.
///
/// `allowed_regions` is the set of regions the operator permits. An empty
/// `allowed_regions` slice means all explicitly-declared regions are permitted.
pub fn enforce_residency(
    declaration: Option<&ResidencyDeclaration>,
    allowed_regions: &[Region],
) -> Result<(), ResidencyError> {
    let decl = declaration.ok_or(ResidencyError::DeclarationMissing)?;

    if !decl.is_complete() {
        return Err(ResidencyError::UnspecifiedRegion);
    }

    if allowed_regions.is_empty() {
        return Ok(());
    }

    for region in decl
        .storage_regions
        .iter()
        .chain(decl.processing_regions.iter())
    {
        if !allowed_regions.contains(region) {
            return Err(ResidencyError::RegionNotAllowed(region.clone()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eea_only_policy_accepts_eea_extension() {
        let decl = ResidencyDeclaration::new(vec![Region::EEA], vec![Region::EEA]);
        assert!(enforce_residency(Some(&decl), &[Region::EEA]).is_ok());
    }

    #[test]
    fn eea_only_policy_rejects_us_extension() {
        let decl = ResidencyDeclaration::new(vec![Region::US], vec![Region::US]);
        assert!(matches!(
            enforce_residency(Some(&decl), &[Region::EEA]),
            Err(ResidencyError::RegionNotAllowed(_))
        ));
    }

    #[test]
    fn missing_declaration_rejected() {
        assert_eq!(
            enforce_residency(None, &[]),
            Err(ResidencyError::DeclarationMissing)
        );
    }
}
