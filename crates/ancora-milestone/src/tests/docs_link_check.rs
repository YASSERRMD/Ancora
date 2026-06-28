/// Verify that milestone docs exist on disk.
///
/// In CI the docs-check job in the workflow does the same via shell commands.
#[test]
fn milestone_docs_present() {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let docs = [
        "docs/milestone/advanced-capabilities-overview.md",
        "docs/milestone/feature-matrix.md",
        "docs/milestone/known-limitations.md",
        "docs/milestone/upgrade-notes.md",
        "docs/milestone/changelog.md",
        "docs/milestone/per-language-quickstarts.md",
        "docs/milestone/security-review-notes.md",
        "docs/milestone/government-preset-readiness.md",
        "docs/milestone/announcement.md",
        "docs/milestone/roadmap-to-part2.md",
    ];
    for rel in &docs {
        let full = workspace_root.join(rel);
        assert!(full.exists(), "missing milestone doc: {rel}");
    }
}
