use crate::license::{require_license, LicenseClass, LicenseDeclaration, LicenseError};

#[test]
fn open_source_license_is_required_and_accepted() {
    let decl = LicenseDeclaration::new("MIT", true).unwrap();
    assert!(require_license(Some(&decl), false).is_ok());
}

#[test]
fn missing_license_is_rejected() {
    assert_eq!(
        require_license(None, false),
        Err(LicenseError::DeclarationMissing)
    );
}

#[test]
fn proprietary_blocked_by_strict_policy() {
    let decl = LicenseDeclaration::new("PROPRIETARY", false).unwrap();
    assert_eq!(
        require_license(Some(&decl), true),
        Err(LicenseError::LicenseClassBlocked(LicenseClass::Proprietary))
    );
}

#[test]
fn proprietary_allowed_by_permissive_policy() {
    let decl = LicenseDeclaration::new("PROPRIETARY", false).unwrap();
    assert!(require_license(Some(&decl), false).is_ok());
}

#[test]
fn gpl_classified_as_copyleft() {
    let decl = LicenseDeclaration::new("GPL-3.0", true).unwrap();
    assert_eq!(decl.classify(), LicenseClass::Copyleft);
}

#[test]
fn apache_classified_as_permissive() {
    let decl = LicenseDeclaration::new("Apache-2.0", true).unwrap();
    assert_eq!(decl.classify(), LicenseClass::Permissive);
}
