use ancora_ageval::CoordinationMetric;
use ancora_skills::{SkillDescriptor, SkillJournal, SkillRegistry, SkillScope};

const EPS: f64 = 1e-9;

#[test]
fn skills_parity_coordination_score() {
    assert!((CoordinationMetric::score(4, 4) - 1.0).abs() < EPS);
    assert!((CoordinationMetric::score(4, 3) - 0.75).abs() < EPS);
    assert!((CoordinationMetric::score(4, 0) - 0.0).abs() < EPS);
}

#[test]
fn skills_parity_registry_load_and_find() {
    let mut reg = SkillRegistry::default();
    reg.load(SkillDescriptor::new(
        "search",
        1,
        "search docs",
        vec![],
        SkillScope::ReadOnly,
    ));
    reg.load(SkillDescriptor::new(
        "search",
        2,
        "search docs v2",
        vec![],
        SkillScope::ReadOnly,
    ));
    // find returns highest version
    assert_eq!(reg.find("search").map(|s| s.version), Some(2));
}

#[test]
fn skills_parity_journal_replay_count() {
    let mut j = SkillJournal::default();
    j.record(1, "search", 1, "node-A");
    j.record(2, "search", 1, "node-B");
    j.record(3, "summarize", 1, "node-A");
    assert_eq!(j.replay().len(), 3);
    assert_eq!(j.records_for_skill("search").len(), 2);
}

#[test]
fn skills_parity_jit_loader_load() {
    use ancora_skills::JitLoader;
    let mut reg = SkillRegistry::default();
    let mut jit = JitLoader::new();
    let desc = SkillDescriptor::new(
        "translate",
        1,
        "translate text",
        vec![],
        SkillScope::ReadOnly,
    );
    jit.load_on_demand(&mut reg, desc).unwrap();
    assert!(reg.find("translate").is_some());
}
