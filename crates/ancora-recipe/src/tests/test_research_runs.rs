use crate::params::ParamSet;
use crate::research_report::{build, generate_outline};

#[test]
fn research_recipe_builds_and_validates() {
    let mut ps = ParamSet::new();
    ps.set("topic", "climate change");
    ps.set("sections", "4");
    let r = build(&ps);
    assert!(r.validate().is_ok());
    assert_eq!(r.id, "research-report");
}

#[test]
fn research_recipe_has_four_steps() {
    let ps = ParamSet::default();
    let r = build(&ps);
    assert_eq!(r.step_count(), 4);
}

#[test]
fn outline_contains_correct_section_count() {
    let outline = generate_outline("Quantum Computing", 5);
    assert_eq!(outline.section_count(), 5);
    assert!(outline.title.contains("Quantum Computing"));
}

#[test]
fn sections_are_uniquely_named() {
    let outline = generate_outline("Topic", 3);
    let names: std::collections::HashSet<_> = outline.sections.iter().collect();
    assert_eq!(names.len(), 3);
}
