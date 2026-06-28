/// Plugin list command - produces a human-readable table of installed plugins
/// and their status.

use crate::interface::PluginMeta;
use crate::update::UpdateStatus;

/// An entry in the plugin list view.
#[derive(Debug, Clone)]
pub struct ListEntry {
    /// Plugin metadata.
    pub meta: PluginMeta,
    /// Whether the plugin is currently active.
    pub enabled: bool,
    /// Update status, if it has been checked.
    pub update_status: Option<UpdateStatus>,
}

impl ListEntry {
    /// Create a basic enabled entry with no update information.
    pub fn new(meta: PluginMeta) -> Self {
        Self {
            meta,
            enabled: true,
            update_status: None,
        }
    }

    /// Mark the entry as disabled.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Attach an update status.
    pub fn with_update_status(mut self, status: UpdateStatus) -> Self {
        self.update_status = Some(status);
        self
    }
}

/// Options that control how the list is rendered.
#[derive(Debug, Clone)]
pub struct ListOptions {
    /// Whether to include disabled plugins.
    pub include_disabled: bool,
    /// Whether to show update status column.
    pub show_updates: bool,
    /// Column widths for alignment.
    pub id_col_width: usize,
    pub name_col_width: usize,
    pub version_col_width: usize,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            include_disabled: false,
            show_updates: true,
            id_col_width: 24,
            name_col_width: 28,
            version_col_width: 10,
        }
    }
}

/// Render a header row.
fn render_header(opts: &ListOptions) -> String {
    let id_h = pad("ID", opts.id_col_width);
    let name_h = pad("Name", opts.name_col_width);
    let ver_h = pad("Version", opts.version_col_width);
    let status_h = "Status";

    if opts.show_updates {
        let update_h = "Update";
        format!("{}  {}  {}  {}  {}", id_h, name_h, ver_h, status_h, update_h)
    } else {
        format!("{}  {}  {}  {}", id_h, name_h, ver_h, status_h)
    }
}

/// Render a single entry row.
fn render_entry(entry: &ListEntry, opts: &ListOptions) -> String {
    let id = pad(&entry.meta.id, opts.id_col_width);
    let name = pad(&entry.meta.name, opts.name_col_width);
    let ver = pad(&entry.meta.version, opts.version_col_width);
    let status = if entry.enabled { "enabled" } else { "disabled" };

    if opts.show_updates {
        let update_col = match &entry.update_status {
            None => "-".to_string(),
            Some(UpdateStatus::UpToDate { .. }) => "up-to-date".to_string(),
            Some(UpdateStatus::UpdateAvailable(u)) => {
                format!("update available: {}", u.available)
            }
            Some(UpdateStatus::AheadOfRegistry { .. }) => "ahead of registry".to_string(),
        };
        format!("{}  {}  {}  {}  {}", id, name, ver, status, update_col)
    } else {
        format!("{}  {}  {}  {}", id, name, ver, status)
    }
}

/// Left-pad a string to a minimum width.
fn pad(s: &str, width: usize) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - s.len()))
    }
}

/// Render the full plugin list as a string.
pub fn render_list(entries: &[ListEntry], opts: &ListOptions) -> String {
    let filtered: Vec<&ListEntry> = entries
        .iter()
        .filter(|e| opts.include_disabled || e.enabled)
        .collect();

    if filtered.is_empty() {
        return "No plugins installed.\n".to_string();
    }

    let header = render_header(opts);
    let separator = "-".repeat(header.len());
    let mut lines = vec![header, separator];

    for entry in filtered {
        lines.push(render_entry(entry, opts));
    }

    lines.join("\n") + "\n"
}

/// Build a list of `ListEntry` values from a collection of plugin metas.
pub fn build_entries(metas: impl IntoIterator<Item = PluginMeta>) -> Vec<ListEntry> {
    metas.into_iter().map(ListEntry::new).collect()
}
