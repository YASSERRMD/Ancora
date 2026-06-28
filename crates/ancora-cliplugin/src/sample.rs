/// A sample CLI plugin implementation that demonstrates the interface.
///
/// This plugin registers a `greet` command that prints a greeting and an
/// `echo` command that echoes its arguments back.

use crate::interface::{
    ArgDef, CliPlugin, CommandSpec, ExecContext, ExecOutput, PluginError, PluginMeta, PluginResult,
};

/// The sample plugin struct.
pub struct SamplePlugin {
    meta: PluginMeta,
}

impl SamplePlugin {
    /// Construct the sample plugin.
    pub fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "ancora.sample",
                "Sample Plugin",
                "1.0.0",
                "A demonstration plugin bundled with ancora-cliplugin",
                "YASSERRMD <arafath.yasser@gmail.com>",
            ),
        }
    }
}

impl Default for SamplePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl CliPlugin for SamplePlugin {
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn commands(&self) -> Vec<CommandSpec> {
        vec![
            CommandSpec::new(
                "greet",
                "Print a friendly greeting",
                "Print a greeting addressed to the given name.\n\nExample:\n  ancora greet --name World",
            )
            .with_alias("hello")
            .with_arg(ArgDef::optional(
                "name",
                Some("World".to_string()),
                "The name to greet",
            )),
            CommandSpec::new(
                "echo",
                "Echo the supplied message",
                "Print the message argument back to stdout.\n\nExample:\n  ancora echo --message 'hi'",
            )
            .with_arg(ArgDef::required("message", "The text to echo")),
        ]
    }

    fn execute(&self, command: &str, ctx: ExecContext) -> PluginResult<ExecOutput> {
        match command {
            "greet" => {
                let name = ctx.get_arg("name").unwrap_or("World");
                let line = format!("Hello, {}!", name);
                Ok(ExecOutput::success(vec![line]))
            }
            "echo" => {
                let message = ctx
                    .get_arg("message")
                    .ok_or_else(|| PluginError::ExecutionFailed("--message is required".into()))?;
                Ok(ExecOutput::success(vec![message.to_string()]))
            }
            other => Err(PluginError::ExecutionFailed(format!(
                "unknown command: {}",
                other
            ))),
        }
    }
}
