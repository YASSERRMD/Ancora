use crate::descriptor::{
    AirGapPolicy, Capability, PresetDescriptor, ResidencyConstraint,
};

/// Research-assistant preset.
///
/// Focuses on memory, reasoning, citations, and long-horizon task tracking.
pub fn research_assistant() -> PresetDescriptor {
    PresetDescriptor::new(
        "research-assistant",
        "Agent optimised for literature review and knowledge synthesis.",
    )
    .with_capability(Capability::Memory)
    .with_capability(Capability::Reasoning)
    .with_capability(Capability::LongHorizon)
    .with_capability(Capability::Planning)
    .with_capability(Capability::Reflection)
    .with_capability(Capability::BehaviorEval)
}

/// Coding-agent preset.
///
/// Enables tool synthesis, skills, planning, and guardrails.
pub fn coding_agent() -> PresetDescriptor {
    PresetDescriptor::new(
        "coding-agent",
        "Agent optimised for writing, reviewing, and executing code.",
    )
    .with_capability(Capability::Planning)
    .with_capability(Capability::ToolSynthesis)
    .with_capability(Capability::Skills)
    .with_capability(Capability::Guardrails)
    .with_capability(Capability::Reflection)
    .with_capability(Capability::CostControl)
}

/// Customer-support preset.
///
/// Enables routing, guardrails, memory, and coordination.
pub fn customer_support() -> PresetDescriptor {
    PresetDescriptor::new(
        "customer-support",
        "Agent optimised for multi-turn support conversations with escalation.",
    )
    .with_capability(Capability::Routing)
    .with_capability(Capability::Guardrails)
    .with_capability(Capability::Memory)
    .with_capability(Capability::Coordination)
    .with_capability(Capability::CostControl)
}

/// Data-analysis preset.
///
/// Enables planning, reasoning, memory, tool synthesis, and behavior evals.
pub fn data_analysis() -> PresetDescriptor {
    PresetDescriptor::new(
        "data-analysis",
        "Agent optimised for structured data exploration and reporting.",
    )
    .with_capability(Capability::Planning)
    .with_capability(Capability::Reasoning)
    .with_capability(Capability::Memory)
    .with_capability(Capability::ToolSynthesis)
    .with_capability(Capability::BehaviorEval)
    .with_capability(Capability::CostControl)
}

/// Government-compliant preset.
///
/// Air-gapped, residency-constrained, locked preset for sovereign deployments.
/// Routing is intentionally excluded because it implies remote model dispatch.
pub fn government_compliant(zone: impl Into<String>) -> PresetDescriptor {
    PresetDescriptor::new(
        "government-compliant",
        "Air-gapped, residency-constrained preset for government and classified environments.",
    )
    .with_capability(Capability::Planning)
    .with_capability(Capability::Memory)
    .with_capability(Capability::Guardrails)
    .with_capability(Capability::Reasoning)
    .with_capability(Capability::LongHorizon)
    .with_capability(Capability::BehaviorEval)
    .with_air_gap(AirGapPolicy::Required)
    .with_residency(ResidencyConstraint::Zone(zone.into()))
    .with_locked(true)
}
