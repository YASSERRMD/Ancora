/// A capability flag that a preset can enable or require.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capability {
    Planning,
    Reflection,
    Routing,
    Memory,
    ToolSynthesis,
    Skills,
    LongHorizon,
    Coordination,
    Guardrails,
    Reasoning,
    CostControl,
    BehaviorEval,
}

/// Air-gap policy: whether the preset must operate without network access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirGapPolicy {
    /// No constraint -- network calls allowed.
    None,
    /// Preset MUST run fully in-process with no external calls.
    Required,
}

/// Data-residency constraint for government/compliance presets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidencyConstraint {
    None,
    /// All data must remain within the specified zone identifier.
    Zone(String),
}

/// Descriptor for a capability preset.
///
/// Presets bundle a fixed set of capabilities with optional overrides and
/// compliance constraints.  They are validated before use -- an invalid
/// descriptor (empty name, conflicting flags, etc.) is rejected by
/// `validate()`.
#[derive(Debug, Clone)]
pub struct PresetDescriptor {
    /// Machine-readable identifier, e.g. `"research-assistant"`.
    pub name: String,
    /// Human-readable summary.
    pub description: String,
    /// Capabilities this preset activates.
    pub capabilities: Vec<Capability>,
    /// If `true`, the preset forbids capabilities not listed in `capabilities`.
    pub locked: bool,
    /// Air-gap policy: `Required` means no network calls are permitted.
    pub air_gap: AirGapPolicy,
    /// Optional data-residency constraint (relevant for government presets).
    pub residency: ResidencyConstraint,
    /// User-supplied key-value overrides applied on top of preset defaults.
    pub overrides: Vec<(String, String)>,
}

impl PresetDescriptor {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            capabilities: Vec::new(),
            locked: false,
            air_gap: AirGapPolicy::None,
            residency: ResidencyConstraint::None,
            overrides: Vec::new(),
        }
    }

    pub fn with_capability(mut self, cap: Capability) -> Self {
        if !self.capabilities.contains(&cap) {
            self.capabilities.push(cap);
        }
        self
    }

    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    pub fn with_air_gap(mut self, policy: AirGapPolicy) -> Self {
        self.air_gap = policy;
        self
    }

    pub fn with_residency(mut self, r: ResidencyConstraint) -> Self {
        self.residency = r;
        self
    }

    /// Apply a key-value override pair.
    pub fn with_override(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.overrides.push((key.into(), value.into()));
        self
    }
}
