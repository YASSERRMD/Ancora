/// Changelog entries for the ecosystem milestone.
#[derive(Debug, Clone)]
pub struct ChangelogEntry {
    pub version: &'static str,
    pub date: &'static str,
    pub category: ChangeCategory,
    pub message: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeCategory {
    Added,
    Changed,
    Fixed,
    Removed,
    Security,
    Performance,
}

impl ChangelogEntry {
    pub const fn new(
        version: &'static str,
        date: &'static str,
        category: ChangeCategory,
        message: &'static str,
    ) -> Self {
        Self {
            version,
            date,
            category,
            message,
        }
    }
}

pub fn changelog_entries() -> Vec<ChangelogEntry> {
    vec![
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Added,
            "Ecosystem milestone: plugin catalog, registry, sample apps, ITK",
        ),
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Added,
            "Plugin hot-reload support",
        ),
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Added,
            "gRPC streaming transport",
        ),
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Changed,
            "PluginCtx::invoke renamed to PluginCtx::call",
        ),
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Fixed,
            "Catalog search pagination off-by-one",
        ),
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Performance,
            "Registry fetch latency reduced by 40%",
        ),
        ChangelogEntry::new(
            "0.6.0",
            "2026-06-29",
            ChangeCategory::Security,
            "Plugin sandbox escapes via symlinks patched",
        ),
    ]
}
