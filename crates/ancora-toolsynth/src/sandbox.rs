use crate::error::SynthError;
use crate::spec::{EffectClass, ToolSpec};

/// Simulates running a synthesized tool in a sandboxed environment.
/// Only ReadOnly and WriteLocal effects are permitted.
pub struct SandboxRunner;

impl SandboxRunner {
    pub fn execute(
        spec: &ToolSpec,
        _input: &serde_json::Value,
    ) -> Result<serde_json::Value, SynthError> {
        match spec.effect_class {
            EffectClass::ReadOnly | EffectClass::WriteLocal => {
                Ok(serde_json::json!({ "status": "ok", "tool": spec.name }))
            }
            EffectClass::WriteExternal | EffectClass::Destructive => {
                Err(SynthError::SandboxViolation(format!(
                    "effect {:?} not allowed in sandbox",
                    spec.effect_class
                )))
            }
        }
    }
}
