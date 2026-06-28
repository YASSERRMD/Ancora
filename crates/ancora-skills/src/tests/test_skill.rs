use crate::skill::{SkillDescriptor, SkillScope};

fn make_skill(name: &str, version: u32, tags: Vec<&str>) -> SkillDescriptor {
    SkillDescriptor::new(name, version, "desc", tags, SkillScope::ReadOnly)
}

#[test]
fn skill_has_tag_true() {
    let s = make_skill("search", 1, vec!["retrieval", "read"]);
    assert!(s.has_tag("retrieval"));
}

#[test]
fn skill_missing_tag_false() {
    let s = make_skill("search", 1, vec!["retrieval"]);
    assert!(!s.has_tag("write"));
}

#[test]
fn skill_descriptor_fields_correct() {
    let s = make_skill("summarize", 2, vec!["nlp"]);
    assert_eq!(s.name, "summarize");
    assert_eq!(s.version, 2);
    assert_eq!(s.permission_scope, SkillScope::ReadOnly);
}
