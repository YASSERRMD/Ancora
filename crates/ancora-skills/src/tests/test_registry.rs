use crate::registry::SkillRegistry;
use crate::skill::{SkillDescriptor, SkillScope};

fn make_skill(name: &str, version: u32, tags: Vec<&str>) -> SkillDescriptor {
    SkillDescriptor::new(name, version, "d", tags, SkillScope::ReadOnly)
}

#[test]
fn registry_loads_and_finds_skill() {
    let mut reg = SkillRegistry::default();
    reg.load(make_skill("search", 1, vec!["retrieval"]));
    assert!(reg.find("search").is_some());
}

#[test]
fn versioned_skill_resolves() {
    let mut reg = SkillRegistry::default();
    reg.load(make_skill("search", 1, vec![]));
    reg.load(make_skill("search", 2, vec![]));
    let s = reg.find_version("search", 1).unwrap();
    assert_eq!(s.version, 1);
}

#[test]
fn find_returns_latest_version() {
    let mut reg = SkillRegistry::default();
    reg.load(make_skill("search", 1, vec![]));
    reg.load(make_skill("search", 3, vec![]));
    let s = reg.find("search").unwrap();
    assert_eq!(s.version, 3);
}

#[test]
fn discovery_by_capability_tag() {
    let mut reg = SkillRegistry::default();
    reg.load(make_skill("search", 1, vec!["retrieval"]));
    reg.load(make_skill("write", 1, vec!["write"]));
    let found = reg.by_tag("retrieval");
    assert_eq!(found.len(), 1);
}

#[test]
fn lookup_missing_returns_error() {
    let reg = SkillRegistry::default();
    assert!(reg.lookup("missing").is_err());
}
