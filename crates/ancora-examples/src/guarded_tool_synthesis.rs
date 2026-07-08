use ancora_toolsynth::{
    spec_from_goal, ApprovalGate, AuditEvent, PermissionScope, SandboxRunner, SchemaValidator,
    SynthAudit, SynthCache, SynthRegistry,
};

pub fn run_guarded_tool_synthesis_example() {
    let goal = "List recent files";
    let spec = spec_from_goal(goal);

    SchemaValidator::validate(&spec.input_schema).expect("schema must be valid");

    let scope = PermissionScope::read_only();
    scope
        .check(&spec.effect_class)
        .expect("effect must be in scope");

    let result = SandboxRunner::execute(&spec, &serde_json::json!({}))
        .expect("sandbox execution must succeed");
    assert_eq!(result["status"], "ok");

    let mut gate = ApprovalGate::default();
    gate.approve(&spec.name);
    gate.check(&spec.name)
        .expect("tool must be approved before use");

    let mut registry = SynthRegistry::default();
    registry.register(spec.clone());

    let mut cache = SynthCache::default();
    cache.insert(goal, spec.clone());

    let mut audit = SynthAudit::default();
    audit.record(
        1,
        AuditEvent::Synthesized {
            tool_name: spec.name.clone(),
            goal: goal.into(),
        },
    );
    audit.record(
        2,
        AuditEvent::Approved {
            tool_name: spec.name.clone(),
            approver: "operator".into(),
        },
    );
    audit.record(
        3,
        AuditEvent::Cached {
            tool_name: spec.name.clone(),
        },
    );

    assert_eq!(audit.events_for_tool(&spec.name).len(), 3);
    assert!(cache.get(goal).is_some());
    assert!(registry.get(&spec.name).is_some());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_tool_synthesis_example_runs() {
        run_guarded_tool_synthesis_example();
    }
}
