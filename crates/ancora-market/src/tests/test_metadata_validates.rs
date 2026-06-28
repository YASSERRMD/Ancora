use crate::metadata_schema::{ExtensionMetadata, MetadataError};

#[test]
fn valid_metadata_passes_validation() {
    let m = ExtensionMetadata::new(
        "com.ancora.example",
        "Example Extension",
        "2.0.1",
        "An example extension for testing.",
        "MIT",
    )
    .unwrap();
    assert!(m.validate().is_ok());
}

#[test]
fn empty_name_fails_validation() {
    let result = ExtensionMetadata::new("com.ancora.example", "", "1.0.0", "desc", "MIT");
    assert_eq!(result, Err(MetadataError::EmptyName));
}

#[test]
fn description_over_limit_fails() {
    let long_desc: String = "a".repeat(257);
    let result = ExtensionMetadata::new("id", "Name", "1.0.0", long_desc, "MIT");
    assert!(matches!(result, Err(MetadataError::DescriptionTooLong(_))));
}

#[test]
fn invalid_version_format_fails() {
    let result = ExtensionMetadata::new("id", "Name", "v1.0", "desc", "MIT");
    assert!(matches!(result, Err(MetadataError::InvalidVersion(_))));
}

#[test]
fn empty_license_fails() {
    let result = ExtensionMetadata::new("id", "Name", "1.0.0", "desc", "");
    assert_eq!(result, Err(MetadataError::EmptyLicense));
}
