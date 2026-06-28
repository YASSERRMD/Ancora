use ancora_orchestrate::{AgentRole, AgentSpec};

use crate::descriptor::{AirGapPolicy, Capability, PresetDescriptor};
use crate::validation::validate;

/// Error produced when a preset cannot be assembled into an `AgentSpec`.
#[derive(Debug)]
pub struct AssemblyError(pub String);

impl std::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "preset assembly error: {}", self.0)
    }
}

/// Assemble a validated preset descriptor into an `AgentSpec`.
///
/// The spec's `system_prompt` encodes capability flags and compliance
/// constraints in a deterministic, round-trippable text form.  The `tools`
/// list mirrors the enabled capabilities so callers can inspect what was
/// activated without parsing the system prompt.
pub fn assemble(preset: &PresetDescriptor) -> Result<AgentSpec, AssemblyError> {
    validate(preset).map_err(|errs| {
        let msg = errs
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        AssemblyError(msg)
    })?;

    let mut prompt_parts = vec![
        format!("preset:{}", preset.name),
        format!("description:{}", preset.description),
    ];

    let tool_names: Vec<String> = preset
        .capabilities
        .iter()
        .map(capability_tool_name)
        .collect();

    for name in &tool_names {
        prompt_parts.push(format!("capability:{name}"));
    }

    if preset.air_gap == AirGapPolicy::Required {
        prompt_parts.push("air_gap:required".to_string());
    }

    match &preset.residency {
        crate::descriptor::ResidencyConstraint::None => {}
        crate::descriptor::ResidencyConstraint::Zone(z) => {
            prompt_parts.push(format!("residency_zone:{z}"));
        }
    }

    if preset.locked {
        prompt_parts.push("locked:true".to_string());
    }

    for (key, val) in &preset.overrides {
        prompt_parts.push(format!("override:{key}={val}"));
    }

    let system_prompt = prompt_parts.join("\n");

    let spec = AgentSpec::new(&preset.name, AgentRole::Orchestrator, &system_prompt)
        .with_tools(tool_names.iter().map(|s| s.as_str()).collect());

    Ok(spec)
}

fn capability_tool_name(cap: &Capability) -> String {
    match cap {
        Capability::Planning => "planning",
        Capability::Reflection => "reflection",
        Capability::Routing => "routing",
        Capability::Memory => "memory",
        Capability::ToolSynthesis => "tool_synthesis",
        Capability::Skills => "skills",
        Capability::LongHorizon => "long_horizon",
        Capability::Coordination => "coordination",
        Capability::Guardrails => "guardrails",
        Capability::Reasoning => "reasoning",
        Capability::CostControl => "cost_control",
        Capability::BehaviorEval => "behavior_eval",
    }
    .to_string()
}
