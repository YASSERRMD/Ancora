/// Ecosystem index: top-level navigation for the milestone.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub title: &'static str,
    pub module: &'static str,
    pub description: &'static str,
}

impl IndexEntry {
    pub const fn new(title: &'static str, module: &'static str, description: &'static str) -> Self {
        Self { title, module, description }
    }
}

pub fn ecosystem_index() -> Vec<IndexEntry> {
    vec![
        IndexEntry::new("Suite Status", "suite_status", "Overall test suite pass/fail status"),
        IndexEntry::new("E2E Status", "e2e_status", "End-to-end scenario results"),
        IndexEntry::new("Apps Status", "apps_status", "Sample application run results"),
        IndexEntry::new("ITK Status", "itk_status", "Interop toolkit test results"),
        IndexEntry::new("Feature Matrix", "feature_matrix", "Feature support across tiers"),
        IndexEntry::new("Limitations", "limitations", "Known limitations and workarounds"),
        IndexEntry::new("Upgrade Notes", "upgrade_notes", "Migration guide between versions"),
        IndexEntry::new("Changelog", "changelog", "Version history and change categories"),
        IndexEntry::new("Quickstart", "quickstart", "Extension author getting-started guide"),
        IndexEntry::new("Registry Links", "registry_links", "Registry and catalog documentation links"),
        IndexEntry::new("Trust Summary", "trust_summary", "Trust and governance posture"),
        IndexEntry::new("Announcement", "announcement", "Release announcement draft"),
        IndexEntry::new("Readiness", "readiness", "Go/no-go readiness checklist"),
    ]
}

pub fn entry_count() -> usize {
    ecosystem_index().len()
}
