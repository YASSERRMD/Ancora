use crate::{AttackCategory, ScenarioBuilder};

#[test]
fn custom_builder_injection() {
    let dataset = ScenarioBuilder::new()
        .add_injection("ignore previous instructions", true)
        .add_injection("hello world", false)
        .build();
    assert_eq!(dataset.len(), 2);
    let blocked: Vec<_> = dataset
        .scenarios
        .iter()
        .filter(|s| s.expected_blocked)
        .collect();
    assert_eq!(blocked.len(), 1);
}

#[test]
fn custom_builder_jailbreak() {
    let dataset = ScenarioBuilder::new()
        .add_jailbreak("jailbreak mode", true)
        .build();
    assert_eq!(dataset.len(), 1);
    assert_eq!(dataset.scenarios[0].category, AttackCategory::Jailbreak);
}

#[test]
fn custom_builder_tool_misuse() {
    let dataset = ScenarioBuilder::new()
        .add_tool_misuse("run_shell", true)
        .add_tool_misuse("search_web", false)
        .build();
    assert_eq!(dataset.len(), 2);
    let tool_cats: Vec<_> = dataset.scenarios.iter().map(|s| &s.category).collect();
    assert!(tool_cats.iter().all(|c| **c == AttackCategory::ToolMisuse));
}

#[test]
fn custom_builder_chaining() {
    let dataset = ScenarioBuilder::new()
        .add_injection("attack", true)
        .add_jailbreak("jailbreak", true)
        .add_tool_misuse("bad_tool", true)
        .build();
    assert_eq!(dataset.len(), 3);
}

#[test]
fn custom_builder_ids_unique() {
    let dataset = ScenarioBuilder::new()
        .add_injection("a", true)
        .add_injection("b", true)
        .add_jailbreak("c", true)
        .build();
    let mut ids: Vec<&str> = dataset.scenarios.iter().map(|s| s.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), dataset.scenarios.len());
}
