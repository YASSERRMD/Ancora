use crate::compat_matrix::CompatMatrix;
use crate::semver::SemVer;
use crate::version_negotiation::{CoreApiVersion, ExtensionManifest};

#[test]
fn compat_matrix_generated_with_multiple_entries() {
    let mut matrix = CompatMatrix::new();

    let ext1 = ExtensionManifest {
        min_api_version: SemVer::new(1, 0, 0),
        max_api_version: SemVer::new(1, 9, 0),
    };
    let ext2 = ExtensionManifest {
        min_api_version: SemVer::new(2, 0, 0),
        max_api_version: SemVer::new(2, 5, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(1, 5, 0),
    };

    matrix.record("ext-alpha", &ext1, &core);
    matrix.record("ext-beta", &ext2, &core);

    assert_eq!(matrix.entries().len(), 2);
    assert_eq!(matrix.compatible_count(), 1);
    assert_eq!(matrix.incompatible_count(), 1);
}

#[test]
fn compat_matrix_report_contains_all_extensions() {
    let mut matrix = CompatMatrix::new();
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 0, 0),
        max_api_version: SemVer::new(1, 9, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(1, 2, 0),
    };
    matrix.record("ext-one", &manifest, &core);
    let report = matrix.generate_report();
    assert!(report.contains("ext-one"));
    assert!(report.contains("Compatible"));
}

#[test]
fn empty_matrix_has_zero_counts() {
    let matrix = CompatMatrix::new();
    assert_eq!(matrix.compatible_count(), 0);
    assert_eq!(matrix.incompatible_count(), 0);
}
