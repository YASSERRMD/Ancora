use crate::residency::{enforce_residency, Region, ResidencyDeclaration, ResidencyError};

#[test]
fn eea_extension_allowed_under_eea_policy() {
    let decl = ResidencyDeclaration::new(vec![Region::EEA], vec![Region::EEA]);
    assert!(enforce_residency(Some(&decl), &[Region::EEA]).is_ok());
}

#[test]
fn us_extension_blocked_under_eea_only_policy() {
    let decl = ResidencyDeclaration::new(vec![Region::US], vec![Region::US]);
    let result = enforce_residency(Some(&decl), &[Region::EEA]);
    assert!(matches!(result, Err(ResidencyError::RegionNotAllowed(_))));
}

#[test]
fn missing_declaration_blocked_by_any_policy() {
    let result = enforce_residency(None, &[Region::EEA]);
    assert_eq!(result, Err(ResidencyError::DeclarationMissing));
}

#[test]
fn empty_policy_allows_any_region() {
    let decl = ResidencyDeclaration::new(vec![Region::APAC], vec![Region::US]);
    assert!(enforce_residency(Some(&decl), &[]).is_ok());
}

#[test]
fn unspecified_region_blocked() {
    let decl = ResidencyDeclaration::new(vec![Region::Unspecified], vec![Region::EEA]);
    let result = enforce_residency(Some(&decl), &[Region::EEA, Region::Unspecified]);
    assert_eq!(result, Err(ResidencyError::UnspecifiedRegion));
}

#[test]
fn local_only_extension_accepted_under_eea_policy() {
    let decl = ResidencyDeclaration::new(vec![Region::LocalOnly], vec![Region::LocalOnly]);
    assert!(enforce_residency(Some(&decl), &[Region::EEA, Region::LocalOnly]).is_ok());
}
