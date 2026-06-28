use crate::spec::EffectClass;
use crate::error::SynthError;

/// Scope of permissions granted to a synthesized tool.
#[derive(Debug, Clone)]
pub struct PermissionScope {
    pub allowed_effects: Vec<EffectClass>,
}

impl PermissionScope {
    pub fn read_only() -> Self {
        Self { allowed_effects: vec![EffectClass::ReadOnly] }
    }

    pub fn local_write() -> Self {
        Self { allowed_effects: vec![EffectClass::ReadOnly, EffectClass::WriteLocal] }
    }

    pub fn check(&self, effect: &EffectClass) -> Result<(), SynthError> {
        if self.allowed_effects.contains(effect) {
            Ok(())
        } else {
            Err(SynthError::PermissionDenied(format!("effect {:?} not in scope", effect)))
        }
    }
}
