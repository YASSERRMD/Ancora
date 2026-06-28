/// Plugin help integration - formats help text that incorporates plugin
/// commands alongside built-in CLI commands.

use crate::interface::CommandSpec;

/// A section in the generated help output.
#[derive(Debug, Clone)]
pub struct HelpSection {
    /// Section header (e.g., "Built-in Commands", "Plugin Commands").
    pub title: String,
    /// Lines of formatted text under this section.
    pub lines: Vec<String>,
}

impl HelpSection {
    /// Create a new section.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            lines: Vec::new(),
        }
    }

    /// Add a line to the section.
    pub fn add_line(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }

    /// Render the section to a multi-line string.
    pub fn render(&self) -> String {
        let mut out = format!("{}:\n", self.title);
        for line in &self.lines {
            out.push_str(&format!("  {}\n", line));
        }
        out
    }
}

/// Configuration for help rendering.
#[derive(Debug, Clone)]
pub struct HelpConfig {
    /// Column width used for left-padding command names.
    pub name_col_width: usize,
    /// Whether to include command aliases in the output.
    pub show_aliases: bool,
}

impl Default for HelpConfig {
    fn default() -> Self {
        Self {
            name_col_width: 24,
            show_aliases: true,
        }
    }
}

/// Format a single command spec as a help line.
fn format_command_line(spec: &CommandSpec, config: &HelpConfig) -> String {
    let name_part = if config.show_aliases && !spec.aliases.is_empty() {
        let aliases = spec.aliases.join(", ");
        format!("{} ({})", spec.name, aliases)
    } else {
        spec.name.clone()
    };

    let pad = config.name_col_width.saturating_sub(name_part.len());
    format!("{}{}{}", name_part, " ".repeat(pad + 2), spec.short_help)
}

/// Build a help section from a list of command specs.
pub fn build_plugin_help_section(
    specs: &[CommandSpec],
    title: impl Into<String>,
    config: &HelpConfig,
) -> HelpSection {
    let mut section = HelpSection::new(title);
    for spec in specs {
        section.add_line(format_command_line(spec, config));
    }
    section
}

/// Combine built-in help text with plugin-contributed sections.
pub fn compose_help(
    builtin_help: &str,
    plugin_sections: &[HelpSection],
) -> String {
    let mut output = builtin_help.to_string();
    if !output.ends_with('\n') {
        output.push('\n');
    }
    for section in plugin_sections {
        output.push('\n');
        output.push_str(&section.render());
    }
    output
}

/// Generate the detailed help text for a single plugin command.
pub fn command_detail_help(spec: &CommandSpec) -> String {
    let mut out = format!("Command: {}\n\n{}\n", spec.name, spec.long_help);

    if !spec.args.is_empty() {
        out.push_str("\nArguments:\n");
        for arg in &spec.args {
            let req_marker = if arg.required { " (required)" } else { "" };
            let default_note = arg
                .default
                .as_deref()
                .map(|d| format!(" [default: {}]", d))
                .unwrap_or_default();
            out.push_str(&format!(
                "  --{}{}{}\n      {}\n",
                arg.name, req_marker, default_note, arg.help
            ));
        }
    }

    if !spec.aliases.is_empty() {
        out.push_str(&format!("\nAliases: {}\n", spec.aliases.join(", ")));
    }

    out
}
