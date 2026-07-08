use ancora_toolsynth::{spec_from_goal, AuditEvent, SynthAudit, SynthCache, SynthRegistry};
use serde_json::json;

fn make_spec() -> ancora_toolsynth::ToolSpec {
    spec_from_goal("search web")
}

#[test]
fn toolsynth_cache_hit_deterministic() {
    let mut cache = SynthCache::default();
    let spec = make_spec();
    cache.insert("search web", spec);

    let hit1 = cache.get("search web").map(|s| s.name.clone());
    let hit2 = cache.get("search web").map(|s| s.name.clone());
    assert_eq!(hit1, hit2);
    assert_eq!(hit1.unwrap(), "search_web");
}

#[test]
fn toolsynth_spec_from_goal_stable() {
    let s1 = spec_from_goal("fetch document");
    let s2 = spec_from_goal("fetch document");
    assert_eq!(s1.name, s2.name);
    assert_eq!(s1.description, s2.description);
}

#[test]
fn toolsynth_audit_replay_stable() {
    let mut audit1 = SynthAudit::default();
    let mut audit2 = SynthAudit::default();

    audit1.record(
        1,
        AuditEvent::Synthesized {
            tool_name: "search_web".into(),
            goal: "search web".into(),
        },
    );
    audit1.record(
        2,
        AuditEvent::Executed {
            tool_name: "search_web".into(),
        },
    );

    audit2.record(
        1,
        AuditEvent::Synthesized {
            tool_name: "search_web".into(),
            goal: "search web".into(),
        },
    );
    audit2.record(
        2,
        AuditEvent::Executed {
            tool_name: "search_web".into(),
        },
    );

    assert_eq!(audit1.entries().len(), audit2.entries().len());
    assert_eq!(
        audit1.events_for_tool("search_web").len(),
        audit2.events_for_tool("search_web").len()
    );
}
