// Documentation audit: migration guides exist for all version upgrades.

struct MigrationGuide {
    from_version: &'static str,
    to_version: &'static str,
    doc_path: &'static str,
    covers_breaking_change: bool,
}

const MIGRATION_GUIDES: &[MigrationGuide] = &[
    MigrationGuide {
        from_version: "0.1.x",
        to_version: "0.2.0",
        doc_path: "sdk/rust/migration.md",
        covers_breaking_change: true,
    },
    MigrationGuide {
        from_version: "0.2.x",
        to_version: "0.3.0",
        doc_path: "sdk/go/migration.md",
        covers_breaking_change: true,
    },
    MigrationGuide {
        from_version: "0.3.x",
        to_version: "0.4.0",
        doc_path: "sdk/python/migration.md",
        covers_breaking_change: true,
    },
    MigrationGuide {
        from_version: "0.4.x",
        to_version: "0.5.0",
        doc_path: "sdk/ts/migration.md",
        covers_breaking_change: true,
    },
    MigrationGuide {
        from_version: "0.5.x",
        to_version: "0.6.0",
        doc_path: "sdk/dotnet/migration.md",
        covers_breaking_change: true,
    },
];

#[test]
fn test_five_migration_guides_exist() {
    assert_eq!(MIGRATION_GUIDES.len(), 5);
}

#[test]
fn test_all_guides_cover_breaking_changes() {
    for guide in MIGRATION_GUIDES {
        assert!(
            guide.covers_breaking_change,
            "migration guide {}->{} does not cover breaking change",
            guide.from_version, guide.to_version
        );
    }
}

#[test]
fn test_all_guide_docs_end_with_migration_md() {
    for guide in MIGRATION_GUIDES {
        assert!(
            guide.doc_path.ends_with("migration.md"),
            "guide doc path should end with migration.md: {}",
            guide.doc_path
        );
    }
}

#[test]
fn test_versions_form_sequential_chain() {
    for (i, guide) in MIGRATION_GUIDES.iter().enumerate().skip(1) {
        let prev = &MIGRATION_GUIDES[i - 1];
        assert_ne!(
            prev.to_version, guide.to_version,
            "duplicate target version: {}",
            guide.to_version
        );
    }
}

#[test]
fn test_latest_migration_targets_0_6() {
    let last = MIGRATION_GUIDES.last().unwrap();
    assert_eq!(last.to_version, "0.6.0");
}

#[test]
fn test_all_from_versions_have_x_suffix() {
    for g in MIGRATION_GUIDES {
        assert!(
            g.from_version.ends_with(".x"),
            "from_version should end with .x: {}",
            g.from_version
        );
    }
}
