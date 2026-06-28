use crate::crew::Crew;
use crate::registry::SkillRegistry;
use crate::skill::{SkillDescriptor, SkillScope};

fn make_skill(name: &str) -> SkillDescriptor {
    SkillDescriptor::new(name, 1, "d", vec![], SkillScope::ReadOnly)
}

#[test]
fn composed_crew_runs() {
    let mut reg = SkillRegistry::default();
    reg.load(make_skill("search"));
    reg.load(make_skill("summarize"));
    let crew = Crew::new("research", vec!["search", "summarize"]);
    let resolved = crew.resolve(&reg).unwrap();
    assert_eq!(resolved.len(), 2);
}

#[test]
fn crew_missing_skill_returns_error() {
    let reg = SkillRegistry::default();
    let crew = Crew::new("broken", vec!["nonexistent"]);
    assert!(crew.resolve(&reg).is_err());
}

#[test]
fn crew_names_are_correct() {
    let mut reg = SkillRegistry::default();
    reg.load(make_skill("step1"));
    reg.load(make_skill("step2"));
    let crew = Crew::new("pipeline", vec!["step1", "step2"]);
    let resolved = crew.resolve(&reg).unwrap();
    assert_eq!(resolved[0].name, "step1");
    assert_eq!(resolved[1].name, "step2");
}
