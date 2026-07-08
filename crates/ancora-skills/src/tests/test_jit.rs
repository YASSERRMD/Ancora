use crate::jit::JitLoader;
use crate::registry::SkillRegistry;
use crate::skill::{SkillDescriptor, SkillScope};

fn make_skill(name: &str) -> SkillDescriptor {
    SkillDescriptor::new(name, 1, "d", vec![], SkillScope::ReadOnly)
}

#[test]
fn jit_loading_bounds_context() {
    let mut loader = JitLoader::new();
    let mut registry = SkillRegistry::default();
    loader
        .load_on_demand(&mut registry, make_skill("lazy_skill"))
        .unwrap();
    assert!(loader.is_loaded("lazy_skill"));
    assert!(registry.find("lazy_skill").is_some());
}

#[test]
fn jit_duplicate_load_is_idempotent() {
    let mut loader = JitLoader::new();
    let mut registry = SkillRegistry::default();
    loader
        .load_on_demand(&mut registry, make_skill("once"))
        .unwrap();
    loader
        .load_on_demand(&mut registry, make_skill("once"))
        .unwrap();
    assert_eq!(loader.loaded_count(), 1);
}

#[test]
fn jit_not_loaded_before_demand() {
    let loader = JitLoader::new();
    assert!(!loader.is_loaded("not_yet"));
}
