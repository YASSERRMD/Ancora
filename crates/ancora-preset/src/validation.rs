use crate::descriptor::{AirGapPolicy, PresetDescriptor};

/// Errors produced when a `PresetDescriptor` fails validation.
#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyName,
    EmptyDescription,
    NoCapabilities,
    /// Air-gap is required but Routing capability (which implies remote routing)
    /// is also listed -- conflicting combination.
    AirGapConflictsWithRouting,
    /// An override key is empty.
    EmptyOverrideKey,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "preset name must not be empty"),
            Self::EmptyDescription => write!(f, "preset description must not be empty"),
            Self::NoCapabilities => write!(f, "preset must list at least one capability"),
            Self::AirGapConflictsWithRouting => {
                write!(
                    f,
                    "air-gap:required is incompatible with the Routing capability"
                )
            }
            Self::EmptyOverrideKey => write!(f, "override key must not be empty"),
        }
    }
}

/// Validate a `PresetDescriptor`, returning all accumulated errors.
///
/// Returns `Ok(())` when no errors are found.
pub fn validate(preset: &PresetDescriptor) -> Result<(), Vec<ValidationError>> {
    use crate::descriptor::Capability;
    let mut errors = Vec::new();

    if preset.name.trim().is_empty() {
        errors.push(ValidationError::EmptyName);
    }
    if preset.description.trim().is_empty() {
        errors.push(ValidationError::EmptyDescription);
    }
    if preset.capabilities.is_empty() {
        errors.push(ValidationError::NoCapabilities);
    }
    if preset.air_gap == AirGapPolicy::Required
        && preset.capabilities.contains(&Capability::Routing)
    {
        errors.push(ValidationError::AirGapConflictsWithRouting);
    }
    for (key, _) in &preset.overrides {
        if key.trim().is_empty() {
            errors.push(ValidationError::EmptyOverrideKey);
            break;
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
