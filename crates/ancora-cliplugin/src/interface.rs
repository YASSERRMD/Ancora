/// Describes the interface that every CLI plugin must implement.

/// The result type used throughout the plugin interface.
pub type PluginResult<T> = Result<T, PluginError>;

/// Errors that a plugin can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginError {
    /// The command name conflicts with an already-registered command.
    ConflictingCommand(String),
    /// The plugin does not have permission to perform the requested action.
    PermissionDenied(String),
    /// The plugin configuration is invalid.
    InvalidConfig(String),
    /// A generic execution failure with a human-readable description.
    ExecutionFailed(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::ConflictingCommand(name) => {
                write!(f, "conflicting command: {}", name)
            }
            PluginError::PermissionDenied(action) => {
                write!(f, "permission denied: {}", action)
            }
            PluginError::InvalidConfig(msg) => {
                write!(f, "invalid config: {}", msg)
            }
            PluginError::ExecutionFailed(msg) => {
                write!(f, "execution failed: {}", msg)
            }
        }
    }
}

/// Metadata that every plugin must provide.
#[derive(Debug, Clone)]
pub struct PluginMeta {
    /// Unique identifier for the plugin.
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Semantic version string, e.g. "1.0.0".
    pub version: String,
    /// Brief description shown in help output.
    pub description: String,
    /// The author(s) of the plugin.
    pub author: String,
}

impl PluginMeta {
    /// Construct a new [`PluginMeta`].
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            description: description.into(),
            author: author.into(),
        }
    }
}

/// A single argument definition for a CLI command.
#[derive(Debug, Clone)]
pub struct ArgDef {
    /// The argument name (used as `--<name>` on the command line).
    pub name: String,
    /// Whether the argument is required.
    pub required: bool,
    /// Default value when the argument is optional and not provided.
    pub default: Option<String>,
    /// Short description shown in help.
    pub help: String,
}

impl ArgDef {
    /// Create a required argument.
    pub fn required(name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
            default: None,
            help: help.into(),
        }
    }

    /// Create an optional argument with an optional default.
    pub fn optional(
        name: impl Into<String>,
        default: Option<String>,
        help: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            required: false,
            default,
            help: help.into(),
        }
    }
}

/// Describes a single command that a plugin registers.
#[derive(Debug, Clone)]
pub struct CommandSpec {
    /// The primary command name.
    pub name: String,
    /// Optional aliases for the command.
    pub aliases: Vec<String>,
    /// Short description shown in the parent help.
    pub short_help: String,
    /// Full help text shown when the user requests `--help` for this command.
    pub long_help: String,
    /// Argument definitions for this command.
    pub args: Vec<ArgDef>,
}

impl CommandSpec {
    /// Create a minimal [`CommandSpec`] with no aliases and no args.
    pub fn new(
        name: impl Into<String>,
        short_help: impl Into<String>,
        long_help: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            short_help: short_help.into(),
            long_help: long_help.into(),
            args: Vec::new(),
        }
    }

    /// Builder method to add an alias.
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Builder method to add an argument definition.
    pub fn with_arg(mut self, arg: ArgDef) -> Self {
        self.args.push(arg);
        self
    }
}

/// The execution context passed to a plugin command handler.
#[derive(Debug, Clone)]
pub struct ExecContext {
    /// Resolved argument values keyed by argument name.
    pub args: std::collections::HashMap<String, String>,
    /// Whether the CLI is running in verbose mode.
    pub verbose: bool,
}

impl ExecContext {
    /// Construct a new execution context.
    pub fn new(args: std::collections::HashMap<String, String>, verbose: bool) -> Self {
        Self { args, verbose }
    }

    /// Retrieve an argument value by name.
    pub fn get_arg(&self, name: &str) -> Option<&str> {
        self.args.get(name).map(|s| s.as_str())
    }
}

/// The output produced by a plugin command execution.
#[derive(Debug, Clone)]
pub struct ExecOutput {
    /// Human-readable output lines.
    pub lines: Vec<String>,
    /// Exit code (0 for success).
    pub exit_code: i32,
}

impl ExecOutput {
    /// Construct a successful output.
    pub fn success(lines: Vec<String>) -> Self {
        Self { lines, exit_code: 0 }
    }

    /// Construct a failure output.
    pub fn failure(lines: Vec<String>, exit_code: i32) -> Self {
        Self { lines, exit_code }
    }
}

/// The core trait that every CLI plugin must implement.
pub trait CliPlugin: Send + Sync {
    /// Return the plugin's metadata.
    fn meta(&self) -> &PluginMeta;

    /// Return the list of commands this plugin registers.
    fn commands(&self) -> Vec<CommandSpec>;

    /// Execute the named command with the given context.
    fn execute(&self, command: &str, ctx: ExecContext) -> PluginResult<ExecOutput>;
}
