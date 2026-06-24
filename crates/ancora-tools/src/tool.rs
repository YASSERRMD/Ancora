use crate::error::ToolError;

/// The side-effect class of a tool call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolEffect {
    /// No side-effects; safe to call freely.
    ReadOnly,
    /// Writes data or calls external services.
    Write,
}

/// A typed local function tool with a JSON Schema contract.
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> serde_json::Value;
    fn effect(&self) -> ToolEffect;
    fn call(&self, input: &serde_json::Value) -> Result<serde_json::Value, ToolError>;
}
