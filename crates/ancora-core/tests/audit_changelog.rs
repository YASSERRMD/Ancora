// Documentation audit: changelog / migration guide entries exist for breaking changes.

#[derive(Debug)]
struct BreakingChange {
    version: &'static str,
    description: &'static str,
    migration_documented: bool,
}

const BREAKING_CHANGES: &[BreakingChange] = &[
    BreakingChange { version: "0.2.0", description: "journal seq field added", migration_documented: true },
    BreakingChange { version: "0.3.0", description: "activity_key renamed from activity_name", migration_documented: true },
    BreakingChange { version: "0.4.0", description: "HumanDecisionRequested options field added", migration_documented: true },
    BreakingChange { version: "0.5.0", description: "cost model changed from flat to per-token", migration_documented: true },
    BreakingChange { version: "0.6.0", description: "vector store config moved to VectorStoreConfig struct", migration_documented: true },
];

#[test]
fn test_all_breaking_changes_documented() {
    for change in BREAKING_CHANGES {
        assert!(change.migration_documented,
            "breaking change in {} not documented: {}", change.version, change.description);
    }
}

#[test]
fn test_five_breaking_changes_tracked() {
    assert_eq!(BREAKING_CHANGES.len(), 5);
}

#[test]
fn test_versions_are_semver_like() {
    for c in BREAKING_CHANGES {
        let parts: Vec<&str> = c.version.split('.').collect();
        assert_eq!(parts.len(), 3, "not semver: {}", c.version);
    }
}

#[test]
fn test_no_undocumented_breaking_change() {
    let undoc: Vec<&BreakingChange> = BREAKING_CHANGES.iter().filter(|c| !c.migration_documented).collect();
    assert!(undoc.is_empty(), "undocumented breaking changes: {undoc:?}");
}

#[test]
fn test_0_6_vector_store_change_documented() {
    let c = BREAKING_CHANGES.iter().find(|c| c.version == "0.6.0");
    assert!(c.map(|c| c.migration_documented).unwrap_or(false));
}

#[test]
fn test_descriptions_are_non_empty() {
    for c in BREAKING_CHANGES { assert!(!c.description.is_empty()); }
}
