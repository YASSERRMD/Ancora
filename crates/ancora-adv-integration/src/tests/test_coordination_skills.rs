use ancora_coord::Blackboard;
use ancora_skills::{JitLoader, SkillDescriptor, SkillRegistry, SkillScope};
use ancora_ageval::CoordinationMetric;

#[test]
fn coordination_plus_skills_registry() {
    let mut registry = SkillRegistry::default();
    let skill = SkillDescriptor {
        name: "search".into(),
        version: 1,
        description: "web search".into(),
        capability_tags: vec!["search".into()],
        input_schema: "{}".into(),
        permission_scope: SkillScope::ReadOnly,
    };
    registry.load(skill);

    let found = registry.find("search").unwrap();
    let mut board = Blackboard::default();
    board.claim_role("coordinator", "selected_skill");
    board.write("coordinator", "selected_skill", &found.name).unwrap();
    assert_eq!(board.read("selected_skill").unwrap(), "search");

    assert_eq!(CoordinationMetric::score(3, 3), 1.0);
}

#[test]
fn jit_loader_integrates_with_skills() {
    let mut registry = SkillRegistry::default();
    let mut loader = JitLoader::new();

    let s1 = SkillDescriptor {
        name: "summarize".into(),
        version: 1,
        description: "summarize text".into(),
        capability_tags: vec![],
        input_schema: "{}".into(),
        permission_scope: SkillScope::ReadOnly,
    };
    let s2 = SkillDescriptor {
        name: "search".into(),
        version: 1,
        description: "search".into(),
        capability_tags: vec![],
        input_schema: "{}".into(),
        permission_scope: SkillScope::ReadOnly,
    };
    let s2_dup = SkillDescriptor {
        name: "search".into(),
        version: 1,
        description: "search".into(),
        capability_tags: vec![],
        input_schema: "{}".into(),
        permission_scope: SkillScope::ReadOnly,
    };

    loader.load_on_demand(&mut registry, s1).unwrap();
    loader.load_on_demand(&mut registry, s2).unwrap();
    loader.load_on_demand(&mut registry, s2_dup).unwrap(); // idempotent

    assert_eq!(loader.loaded_count(), 2);
    assert!(loader.is_loaded("summarize"));
    assert!(loader.is_loaded("search"));
}
